use crate::types::{
    AssemblySyntax, ISABase, ISAExtension, Instruction, Operand, OperandRestriction, OperandType,
};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;

pub fn generate_detailed_extension_report(
    instructions: &[Instruction],
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut report = String::new();
    report.push_str("# RISC-V 指令按扩展分类详细报告\n\n");

    // 获取当前时间（简化版本）
    report.push_str(&format!(
        "**报告生成时间**: {}\n\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    ));

    // 收集所有指令并按扩展分组
    let mut extension_groups: BTreeMap<ISAExtension, Vec<&Instruction>> = BTreeMap::new();

    for inst in instructions.iter() {
        extension_groups
            .entry(inst.extension)
            .or_insert_with(Vec::new)
            .push(inst);
    }

    // 生成概览统计表
    report.push_str("## 📊 概览统计\n\n");
    report.push_str("| 扩展 | 标准指令数量 | 压缩指令数量 | 总计 | 描述 |\n");
    report.push_str("|------|-------------|-------------|------|------|\n");

    for (extension, instructions_in_ext) in &extension_groups {
        let standard_count = instructions_in_ext
            .iter()
            .filter(|inst| !inst.name.starts_with("c."))
            .count();
        let compressed_count = instructions_in_ext
            .iter()
            .filter(|inst| inst.name.starts_with("c."))
            .count();
        let total_count = instructions_in_ext.len();
        let description = get_extension_description(*extension);

        report.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            extension, standard_count, compressed_count, total_count, description
        ));
    }

    report.push_str("\n");

    // 为每个扩展生成详细表格
    for (extension, instructions_in_ext) in &extension_groups {
        report.push_str(&format!("## 🔧 {} 扩展指令\n\n", extension));
        report.push_str(&format!(
            "**扩展描述**: {}\n\n",
            get_extension_description(*extension)
        ));
        report.push_str(&format!(
            "**指令总数**: {} 条\n\n",
            instructions_in_ext.len()
        ));

        // 按指令类型分组
        let mut standard_instructions: Vec<&Instruction> = Vec::new();
        let mut compressed_instructions: Vec<&Instruction> = Vec::new();

        for inst in instructions_in_ext {
            if inst.name.starts_with("c.") {
                compressed_instructions.push(inst);
            } else {
                standard_instructions.push(inst);
            }
        }

        // 标准指令表格
        if !standard_instructions.is_empty() {
            report.push_str("### 📝 标准指令\n\n");
            generate_instruction_table(&mut report, &standard_instructions);
            report.push_str("\n");
        }

        // 压缩指令表格
        if !compressed_instructions.is_empty() {
            report.push_str("### 📦 压缩指令\n\n");
            generate_instruction_table(&mut report, &compressed_instructions);
            report.push_str("\n");
        }

        report.push_str("---\n\n");
    }

    // 生成ISA基础架构兼容性报告
    report.push_str("## 🏗️ ISA基础架构兼容性\n\n");
    generate_isa_compatibility_report(&mut report, instructions);

    // 生成操作数分析报告
    report.push_str("## 📋 操作数使用统计\n\n");
    generate_operand_usage_report(&mut report, instructions);

    // 生成按指令长度分组的统计
    report.push_str("## 📏 按操作数数量分组统计\n\n");
    generate_operand_count_statistics(&mut report, instructions);

    fs::write(output_path, report)?;

    Ok(())
}

fn generate_instruction_table(report: &mut String, instructions: &[&Instruction]) {
    report.push_str("| 指令名称 | ISA支持 | 操作数数量 | 汇编语法 | 操作数名称 | 操作数长度(RV32/RV64) | 操作数限制 |\n");
    report.push_str("|----------|---------|-----------|----------|------------|-------------------|----------|\n");

    let mut sorted_instructions = instructions.to_vec();
    sorted_instructions.sort_by(|a, b| a.name.cmp(&b.name));

    for inst in sorted_instructions {
        let isa_support = format_isa_bases(&inst.isa_bases);
        let operand_count = inst.operands.len();
        let assembly_syntax = format_assembly_syntax(&inst.assembly_syntax);
        let operand_names = format_operand_names(&inst.operands);
        let operand_lengths = format_operand_lengths(&inst.operands);
        let operand_restrictions = format_operand_restrictions(&inst.operands);

        report.push_str(&format!(
            "| `{}` | {} | {} | `{}` | {} | {} | {} |\n",
            inst.name,
            isa_support,
            operand_count,
            escape_markdown(&assembly_syntax),
            operand_names,
            operand_lengths,
            operand_restrictions
        ));
    }
}

fn generate_isa_compatibility_report(report: &mut String, instructions: &[Instruction]) {
    let mut rv32_only = 0;
    let mut rv64_only = 0;
    let mut both = 0;
    let mut rv32_extensions: BTreeSet<ISAExtension> = BTreeSet::new();
    let mut rv64_extensions: BTreeSet<ISAExtension> = BTreeSet::new();
    let mut both_extensions: BTreeSet<ISAExtension> = BTreeSet::new();

    for inst in instructions.iter() {
        let has_rv32 = inst.isa_bases.contains(&ISABase::RV32);
        let has_rv64 = inst.isa_bases.contains(&ISABase::RV64);

        match (has_rv32, has_rv64) {
            (true, true) => {
                both += 1;
                both_extensions.insert(inst.extension);
            }
            (true, false) => {
                rv32_only += 1;
                rv32_extensions.insert(inst.extension);
            }
            (false, true) => {
                rv64_only += 1;
                rv64_extensions.insert(inst.extension);
            }
            (false, false) => {} // 不应该发生
        }
    }

    report.push_str("| ISA基础 | 指令数量 | 相关扩展 |\n");
    report.push_str("|---------|----------|----------|\n");
    report.push_str(&format!(
        "| 仅RV32 | {} | {} |\n",
        rv32_only,
        format_extension_list(&rv32_extensions)
    ));
    report.push_str(&format!(
        "| 仅RV64 | {} | {} |\n",
        rv64_only,
        format_extension_list(&rv64_extensions)
    ));
    report.push_str(&format!(
        "| RV32和RV64 | {} | {} |\n",
        both,
        format_extension_list(&both_extensions)
    ));

    report.push_str("\n");
}

fn generate_operand_usage_report(report: &mut String, instructions: &[Instruction]) {
    let mut operand_counts: HashMap<String, usize> = HashMap::new();
    let mut operand_in_extensions: HashMap<String, BTreeSet<ISAExtension>> = HashMap::new();
    let mut operand_lengths: HashMap<String, HashMap<ISABase, BTreeSet<u8>>> = HashMap::new();
    let mut operand_restrictions_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut operand_types: HashMap<String, BTreeSet<OperandType>> = HashMap::new();

    for inst in instructions.iter() {
        for operand in &inst.operands {
            *operand_counts.entry(operand.name.clone()).or_insert(0) += 1;
            operand_in_extensions
                .entry(operand.name.clone())
                .or_insert_with(BTreeSet::new)
                .insert(inst.extension);

            // 收集操作数类型信息
            if let Some(operand_type) = &operand.operand_type {
                operand_types
                    .entry(operand.name.clone())
                    .or_insert_with(BTreeSet::new)
                    .insert(operand_type.clone());
            }

            // 收集操作数长度信息
            let lengths = operand_lengths
                .entry(operand.name.clone())
                .or_insert_with(HashMap::new);
            for (base, length) in &operand.bit_lengths {
                lengths
                    .entry(*base)
                    .or_insert_with(BTreeSet::new)
                    .insert(*length);
            }

            // 收集操作数限制信息
            if let Some(restrictions) = &operand.restrictions {
                let restriction_desc = format_single_operand_restrictions(restrictions);
                if !restriction_desc.is_empty() {
                    operand_restrictions_map
                        .entry(operand.name.clone())
                        .or_insert_with(Vec::new)
                        .push(restriction_desc);
                }
            }
        }
    }

    let mut operand_vec: Vec<_> = operand_counts.iter().collect();
    operand_vec.sort_by_key(|(_, count)| std::cmp::Reverse(**count));

    report.push_str("### 🏷️ 操作数详细统计\n\n");
    report.push_str(
        "| 操作数名称 | 使用次数 | 操作数类型 | 出现在扩展 | 长度分布(RV32/RV64) | 限制条件 |\n",
    );
    report.push_str(
        "|------------|----------|------------|------------|-------------------|----------|\n",
    );

    for (operand_name, count) in operand_vec.iter() {
        let operand_type_str = operand_types
            .get(*operand_name)
            .map(|types| {
                if types.len() == 1 {
                    format!("{}", types.iter().next().unwrap())
                } else {
                    types
                        .iter()
                        .map(|t| format!("{}", t))
                        .collect::<Vec<_>>()
                        .join(", ")
                }
            })
            .unwrap_or_else(|| "未知".to_string());

        let extensions = operand_in_extensions
            .get(*operand_name)
            .map(|exts| format_extension_list(exts))
            .unwrap_or_default();

        let lengths = operand_lengths
            .get(*operand_name)
            .map(|length_map| format_operand_length_distribution(length_map))
            .unwrap_or_else(|| "未知".to_string());

        let restrictions = operand_restrictions_map
            .get(*operand_name)
            .map(|restr_vec| {
                let unique_restrictions: BTreeSet<_> = restr_vec.iter().collect();
                if unique_restrictions.is_empty() {
                    "无".to_string()
                } else {
                    unique_restrictions
                        .into_iter()
                        .cloned()
                        .collect::<Vec<_>>()
                        .join("; ")
                }
            })
            .unwrap_or_else(|| "无".to_string());

        report.push_str(&format!(
            "| `{}` | {} | {} | {} | {} | {} |\n",
            operand_name, count, operand_type_str, extensions, lengths, restrictions
        ));
    }

    report.push_str("\n");

    // 添加操作数长度统计报告
    generate_operand_length_statistics(report, instructions);

    // 添加操作数限制统计报告
    generate_operand_restriction_statistics(report, instructions);
}

fn generate_operand_count_statistics(report: &mut String, instructions: &[Instruction]) {
    let mut operand_count_stats: HashMap<usize, Vec<String>> = HashMap::new();

    for inst in instructions.iter() {
        let count = inst.operands.len();
        operand_count_stats
            .entry(count)
            .or_insert_with(Vec::new)
            .push(inst.name.clone());
    }

    let mut count_vec: Vec<_> = operand_count_stats.iter().collect();
    count_vec.sort_by_key(|(count, _)| **count);

    report.push_str("| 操作数数量 | 指令数量 | 示例指令 |\n");
    report.push_str("|-----------|----------|----------|\n");

    for (count, inst_names) in count_vec {
        let examples = if inst_names.len() <= 5 {
            inst_names
                .iter()
                .map(|s| format!("`{}`", s))
                .collect::<Vec<_>>()
                .join(", ")
        } else {
            let first_five: Vec<String> = inst_names
                .iter()
                .take(5)
                .map(|s| format!("`{}`", s))
                .collect();
            format!("{}, ... (共{}条)", first_five.join(", "), inst_names.len())
        };

        report.push_str(&format!(
            "| {} | {} | {} |\n",
            count,
            inst_names.len(),
            examples
        ));
    }

    report.push_str("\n");
}

fn get_extension_description(extension: ISAExtension) -> &'static str {
    match extension {
        ISAExtension::I => "基本整数指令集",
        ISAExtension::M => "乘法除法扩展",
        ISAExtension::F => "单精度浮点扩展",
        ISAExtension::D => "双精度浮点扩展",
        ISAExtension::Q => "四精度浮点扩展",
        ISAExtension::C => "压缩指令扩展",
        ISAExtension::V => "向量扩展",
        ISAExtension::B => "位操作扩展",
        ISAExtension::H => "虚拟化扩展",
        ISAExtension::S => "特权架构扩展",
        ISAExtension::Zifencei => "指令同步扩展",
        ISAExtension::Zicsr => "控制状态寄存器扩展",
        ISAExtension::Zaamo => "原子内存操作扩展",
        ISAExtension::Zabha => "字节和半字原子操作扩展",
        ISAExtension::Zacas => "比较交换原子操作扩展",
        ISAExtension::Zalasr => "加载保留/存储条件扩展",
        ISAExtension::Zalrsc => "LR/SC原子操作扩展",
        ISAExtension::Zawrs => "等待保留集扩展",
        ISAExtension::Zba => "地址生成位操作扩展",
        ISAExtension::Zbb => "基本位操作扩展",
        ISAExtension::Zbc => "进位位操作扩展",
        ISAExtension::Zbkb => "位操作加密扩展(基本)",
        ISAExtension::Zbkx => "位操作加密扩展(交叉)",
        ISAExtension::Zbs => "单位位操作扩展",
        ISAExtension::Zcb => "压缩基本扩展",
        ISAExtension::Zcd => "压缩双精度浮点扩展",
        ISAExtension::Zcf => "压缩单精度浮点扩展",
        ISAExtension::Zcmop => "压缩可能操作扩展",
        ISAExtension::Zcmp => "压缩指针操作扩展",
        ISAExtension::Zfbfmin => "标量BF16转换扩展",
        ISAExtension::Zfh => "半精度浮点扩展",
        ISAExtension::Zicbom => "缓存块管理扩展",
        ISAExtension::Zicboz => "缓存块清零扩展",
        ISAExtension::Zicfilp => "控制流完整性扩展",
        ISAExtension::Zicfiss => "影子栈扩展",
        ISAExtension::Zicond => "条件操作扩展",
        ISAExtension::Zilsd => "负载存储成对扩展",
        ISAExtension::Zimop => "可能操作扩展",
        ISAExtension::Zkn => "加密NIST算法扩展",
        ISAExtension::Zknd => "NIST AES解密扩展",
        ISAExtension::Zkne => "NIST AES加密扩展",
        ISAExtension::Zknh => "NIST SHA哈希扩展",
        ISAExtension::Zks => "加密ShangMi算法扩展",
        ISAExtension::Zvbb => "向量基本位操作扩展",
        ISAExtension::Zvbc => "向量进位位操作扩展",
        ISAExtension::Zvfbfmin => "向量BF16转换扩展",
        ISAExtension::Zvfbfwma => "向量BF16乘加扩展",
        ISAExtension::Zvkg => "向量GCM/GMAC扩展",
        ISAExtension::Zvkned => "向量NIST AES扩展",
        ISAExtension::Zvknha => "向量NIST SHA-2扩展",
        ISAExtension::Zvks => "向量ShangMi扩展",
        ISAExtension::Sdext => "调试扩展",
        ISAExtension::Smdbltrp => "M模式双陷阱扩展",
        ISAExtension::Smrnmi => "M模式可恢复非屏蔽中断扩展",
        ISAExtension::Svinval => "细粒度地址转换缓存无效化扩展",
    }
}

fn format_isa_bases(bases: &[ISABase]) -> String {
    if bases.len() == 2 {
        "RV32/64".to_string()
    } else if bases.contains(&ISABase::RV32) {
        "RV32".to_string()
    } else if bases.contains(&ISABase::RV64) {
        "RV64".to_string()
    } else {
        "未知".to_string()
    }
}

fn format_assembly_syntax(syntax: &AssemblySyntax) -> String {
    match syntax {
        AssemblySyntax::Format(format) => format.clone(),
        AssemblySyntax::RustCode(_) => {
            format!("Rust代码: 略")
        }
    }
}

fn format_extension_list(extensions: &BTreeSet<ISAExtension>) -> String {
    if extensions.len() <= 3 {
        extensions
            .iter()
            .map(|e| format!("{}", e))
            .collect::<Vec<_>>()
            .join(", ")
    } else {
        let first_three: Vec<String> = extensions
            .iter()
            .take(3)
            .map(|e| format!("{}", e))
            .collect();
        format!("{}, ... (共{}个)", first_three.join(", "), extensions.len())
    }
}

fn escape_markdown(text: &str) -> String {
    text.replace("|", "|").replace("{", "{").replace("}", "}")
}

fn format_operand_names(operands: &[Operand]) -> String {
    if operands.is_empty() {
        "无".to_string()
    } else {
        operands
            .iter()
            .map(|op| format!("`{}`", op.name))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn format_operand_lengths(operands: &[Operand]) -> String {
    // 打印每个指令在 RV32 和 RV64 下的位长度
    if operands.is_empty() {
        "无".to_string()
    } else {
        let mut lengths = Vec::new();
        for op in operands {
            let mut length_str = String::new();
            for (base, length) in &op.bit_lengths {
                if !length_str.is_empty() {
                    length_str.push_str(", ");
                }
                length_str.push_str(&format!("{}:{}", base, length));
            }
            lengths.push(format!("`{}`: {}", op.name, length_str));
        }
        lengths.join("; ")
    }
}

fn format_operand_restrictions(operands: &[Operand]) -> String {
    if operands.is_empty() {
        "无".to_string()
    } else {
        let restrictions: Vec<String> = operands
            .iter()
            .filter_map(|op| {
                op.restrictions
                    .as_ref()
                    .map(|r| {
                        let desc = format_single_operand_restrictions(r);
                        if desc.is_empty() {
                            None
                        } else {
                            Some(format!("`{}`: {}", op.name, desc))
                        }
                    })
                    .flatten()
            })
            .collect();

        if restrictions.is_empty() {
            "无".to_string()
        } else {
            restrictions.join("; ")
        }
    }
}

fn format_single_operand_restrictions(restrictions: &OperandRestriction) -> String {
    let mut parts = Vec::new();

    if let Some(multiple) = restrictions.multiple_of {
        parts.push(format!("{}的倍数", multiple));
    }

    if let Some((min, max)) = restrictions.min_max {
        parts.push(format!("范围[{},{}]", min, max));
    }

    if !restrictions.forbidden_values.is_empty() {
        let forbidden_str = restrictions
            .forbidden_values
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(",");
        parts.push(format!("禁止:{}", forbidden_str));
    }

    parts.join(", ")
}
fn format_operand_length_distribution(length_map: &HashMap<ISABase, BTreeSet<u8>>) -> String {
    let format_lengths = |base: &ISABase| {
        length_map.get(base).map(|set| {
            set.iter()
                .map(|l| l.to_string())
                .collect::<Vec<_>>()
                .join(",")
        })
    };

    let rv32_lengths = format_lengths(&ISABase::RV32);
    let rv64_lengths = format_lengths(&ISABase::RV64);

    match (rv32_lengths, rv64_lengths) {
        (Some(rv32), Some(rv64)) => format!("RV32:{}, RV64:{}", rv32, rv64),
        (Some(rv32), None) => format!("RV32:{}, RV64:无", rv32),
        (None, Some(rv64)) => format!("RV32:无, RV64:{}", rv64),
        (None, None) => "RV32:无, RV64:无".to_string(),
    }
}

fn generate_operand_length_statistics(report: &mut String, instructions: &[Instruction]) {
    let mut length_stats: HashMap<(ISABase, u8), usize> = HashMap::new();

    for inst in instructions.iter() {
        for operand in &inst.operands {
            for (base, length) in &operand.bit_lengths {
                *length_stats.entry((*base, *length)).or_insert(0) += 1;
            }
        }
    }

    report.push_str("### 📐 操作数长度分布统计\n\n");
    report.push_str("| ISA基础 | 位长度 | 使用次数 | 占比 |\n");
    report.push_str("|---------|--------|----------|------|\n");

    let total_operands: usize = length_stats.values().sum();
    let mut stats_vec: Vec<_> = length_stats.iter().collect();
    stats_vec.sort_by_key(|((base, length), _)| (*base, *length));

    for ((base, length), count) in stats_vec {
        let percentage = if total_operands > 0 {
            *count as f64 / total_operands as f64 * 100.0
        } else {
            0.0
        };

        report.push_str(&format!(
            "| {} | {} | {} | {:.1}% |\n",
            base, length, count, percentage
        ));
    }

    report.push_str("\n");
}

fn generate_operand_restriction_statistics(report: &mut String, instructions: &[Instruction]) {
    let mut restriction_stats: HashMap<String, usize> = HashMap::new();
    let mut total_operands = 0;
    let mut restricted_operands = 0;

    for inst in instructions.iter() {
        for operand in &inst.operands {
            total_operands += 1;

            if let Some(restrictions) = &operand.restrictions {
                restricted_operands += 1;

                if restrictions.multiple_of.is_some() {
                    *restriction_stats.entry("倍数限制".to_string()).or_insert(0) += 1;
                }

                if restrictions.min_max.is_some() {
                    *restriction_stats.entry("范围限制".to_string()).or_insert(0) += 1;
                }

                if !restrictions.forbidden_values.is_empty() {
                    *restriction_stats
                        .entry("禁止值限制".to_string())
                        .or_insert(0) += 1;
                }
            }
        }
    }

    report.push_str("### 🚫 操作数限制条件统计\n\n");
    report.push_str("| 限制类型 | 使用次数 | 占受限操作数比例 |\n");
    report.push_str("|----------|----------|------------------|\n");

    for (restriction_type, count) in restriction_stats.iter() {
        let percentage = if restricted_operands > 0 {
            *count as f64 / restricted_operands as f64 * 100.0
        } else {
            0.0
        };

        report.push_str(&format!(
            "| {} | {} | {:.1}% |\n",
            restriction_type, count, percentage
        ));
    }

    report.push_str(&format!(
        "\n**总操作数**: {} 个，**受限操作数**: {} 个 ({:.1}%)\n\n",
        total_operands,
        restricted_operands,
        if total_operands > 0 {
            restricted_operands as f64 / total_operands as f64 * 100.0
        } else {
            0.0
        }
    ));
}
