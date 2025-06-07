use crate::types::*;
use std::{
    collections::{HashMap, HashSet}, // 确保 HashSet 已导入
    fs,
    path::Path,
};

pub fn parse_insts_from_riscv_unified_db<P: AsRef<Path>>(
    riscv_unified_db_repo_path: P,
) -> Result<Vec<Instruction>, Box<dyn std::error::Error>> {
    let inst_path = riscv_unified_db_repo_path.as_ref().join("arch/inst");

    let mut all_instructions = Vec::new();

    // 读取所有扩展文件夹
    for entry in fs::read_dir(&inst_path)? {
        let entry = entry?;
        let path = entry.path();

        // 跳过非目录文件
        if !path.is_dir() {
            continue;
        }

        let extension_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or_default();

        // 解析扩展名
        let extension = if let Some(ext) = ISAExtension::from_str(extension_name) {
            ext
        } else {
            panic!("未知扩展: {}", extension_name);
        };

        // 读取扩展文件夹中的所有YAML文件
        for inst_entry in fs::read_dir(&path)? {
            let inst_entry = inst_entry?;
            let inst_path = inst_entry.path();

            if inst_path.extension().and_then(|s| s.to_str()) != Some("yaml") {
                continue;
            }

            match parse_instruction_file(&inst_path, extension) {
                Ok(instruction) => {
                    all_instructions.push(instruction);
                }
                Err(e) => {
                    panic!("解析指令文件 {:?} 失败: {}", inst_path, e);
                }
            }
        }
    }

    Ok(all_instructions)
}

fn parse_instruction_file(
    file_path: &Path,
    extension: ISAExtension,
) -> Result<Instruction, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let yaml_inst: YamlInstruction = serde_yaml::from_str(&content)?;

    // 确定支持的ISA基础架构
    let isa_bases = determine_isa_bases(&yaml_inst);

    // 解析操作数
    let operands = parse_operands(&yaml_inst, &isa_bases)?;

    // 如果汇编语法与指令名称相同，或者与指令名称中点号替换为下划线后相同，则设置为空
    let name_with_underscores = yaml_inst.name.replace('.', "_");
    let assembly_syntax =
        if yaml_inst.assembly == yaml_inst.name || yaml_inst.assembly == name_with_underscores {
            AssemblySyntax::Format(String::new())
        } else {
            AssemblySyntax::Format(yaml_inst.assembly.clone())
        };

    let instruction = Instruction {
        name: yaml_inst.name,
        extension,
        isa_bases,
        operands,
        assembly_syntax,
    };

    Ok(instruction)
}

fn parse_operands(
    yaml_inst: &YamlInstruction,
    isa_bases: &[ISABase], // 新增参数
) -> Result<Vec<Operand>, Box<dyn std::error::Error>> {
    let mut operands = Vec::new();

    if let Some(encoding) = &yaml_inst.encoding {
        match encoding {
            Encoding::Simple { variables, .. } => {
                if let Some(variables) = variables {
                    for var in variables {
                        // 对于简单编码，所有ISA使用相同的位长度
                        // 但现在我们根据指令实际支持的ISA来填充
                        let bit_lengths_map = parse_bit_range_simple(&var.location, isa_bases)?;
                        let operand_bit_length =
                            bit_lengths_map.values().next().copied().unwrap_or(0);
                        let max_value_for_operand =
                            if operand_bit_length > 0 && operand_bit_length < 8 {
                                ((1u16 << operand_bit_length) - 1) as u64
                            } else if operand_bit_length == 8 {
                                u64::MAX
                            } else if operand_bit_length > 8 {
                                u64::MAX
                            } else {
                                0
                            };

                        let restrictions = parse_restrictions(var, max_value_for_operand);

                        let operand = Operand {
                            name: var.name.clone(),
                            operand_type: None,
                            bit_lengths: bit_lengths_map,
                            restrictions,
                        };

                        operands.push(operand);
                    }
                }
            }
            Encoding::PerISA { rv32, rv64 } => {
                // 收集所有变量名
                let mut all_variables: HashMap<String, HashMap<ISABase, String>> = HashMap::new();

                if let Some(rv32_enc) = rv32 {
                    if let Some(vars) = &rv32_enc.variables {
                        for var in vars {
                            all_variables
                                .entry(var.name.clone())
                                .or_insert_with(HashMap::new)
                                .insert(ISABase::RV32, var.location.clone());
                        }
                    }
                }

                if let Some(rv64_enc) = rv64 {
                    if let Some(vars) = &rv64_enc.variables {
                        for var in vars {
                            all_variables
                                .entry(var.name.clone())
                                .or_insert_with(HashMap::new)
                                .insert(ISABase::RV64, var.location.clone());
                        }
                    }
                }

                // 为每个变量创建操作数
                for (var_name, locations) in all_variables {
                    let mut bit_lengths_map = HashMap::new();
                    let mut max_bit_length = 0u8;

                    // 为每个ISA计算位长度
                    for (isa_base, location) in locations {
                        let bit_length = calculate_bit_length(&location)?;
                        bit_lengths_map.insert(isa_base, bit_length);
                        max_bit_length = max_bit_length.max(bit_length);
                    }

                    let max_value_for_operand = if max_bit_length > 0 && max_bit_length < 8 {
                        ((1u16 << max_bit_length) - 1) as u64
                    } else if max_bit_length == 8 {
                        u64::MAX
                    } else if max_bit_length > 8 {
                        u64::MAX
                    } else {
                        0
                    };

                    // 获取约束信息（从任一ISA的变量中获取）
                    let restrictions = if let Some(rv32_enc) = rv32 {
                        if let Some(vars) = &rv32_enc.variables {
                            if let Some(var) = vars.iter().find(|v| v.name == var_name) {
                                parse_restrictions(var, max_value_for_operand)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else if let Some(rv64_enc) = rv64 {
                        if let Some(vars) = &rv64_enc.variables {
                            if let Some(var) = vars.iter().find(|v| v.name == var_name) {
                                parse_restrictions(var, max_value_for_operand)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    let operand = Operand {
                        name: var_name,
                        operand_type: None,
                        bit_lengths: bit_lengths_map,
                        restrictions,
                    };

                    operands.push(operand);
                }
            }
        }
    }

    Ok(operands)
}

fn parse_bit_range_simple(
    location: &str,
    isa_bases: &[ISABase], // 新增参数
) -> Result<HashMap<ISABase, u8>, Box<dyn std::error::Error>> {
    let bit_length = calculate_bit_length(location)?;

    let mut bit_lengths = HashMap::new();
    for base in isa_bases {
        bit_lengths.insert(*base, bit_length);
    }

    Ok(bit_lengths)
}

fn calculate_bit_length(location: &str) -> Result<u8, Box<dyn std::error::Error>> {
    let mut total_length = 0u8;

    // 处理用"|"分隔的多个位范围，例如 "31-25|11-7" 或 "12|6-2"
    for part in location.split('|') {
        let part = part.trim();

        if let Some((high_str, low_str)) = part.split_once('-') {
            // 位范围格式：high-low
            let high: u8 = high_str
                .trim()
                .parse()
                .map_err(|e| format!("无法解析高位 '{}': {}", high_str.trim(), e))?;
            let low: u8 = low_str
                .trim()
                .parse()
                .map_err(|e| format!("无法解析低位 '{}': {}", low_str.trim(), e))?;

            if high < low {
                return Err(format!("位范围无效: {} < {}", high, low).into());
            }

            let range_length = high - low + 1;
            total_length += range_length;
        } else {
            total_length += 1;
        }
    }

    Ok(total_length)
}

fn parse_restrictions(var: &Variable, max_value_for_operand: u64) -> Option<OperandRestriction> {
    let mut restriction = OperandRestriction::default();
    let mut has_any_restriction_field_set = false;

    // 解析 "left_shift" 约束
    if let Some(left_shift_yaml) = &var.left_shift {
        has_any_restriction_field_set = true;
        let multiple_of_value = 1u16 << left_shift_yaml; // 2^shift_amount
        if multiple_of_value <= u16::MAX {
            restriction.multiple_of = Some(multiple_of_value.try_into().unwrap());
        }
    }

    // 解析 "not" 约束
    if let Some(not_values_yaml) = &var.not_values {
        has_any_restriction_field_set = true;
        let mut temp_forbidden_values = HashSet::new();

        match not_values_yaml {
            serde_yaml::Value::Number(n) => {
                if let Some(val_u64) = n.as_u64() {
                    if val_u64 <= i64::MAX as u64 {
                        let val_i64 = val_u64 as i64;
                        temp_forbidden_values.insert(val_i64);
                    }
                }
            }
            serde_yaml::Value::Sequence(seq) => {
                for item in seq {
                    if let Some(n_u64) = item.as_u64() {
                        if n_u64 <= i64::MAX as u64 {
                            let val_i64 = n_u64 as i64;
                            temp_forbidden_values.insert(val_i64);
                        }
                    }
                }
            }
            _ => {} // 其他类型的值不处理
        }

        // 如果已经有multiple_of约束，检查forbidden_values是否与其冲突
        if let Some(multiple_of) = restriction.multiple_of {
            // 过滤掉那些不符合multiple_of约束但被forbidden的值（它们本来就是无效的）
            temp_forbidden_values.retain(|&val| val % (multiple_of as i64) == 0);

            // 直接设置过滤后的forbidden_values
            if !temp_forbidden_values.is_empty() {
                let mut forbidden_values: Vec<i64> = temp_forbidden_values.into_iter().collect();
                forbidden_values.sort_unstable();
                restriction.forbidden_values = forbidden_values;
            }
        } else {
            // 检查是否所有奇数都被禁止（暗示 multiple_of: 2）
            let all_odd_numbers_in_range: HashSet<i64> =
                (1..=max_value_for_operand as i64).step_by(2).collect();

            if !all_odd_numbers_in_range.is_empty()
                && all_odd_numbers_in_range.is_subset(&temp_forbidden_values)
            {
                restriction.multiple_of = Some(2);

                // 计算被此模式解释的值
                let mut explained_by_pattern = all_odd_numbers_in_range;
                // 如果0本身也被禁止 (通过检查temp_forbidden_values)
                if temp_forbidden_values.contains(&0) {
                    explained_by_pattern.insert(0);
                }

                // 剩余的禁止值（不被 multiple_of:2 和 (可能的) forbidden 0 覆盖的）
                let remaining_forbidden: Vec<i64> = temp_forbidden_values
                    .difference(&explained_by_pattern)
                    .cloned()
                    .collect();

                let mut sorted_remaining: Vec<i64> = remaining_forbidden;
                sorted_remaining.sort_unstable(); // 保持一致性
                restriction.forbidden_values = sorted_remaining;
            } else {
                // 如果不符合 "multiple_of: 2" 模式，则直接使用收集到的禁止值
                let mut all_forbidden_from_yaml: Vec<_> =
                    temp_forbidden_values.into_iter().collect();
                all_forbidden_from_yaml.sort_unstable(); // 保持一致性
                restriction.forbidden_values = all_forbidden_from_yaml;
            }
        }
    }

    // 如果 restriction 结构中的任何字段被设置（即不等于默认值），则返回 Some
    if restriction != OperandRestriction::default() || has_any_restriction_field_set {
        if restriction != OperandRestriction::default() {
            Some(restriction)
        } else if has_any_restriction_field_set {
            // 例如 not: [] 的情况
            Some(OperandRestriction::default()) // 返回一个空的（默认的）约束
        } else {
            None
        }
    } else {
        None
    }
}

fn determine_isa_bases(yaml_inst: &YamlInstruction) -> Vec<ISABase> {
    // 首先检查是否有明确的 base 字段
    if let Some(base_value) = &yaml_inst.base {
        match base_value {
            32 => return vec![ISABase::RV32],
            64 => return vec![ISABase::RV64],
            _ => {}
        }
    }

    // 其他默认全部支持
    vec![ISABase::RV32, ISABase::RV64]
}
