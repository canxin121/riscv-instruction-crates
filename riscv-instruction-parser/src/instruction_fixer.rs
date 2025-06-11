use crate::{operand_extractor::extract_operands_from_asm_without_name, types::*};
use std::collections::{HashMap, HashSet};

/// 修复指令定义中的所有问题
pub fn fix_instructions(instructions: &mut [Instruction]) {
    for instruction in instructions.iter_mut() {
        fix_instruction(instruction);
    }
}

/// 修复单个指令
fn fix_instruction(instruction: &mut Instruction) {
    // 不应该有RustCode语法的指令, 因为修复前的指令应该都是YAML格式的转换來的, 应该是Format类型
    if matches!(instruction.assembly_syntax, AssemblySyntax::RustCode(_)) {
        panic!(
            "Unexpected RustCode syntax for instruction: {}",
            instruction.name
        );
    }

    let mut assembly_syntax = match &instruction.assembly_syntax {
        AssemblySyntax::Format(s) => s.clone(),
        AssemblySyntax::RustCode(_) => unreachable!(),
    };

    // 提取原始汇编语法中的操作数
    let mut syntax_operands = extract_operands_from_asm_without_name(&assembly_syntax);

    // 协调操作数名称映射, 将instruction.operands中的名称修改成与汇编语法一致的名称
    reconcile_operand_names(instruction, &syntax_operands);

    // 处理特殊指令
    handle_special_instruction(&mut assembly_syntax, instruction, &mut syntax_operands);

    // 根据指令类型生成最终汇编语法
    instruction.assembly_syntax = generate_assembly_syntax(
        &instruction.name,
        &assembly_syntax,
        &syntax_operands,
        instruction,
    );
    // 为每个操作数分配operand_type
    assign_operand_types(instruction);
}

/// 根据指令类型生成最终汇编语法
fn generate_assembly_syntax(
    name: &str,
    assembly_syntax: &str,
    syntax_operands: &HashSet<String>,
    instruction: &Instruction,
) -> AssemblySyntax {
    let operands_segment = wrap_operands_with_braces(&assembly_syntax, &syntax_operands);

    if name == "c.lui" {
        // c.lui 指令使用特殊的格式化代码
        AssemblySyntax::RustCode(
            "format!(\"c.lui {}, 0x{:x}\", xd, imm.get() as u32 & 0xfffff)".to_string(),
        )
    } else if name == "fcvtmod.w.d" {
        // fcvtmod.w.d 指令使用硬编码的 rtz 舍入模式
        AssemblySyntax::Format(format!("fcvtmod.w.d {}, rtz", operands_segment))
    } else if matches!(name, "cm.pop" | "cm.push" | "cm.popretz" | "cm.popret") {
        // cm.* 指令使用特殊的格式化代码处理 SavedRegListWithStackAdj
        AssemblySyntax::RustCode(generate_cm_instruction_assembly_code(name))
    } else if matches!(name, "c.sh" | "c.lh" | "c.lhu") {
        // 这些压缩指令的 uimm 是 1 位布尔值，需要转换为偏移量
        AssemblySyntax::RustCode(generate_compressed_offset_assembly_code(
            name,
            &operands_segment,
            instruction,
        ))
    } else if matches!(name, "vs1r.v" | "vs2r.v" | "vs4r.v" | "vs8r.v") {
        // vs*r.v 指令使用地址偏移格式
        AssemblySyntax::Format(format!("{} {{vs3}}, 0({{xs1}})", name))
    } else if is_aqrl_instruction(instruction) {
        // 对于包含aqrl的指令，生成特定的 Rust 代码
        AssemblySyntax::RustCode(generate_aqrl_assembly_code(
            name,
            &operands_segment,
            instruction,
        ))
    } else if is_mop_instruction(instruction) {
        // 对于 MOP 指令，生成特定的 Rust 代码
        AssemblySyntax::RustCode(generate_mop_assembly_code(
            name,
            &operands_segment,
            instruction,
        ))
    } else if is_vector_instruction_with_vm(instruction) {
        // 对于包含 vm 操作数的向量指令，生成特定的 Rust 代码
        AssemblySyntax::RustCode(generate_vector_vm_assembly_code(
            name,
            &operands_segment,
            instruction,
        ))
    } else if matches!(
        name,
        "sspush.x1" | "sspush.x5" | "sspopchk.x1" | "sspopchk.x5"
    ) {
        // 影子栈指令使用硬编码格式
        AssemblySyntax::Format(name.replace(".", " "))
    } else {
        // 对于普通指令, 添加指令名称 + 包裹操作数
        let final_assembly = if operands_segment.trim().is_empty() {
            name.to_string()
        } else {
            format!("{} {}", name, operands_segment)
        };
        AssemblySyntax::Format(final_assembly)
    }
}

/// 协调操作数名称映射
fn reconcile_operand_names(
    instruction: &mut Instruction,
    syntax_operands: &std::collections::HashSet<String>,
) {
    if syntax_operands.is_empty() && instruction.operands.is_empty() {
        return;
    }

    let name_mapping = create_operand_name_mapping();
    let mut available_syntax_ops: Vec<String> = syntax_operands.iter().cloned().collect();
    let mut updated_operands = Vec::with_capacity(instruction.operands.len());

    for operand in &instruction.operands {
        let new_name =
            find_matching_operand_name(&operand.name, &mut available_syntax_ops, &name_mapping);

        updated_operands.push(Operand {
            name: new_name,
            operand_type: operand.operand_type.clone(),
            bit_lengths: operand.bit_lengths.clone(),
            restrictions: operand.restrictions.clone(),
        });
    }

    instruction.operands = updated_operands;
}

/// 创建操作数名称映射表
fn create_operand_name_mapping() -> HashMap<&'static str, Vec<&'static str>> {
    [
        ("rd", vec!["xd", "fd", "qd"]),
        ("rs1", vec!["xs1", "fs1", "qs1"]),
        ("rs2", vec!["xs2", "fs2", "qs2"]),
        ("rs3", vec!["xs3", "fs3"]),
        ("fd", vec!["xd"]),
        ("fs1", vec!["xs1"]),
        ("fs2", vec!["xs2"]),
        ("fs3", vec!["xs3"]),
        ("rm", vec!["frm"]),
        ("rlist", vec!["reg_list"]),
        ("spimm", vec!["stack_adj"]),
        ("imm", vec!["offset", "imm12", "uimm"]),
        ("uimm", vec!["imm", "nzuimm"]),
        ("nzuimm", vec!["uimm", "imm"]),
        ("zimm5", vec!["imm"]),
    ]
    .iter()
    .map(|(k, v)| (*k, v.to_vec()))
    .collect()
}

/// 查找匹配的操作数名称
fn find_matching_operand_name(
    current_name: &str,
    available_syntax_ops: &mut Vec<String>,
    name_mapping: &HashMap<&str, Vec<&str>>,
) -> String {
    // 优先直接匹配
    if let Some(pos) = available_syntax_ops.iter().position(|x| x == current_name) {
        return available_syntax_ops.remove(pos);
    }

    // 尝试映射匹配
    if let Some(possible_names) = name_mapping.get(current_name) {
        for &mapped_name in possible_names {
            if let Some(pos) = available_syntax_ops.iter().position(|x| x == mapped_name) {
                return available_syntax_ops.remove(pos);
            }
        }
    }

    current_name.to_string()
}

/// 统一处理所有特殊指令的汇编语法和操作数
fn handle_special_instruction(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    // 修复 vsetvli 中 vtypei 被错误命名为 xs1 的情况
    fix_vsetvli_operand_names(instruction);
    // 修复 fence 指令的 assembly 语法被标记为 "TODO" 的情况
    fix_fence_instruction(operands_part, instruction, final_syntax_operands);
    // 修复 c.lui 指令的 imm 的特殊范围
    fix_clui_instruction(operands_part, instruction, final_syntax_operands);
    // 修复几条需要去除 round mode 的指令
    fix_round_mode_removal(operands_part, instruction, final_syntax_operands);
    // 修复 fli 指令的第二个操作数类型错误
    fix_fli_instruction_operands(operands_part, instruction, final_syntax_operands);
    // 修复 uimm 被误写为 imm 的一些指令
    fix_uimm_instead_of_imm(operands_part, instruction, final_syntax_operands);
    // 修复特定浮点加载/存储指令的基址+偏移量寻址语法
    fix_floating_point_load_store_addressing(operands_part, instruction, final_syntax_operands);
    // 修复浮点寄存器和整数寄存器误用的一些指令
    fix_floating_point_register_errors(instruction, operands_part, final_syntax_operands);
    // 修复向量寄存器和整数寄存器误用的一些指令
    fix_vector_register_errors_wrapper(instruction, operands_part, final_syntax_operands);
    // 修复所有缺失的 round mode
    fix_missing_round_mode(operands_part, instruction, final_syntax_operands);
    fix_compressed_stack_pointer_instructions(operands_part, instruction, final_syntax_operands);
    // 修复缺少 (sp) 后缀的一些相关指令
    fix_compressed_stack_adjustment_instructions(operands_part, instruction);
    // 修复 adduw 指令缺失操作数的情况
    fix_add_uw_missing_operands(instruction);
    // 修复超级用户模式加载/存储指令的地址语法
    fix_hypervisor_load_store_instructions(operands_part, instruction);
    // 修复 aes64ks1i 指令的 rnum 操作数范围限制
    fix_aes64ks1i_rnum_range(instruction);
    // 修复 isabase错误(如仅rv32的却被标记为rv32+rv64) 相关指令的特殊情况
    fix_isabase_error_instructions(instruction);
    // 修复 c.mop.n 指令的 n 没有限定为奇数的问题
    fix_c_mop_n_instruction(instruction);
    // 修复 cm.* 指令的 reg_list 和 stack_adj 合并问题
    fix_cm_instructions_reg_list_stack_adj(instruction, operands_part, final_syntax_operands);
    // 修复 cm.mvsa01 指令的操作数合并问题
    fix_cm_mvsa01_instruction(instruction, operands_part, final_syntax_operands);

    // 应当最后运行
    // 确保 v0 不在最终的语法操作数中
    fix_vector_v0_hardcoded_operands(final_syntax_operands);
    // 确保 sp 不在最终的语法操作数中
    fix_sp_should_not_in_final_syntax_operands(final_syntax_operands);
}

fn fix_c_mop_n_instruction(instruction: &mut Instruction) {
    // 修复 c.mop.n 指令的 n 没有限定为奇数的问题
    if instruction.name == "c.mop.n" {
        instruction
            .operands
            .iter_mut()
            .find(|op| op.name == "n")
            .map(|op| {
                op.restrictions = Some(OperandRestriction {
                    multiple_of: None,
                    min_max: None,
                    odd_only: Some(true),
                    forbidden_values: Vec::with_capacity(0),
                });
            });
    }
}

fn fix_add_uw_missing_operands(instruction: &mut Instruction) {
    // 为 adduw 指令添加缺失的操作数
    if instruction.name == "add.uw" {
        instruction.operands.push(Operand {
            name: "xd".to_string(),
            operand_type: Some(OperandType::IntegerRegister),
            bit_lengths: HashMap::from([(ISABase::RV32, 5), (ISABase::RV64, 5)]),
            restrictions: None,
        });

        instruction.operands.push(Operand {
            name: "xs1".to_string(),
            operand_type: Some(OperandType::IntegerRegister),
            bit_lengths: HashMap::from([(ISABase::RV32, 5), (ISABase::RV64, 5)]),
            restrictions: None,
        });

        instruction.operands.push(Operand {
            name: "xs2".to_string(),
            operand_type: Some(OperandType::IntegerRegister),
            bit_lengths: HashMap::from([(ISABase::RV32, 5), (ISABase::RV64, 5)]),
            restrictions: None,
        });
    }
}

fn fix_sp_should_not_in_final_syntax_operands(
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    // 确保 sp 不在最终的语法操作数中
    if final_syntax_operands.contains("sp") {
        final_syntax_operands.remove("sp");
    }
}

/// 修复 vsetvli 指令操作数名称错误
fn fix_vsetvli_operand_names(instruction: &mut Instruction) {
    if instruction.name == "vsetvli" {
        instruction
            .operands
            .iter_mut()
            .find(|op| op.name == "xs1" && op.bit_lengths.get(&ISABase::RV32) == Some(&11))
            .map(|op| op.name = "vtypei".to_string());
    }
}

/// 修复 fence 指令被描述为"TODO"的情况
fn fix_fence_instruction(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if instruction.name != "fence" {
        return;
    }

    *operands_part = "pred, succ".to_string();
    final_syntax_operands.insert("pred".to_string());
    final_syntax_operands.insert("succ".to_string());
}

/// 修复 c.lui 指令的特殊情况
fn fix_clui_instruction(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if instruction.name != "c.lui" {
        return;
    }

    // 修正操作数限制：从multiple_of: 4096改为范围限制和禁止值
    instruction
        .operands
        .iter_mut()
        .find(|op| op.name == "imm")
        .map(|op| {
            op.restrictions = Some(OperandRestriction {
                multiple_of: None,
                min_max: Some((-32, 31)),
                forbidden_values: vec![0],
                odd_only: None,
            });
        });

    // 清空操作数部分，因为汇编语法将由 generate_assembly_syntax 处理
    *operands_part = String::new();
    final_syntax_operands.clear();
}

/// 修复需要去除round mode的特定指令
fn fix_round_mode_removal(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !matches!(
        instruction.name.as_str(),
        "fcvt.d.w"
            | "fcvt.d.wu"
            | "fcvt.d.s"
            | "fcvt.q.w"
            | "fcvt.q.wu"
            | "fcvt.q.s"
            | "fcvt.q.d"
            | "fcvt.q.h"
            | "fcvt.s.bf16"
            | "fcvt.s.h"
            | "fcvt.d.h"
            | "fcvtmod.w.d"
    ) {
        return;
    }

    // 从操作数定义中移除rm操作数
    instruction
        .operands
        .retain(|op| op.name != "rm" && op.name != "frm");

    // 从汇编语法中移除round mode相关的引用
    *operands_part = operands_part.replace(", rm", "").replace(", frm", "");

    // 从语法操作数集合中移除
    final_syntax_operands.remove("rm");
    final_syntax_operands.remove("frm");
}

/// 修复 FLI 指令的操作数类型错误
fn fix_fli_instruction_operands(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !matches!(
        instruction.name.as_str(),
        "fli.d" | "fli.s" | "fli.h" | "fli.q"
    ) {
        return;
    }

    // 这些指令的第二个操作数应该是无符号立即数，而不是寄存器
    if let Some(operand) = instruction
        .operands
        .iter_mut()
        .find(|op| matches!(op.name.as_str(), "qs1" | "fs1" | "xs1" | "imm"))
    {
        let old_name = std::mem::replace(&mut operand.name, "uimm".to_string());
        operand.operand_type = Some(OperandType::FliConstant);

        if final_syntax_operands.remove(&old_name) {
            final_syntax_operands.insert("uimm".to_string());
            *operands_part = operands_part.replace(&old_name, "uimm");
        }
    }
}

/// 修复需要将imm修正为uimm的指令
fn fix_uimm_instead_of_imm(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !matches!(
        instruction.name.as_str(),
        // 基本指令
        "lui" | "auipc" |
        // CSR指令
        "csrrsi" | "csrrwi" | "csrrci" |
        // 压缩指令中的立即数（通常是无符号的）
        "c.fswsp" | "c.sdsp" | "c.lwsp" | "c.ldsp" | "c.fsdsp" | "c.fldsp" | "c.swsp" | "c.flwsp" | "c.fld" | "c.fsd" | "c.ld" | "c.sd" | "c.sw" | "c.lw" | "c.flw" | "c.fsw"| "c.addi4spn" | "c.srai" | "c.srli" | "c.slli" |
        // Zcb 扩展
        "c.lbu" | "c.lh" | "c.lhu" | "c.sb" | "c.sh" |
        // V 扩展 - 向量滑动指令
        "vslidedown.vi" | "vslideup.vi" |
        // V 扩展 - 向量收集指令
        "vrgather.vi" |
        // V 扩展 - 向量逻辑/算术移位指令
        "vsll.vi" | "vsrl.vi" | "vsra.vi" |
        // V 扩展 - 向量窄化饱和指令
        "vnclipu.wi" |
        // V 扩展 - 向量窄化/饱和移位指令
        "vnsra.wi" | "vnsrl.wi" | "vssra.wi" | "vssrl.wi" |
        // V 扩展 - 向量饱和算术/逻辑右移立即数指令
        "vssra.vi" | "vssrl.vi" |
        // Zvbb 扩展
        "vror.vi" |
        // Zvbb 扩展 - 向量加宽移位指令
        "vwsll.vi" |
        "vnclip.wi"|
        "lpad" |
        "vaeskf1.vi" | "vaeskf2.vi" | "vsm3c.vi" | "vsm4k.vi"
    ) {
        return;
    }

    // 修正操作数名称：将imm改为uimm
    for operand in instruction.operands.iter_mut() {
        if operand.name == "imm" {
            operand.name = "uimm".to_string();
        }
    }

    // 修正汇编语法中的操作数引用
    if final_syntax_operands.contains("imm") {
        final_syntax_operands.remove("imm");
        final_syntax_operands.insert("uimm".to_string());
        *operands_part = operands_part.replace("imm", "uimm");
    }
}

/// 修复特定浮点加载/存储指令的基址+偏移量寻址语法
fn fix_floating_point_load_store_addressing(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !matches!(instruction.name.as_str(), "flw" | "fsw" | "fld" | "flq") {
        return;
    }

    let (expected_first_reg, expected_base_reg, expected_offset_imm) =
        match instruction.name.as_str() {
            "flw" => ("fd", "xs1", "imm"),
            "fsw" => ("fs2", "xs1", "imm"), // fs2 是源寄存器
            "fld" => ("fd", "xs1", "imm"),
            "flq" => ("fd", "xs1", "imm"),
            _ => return,
        };

    let current_operands_str = operands_part.trim();
    let parts: Vec<&str> = current_operands_str.split(',').map(|s| s.trim()).collect();

    if parts.len() == 3
        && parts[0] == expected_first_reg
        && parts[1] == expected_base_reg
        && parts[2] == expected_offset_imm
        && final_syntax_operands.contains(expected_first_reg)
        && final_syntax_operands.contains(expected_base_reg)
        && final_syntax_operands.contains(expected_offset_imm)
    {
        // 从 "reg, base, imm" 转换为 "reg, imm(base)"
        *operands_part = format!(
            "{}, {}({})",
            expected_first_reg, expected_offset_imm, expected_base_reg
        );
    }
}

/// 修复向量寄存器类型错误的包装函数
fn fix_vector_register_errors_wrapper(
    instruction: &mut Instruction,
    operands_part: &mut String,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    match instruction.name.as_str() {
        "vmv.x.s" => {
            fix_operand_name(instruction, "vd", "xd");
            update_syntax_operands(operands_part, final_syntax_operands, "vd", "xd");
        }
        "vcpop.m" => {
            fix_operand_name(instruction, "vd", "xd");
            update_syntax_operands(operands_part, final_syntax_operands, "vd", "xd");
        }
        _ => {}
    }
}

/// 修复round mode丢失的情况
fn fix_missing_round_mode(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !instruction.operands.iter().any(|o| o.name == "rm") || final_syntax_operands.contains("rm")
    {
        return;
    }

    if operands_part.trim().is_empty() {
        *operands_part = "rm".to_string();
    } else {
        *operands_part = format!(
            "{}, rm",
            operands_part.trim_end_matches(|c: char| c.is_whitespace() || c == ',')
        );
    }

    final_syntax_operands.insert("rm".to_string());
}

/// 修复压缩栈指针相关指令 - 添加 (sp) 后缀
fn fix_compressed_stack_pointer_instructions(
    operands_part: &mut String,
    instruction: &mut Instruction,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !matches!(
        instruction.name.as_str(),
        "c.flwsp" | "c.fswsp" | "c.ldsp" | "c.lwsp" | "c.sdsp" | "c.swsp" | "c.fsdsp" | "c.fldsp"
    ) {
        return;
    }

    let expected_pattern = match instruction.name.as_str() {
        "c.flwsp" => ("fd", "uimm"),
        "c.fswsp" => ("fs2", "uimm"),
        "c.ldsp" => ("xd", "uimm"),
        "c.lwsp" => ("xd", "uimm"),
        "c.sdsp" => ("xs2", "uimm"),
        "c.swsp" => ("xs2", "uimm"),
        "c.fsdsp" => ("fs2", "uimm"),
        "c.fldsp" => ("fd", "uimm"),
        _ => return,
    };

    let current_operands_str = operands_part.trim();
    let parts: Vec<&str> = current_operands_str.split(',').map(|s| s.trim()).collect();

    if parts.len() == 2
        && parts[0] == expected_pattern.0
        && parts[1] == expected_pattern.1
        && final_syntax_operands.contains(expected_pattern.0)
        && final_syntax_operands.contains(expected_pattern.1)
    {
        // 从 "{reg}, {uimm}" 转换为 "{reg}, {uimm}(sp)"
        *operands_part = format!("{}, {}(sp)", expected_pattern.0, expected_pattern.1);
    }
}

/// 修复 c.addi4spn 和 c.addi16sp 指令 - 添加 sp 寄存器
fn fix_compressed_stack_adjustment_instructions(
    operands_part: &mut String,
    instruction: &mut Instruction,
) {
    if instruction.name == "c.addi4spn" {
        // 从 "xd, uimm" 转换为 "xd, sp, uimm"
        *operands_part = "xd, sp, uimm".to_string();
    } else if instruction.name == "c.addi16sp" {
        *operands_part = "sp, imm".to_string();
    }
}

/// 修复向量指令中的v0硬编码操作数
fn fix_vector_v0_hardcoded_operands(final_syntax_operands: &mut std::collections::HashSet<String>) {
    final_syntax_operands.remove("v0");
}

/// 修复浮点寄存器类型错误
fn fix_floating_point_register_errors(
    instruction: &mut Instruction,
    operands_part: &mut String,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    match instruction.name.as_str() {
        // Q 扩展指令
        "fcvt.d.q" => {
            // 修正：xd -> fd (目标应为浮点寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        "fcvt.h.q" => {
            // 修正：xd -> hd (目标应为半精度浮点寄存器)
            fix_operand_name(instruction, "xd", "hd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "hd");
        }
        "fcvt.lu.q" => {
            // 修正：hs1 -> xs1 (源应为整数寄存器)
            fix_operand_name(instruction, "qd", "xd");
            update_syntax_operands(operands_part, final_syntax_operands, "qd", "xd");
        }
        "fcvt.wu.q" => {
            // 修正：xs1 -> qs1 (源应为四精度浮点寄存器)
            fix_operand_name(instruction, "xs1", "qs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "qs1");
        }
        "fmin.q" => {
            // 修正：所有整数寄存器 -> 四精度浮点寄存器
            fix_operand_name(instruction, "xd", "qd");
            fix_operand_name(instruction, "xs1", "qs1");
            fix_operand_name(instruction, "xs2", "qs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "qd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "qs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "qs2");
        }
        // F 扩展指令
        "fmaxm.s" | "fmin.s" => {
            // 修正：所有整数寄存器 -> 单精度浮点寄存器
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            fix_operand_name(instruction, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
        }
        "fnmsub.s" => {
            // 修正：所有整数寄存器 -> 单精度浮点寄存器
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            fix_operand_name(instruction, "xs2", "fs2");
            fix_operand_name(instruction, "xs3", "fs3");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xs3", "fs3");
        }
        "fround.s" | "froundnx.s" => {
            // 修正：源操作数 xs1 -> fs1 (源应为单精度浮点寄存器)
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        // D 扩展 - 浮点算术运算指令 (所有操作数都应为浮点寄存器)
        "fadd.d" | "fsub.d" | "fmul.d" | "fdiv.d" => {
            // 修正：xd -> fd, xs1 -> fs1, xs2 -> fs2
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            fix_operand_name(instruction, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
        }
        "fsqrt.d" | "fround.d" | "froundnx.d" => {
            // 修正：xd -> fd, xs1 -> fs1
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        "fmadd.d" | "fmsub.d" | "fnmadd.d" | "fnmsub.d" => {
            // 修正：xd -> fd, xs1 -> fs1, xs2 -> fs2, xs3 -> fs3
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            fix_operand_name(instruction, "xs2", "fs2");
            fix_operand_name(instruction, "xs3", "fs3");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xs3", "fs3");
        }
        "fli.d" => {
            // 修正：xd -> fd
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        // D 扩展 - 整数转浮点 (源为整数寄存器，目标为浮点寄存器)
        "fcvt.d.l" | "fcvt.d.lu" | "fcvt.d.w" | "fcvt.d.wu" => {
            // 修正：xd -> fd (xs1保持为整数寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        // D 扩展 - 浮点转整数 (源为浮点寄存器，目标为整数寄存器)
        "fcvt.l.d" | "fcvt.lu.d" | "fcvt.w.d" | "fcvt.wu.d" | "fcvtmod.w.d" => {
            // 修正：xs1 -> fs1 (xd保持为整数寄存器)
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        // D 扩展 - 浮点间转换
        "fcvt.d.s" => {
            // 修正：xd -> fd, xs1 -> fs1
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        "fcvt.s.d" => {
            // 修正：xd -> fd, xs1 -> fs1
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        // D 扩展 - 浮点比较指令 (源为浮点寄存器，目标为整数寄存器)
        "feq.d" | "fle.d" | "fleq.d" | "flt.d" | "fltq.d" => {
            // 修正：xs1 -> fs1, xs2 -> fs2 (xd保持为整数寄存器)
            fix_operand_name(instruction, "xs1", "fs1");
            fix_operand_name(instruction, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
        }
        "fclass.d" => {
            // 修正：xs1 -> fs1 (xd保持为整数寄存器)
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        // D 扩展 - 浮点取极值、符号操作 (所有操作数都应为浮点寄存器)
        "fmax.d" | "fmaxm.d" | "fmin.d" | "fminm.d" | "fsgnj.d" | "fsgnjn.d" | "fsgnjx.d" => {
            // 修正：xd -> fd, xs1 -> fs1, xs2 -> fs2
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            fix_operand_name(instruction, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
        }
        // D 扩展 - 浮点移动指令
        "fmv.d.x" => {
            // 修正：xd -> fd (xs1保持为整数寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        "fmv.x.d" | "fmvh.x.d" => {
            // 修正：xs1 -> fs1 (xd保持为整数寄存器)
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        "fmvp.d.x" => {
            // 修正：xd -> fd (目标为浮点寄存器，源保持为整数寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            // 注意：xs1 和 xs2 应该保持为整数寄存器，不需要修正
        }
        // Zcd 扩展 - 压缩双精度浮点指令
        "c.fld" => {
            // 修正：xd -> fd (浮点加载指令，目标应为浮点寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        "c.fsd" => {
            // 修正：xs2 -> fs2 (浮点存储指令，源应为浮点寄存器)
            fix_operand_name(instruction, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
        }
        // Zcf 扩展 - 压缩单精度浮点指令
        "c.flw" => {
            // 修正：xd -> fd (浮点加载指令，目标应为浮点寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        "c.fsw" => {
            // 修正：xs2 -> fs2 (浮点存储指令，源应为浮点寄存器)
            fix_operand_name(instruction, "xs2", "fs2");
            update_syntax_operands(operands_part, final_syntax_operands, "xs2", "fs2");
        }
        // Zfbfmin 扩展 - 标量BF16浮点指令
        "fcvt.bf16.s" | "fcvt.s.bf16" => {
            // 修正：xd -> fd, xs1 -> fs1 (浮点转换指令，源和目标都应为浮点寄存器)
            fix_operand_name(instruction, "xd", "fd");
            fix_operand_name(instruction, "xs1", "fs1");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xs1", "fs1");
        }
        // Zfh 扩展 - 半精度浮点指令
        "fclass.h" | "fcvt.l.h" | "fcvt.lu.h" | "fcvt.w.h" | "fcvt.wu.h" | "feq.h" | "fle.h"
        | "fleq.h" | "flt.h" | "fltq.h" => {
            // 修正：fd -> xd (浮点比较指令的结果应写入整数寄存器)
            fix_operand_name(instruction, "fd", "xd");
            update_syntax_operands(operands_part, final_syntax_operands, "fd", "xd");
        }
        "flh" | "fcvt.h.wu" | "fcvt.h.w" | "fcvt.h.lu" | "fcvt.h.l" => {
            // 修正：xd -> fd (浮点加载指令，目标应为浮点寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        "fmv.h.x" => {
            // 修正：xd -> fd (从整数寄存器移动到浮点寄存器)
            fix_operand_name(instruction, "xd", "fd");
            update_syntax_operands(operands_part, final_syntax_operands, "xd", "fd");
        }
        "fmv.x.h" => {
            // 修正：fd -> xd (从浮点寄存器移动到整数寄存器)
            fix_operand_name(instruction, "fd", "xd");
            update_syntax_operands(operands_part, final_syntax_operands, "fd", "xd");
        }
        "fltq.q" => {
            // 修正：qd -> xd (浮点比较指令的结果应写入整数寄存器)
            fix_operand_name(instruction, "fd", "xd");
            update_syntax_operands(operands_part, final_syntax_operands, "fd", "xd");
        }

        _ => {}
    }
}

/// 修复指令中的操作数名称
fn fix_operand_name(instruction: &mut Instruction, old_name: &str, new_name: &str) {
    for operand in instruction.operands.iter_mut() {
        if operand.name == old_name {
            operand.name = new_name.to_string();
        }
    }
}

/// 更新汇编语法中的操作数引用
fn update_syntax_operands(
    operands_part: &mut String,
    final_syntax_operands: &mut std::collections::HashSet<String>,
    old_name: &str,
    new_name: &str,
) {
    if final_syntax_operands.contains(old_name) {
        final_syntax_operands.remove(old_name);
        final_syntax_operands.insert(new_name.to_string());
        *operands_part = operands_part.replace(old_name, new_name);
    }
}

/// 用花括号包裹操作数
fn wrap_operands_with_braces(
    asm_without_name: &str,
    syntax_operands: &std::collections::HashSet<String>,
) -> String {
    if asm_without_name.trim().is_empty() || syntax_operands.is_empty() {
        return asm_without_name.to_string();
    }

    let mut result = asm_without_name.to_string();

    for operand_name in syntax_operands {
        result = result.replace(operand_name, &format!("{{{}}}", operand_name));
    }

    result
}

/// 检查是否为AMO指令
fn is_aqrl_instruction(instruction: &Instruction) -> bool {
    instruction
        .operands
        .iter()
        .any(|o| o.name == "aq" || o.name == "rl")
}

/// 检查是否需要地址包裹的aqrl指令
fn needs_address_wrapping(name: &str) -> bool {
    matches!(
        name,
        "ssamoswap.d" | "ssamoswap.w" | "sc.d" | "sc.w" | "lr.d" | "lr.w"
    )
}

/// 生成包含aqrl的指令的汇编代码
fn generate_aqrl_assembly_code(
    name: &str,
    operands_segment: &str,
    instruction: &Instruction,
) -> String {
    let has_aq = instruction.operands.iter().any(|o| o.name == "aq");
    let has_rl = instruction.operands.iter().any(|o| o.name == "rl");

    // 检查是否需要地址包裹
    if needs_address_wrapping(name) {
        // 对于需要地址包裹的指令，将最后一个操作数用 0() 包裹
        let operands_without_braces = operands_segment.replace("{", "").replace("}", "");

        let operands: Vec<&str> = operands_without_braces
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        let (prefix_operands, last_operand) = if operands.len() > 0 {
            let last_idx = operands.len() - 1;
            let prefix = if last_idx > 0 {
                // 为前面的操作数添加花括号
                let prefix_with_braces: Vec<String> = operands[..last_idx]
                    .iter()
                    .map(|op| format!("{{{}}}", op))
                    .collect();
                format!(" {}, ", prefix_with_braces.join(", "))
            } else {
                " ".to_string()
            };
            (prefix, operands[last_idx])
        } else {
            (" ".to_string(), "")
        };

        let escaped_prefix = prefix_operands.replace('\\', "\\\\").replace('"', "\\\"");

        let last_operand = last_operand.replace('(', "").replace(')', "");

        match (has_aq, has_rl) {
            (true, true) => format!(
                r#"{{
    let suffix = if *aq && *rl {{
        ".aqrl"
    }} else if *aq {{
        ".aq"
    }} else if *rl {{
        ".rl"
    }} else {{
        ""
    }};
    format!("{name}{{suffix}}{escaped_prefix}0({{{last_operand}}})")
}}"#
            ),
            (true, false) => format!(
                r#"{{
    let suffix = if *aq {{
        ".aq"
    }} else {{
        ""
    }};
    format!("{name}{{suffix}}{escaped_prefix}0({{{last_operand}}})")
}}"#
            ),
            (false, true) => format!(
                r#"{{
    let suffix = if *rl {{
        ".rl"
    }} else {{
        ""
    }};
    format!("{name}{{suffix}}{escaped_prefix}0({{{last_operand}}})")
}}"#
            ),
            (false, false) => format!(r#"format!("{name}{escaped_prefix}0({{{last_operand}}})"#),
        }
    } else {
        // 对于普通的aqrl指令，不需要地址包裹
        let operands_part = if operands_segment.trim().is_empty() {
            String::new()
        } else {
            format!(" {}", operands_segment)
        };

        let escaped_operands = operands_part.replace('\\', "\\\\").replace('"', "\\\"");

        match (has_aq, has_rl) {
            (true, true) => format!(
                r#"{{
    let suffix = if *aq && *rl {{
        ".aqrl"
    }} else if *aq {{
        ".aq"
    }} else if *rl {{
        ".rl"
    }} else {{
        ""
    }};
    format!("{}{{suffix}}{}", )
}}"#,
                name, escaped_operands
            ),
            (true, false) => format!(
                r#"{{
    let suffix = if *aq {{
        ".aq"
    }} else {{
        ""
    }};
    format!("{}{{suffix}}{}", )
}}"#,
                name, escaped_operands
            ),
            (false, true) => format!(
                r#"{{
    let suffix = if *rl {{
        ".rl"
    }} else {{
        ""
    }};
    format!("{}{{suffix}}{}", )
}}"#,
                name, escaped_operands
            ),
            (false, false) => format!(r#"format!("{}{}", )"#, name, escaped_operands),
        }
    }
}

/// 检查是否为MOP指令
fn is_mop_instruction(instruction: &Instruction) -> bool {
    (instruction.name.starts_with("mop.") || instruction.name.starts_with("c.mop"))
        && instruction.operands.iter().any(|o| o.name == "n")
}

/// 生成MOP指令的汇编代码
fn generate_mop_assembly_code(
    name: &str,
    operands_segment: &str,
    _instruction: &Instruction,
) -> String {
    let operands_part = if operands_segment.trim().is_empty() {
        String::new()
    } else {
        format!(" {}", operands_segment)
    };

    let escaped_operands = operands_part.replace('\\', "\\\\").replace('"', "\\\"");

    // 提取指令名称前缀（去掉末尾的.n）
    let name_prefix = if name.ends_with(".n") {
        &name[..name.len() - 2]
    } else {
        name
    };

    format!(r#"format!("{}.{{}}{}", n)"#, name_prefix, escaped_operands)
}

/// 检查是否为包含 vm 操作数的向量指令
fn is_vector_instruction_with_vm(instruction: &Instruction) -> bool {
    instruction.name.starts_with('v') && instruction.operands.iter().any(|o| o.name == "vm")
}

/// 生成包含 vm 操作数的向量指令的汇编代码
fn generate_vector_vm_assembly_code(
    name: &str,
    operands_segment: &str,
    _instruction: &Instruction,
) -> String {
    // 从操作数段中移除 {vm} 部分
    let operands_without_vm = operands_segment
        .replace(", {vm}", "")
        .replace("{vm}, ", "")
        .replace("{vm}", "");

    let operands_part = if operands_without_vm.trim().is_empty() {
        String::new()
    } else {
        format!(" {}", operands_without_vm)
    };

    let escaped_operands = operands_part.replace('\\', "\\\\").replace('"', "\\\"");

    format!(
        r#"{{
    let vm_suffix = if *vm {{
        ""
    }} else {{
        ", v0.t"
    }};
    format!("{}{}{{vm_suffix}}")
}}"#,
        name, escaped_operands
    )
}

/// 为指令的每个操作数分配正确的operand_type
fn assign_operand_types(instruction: &mut Instruction) {
    for operand in instruction.operands.iter_mut() {
        if operand.operand_type.is_none() {
            operand.operand_type = Some(determine_operand_type(&operand.name));
        }
    }
}

/// 根据操作数名称确定操作数类型
fn determine_operand_type(operand_name: &str) -> OperandType {
    match operand_name {
        // 1. 明确的寄存器类型
        "xd" | "xs1" | "xs2" | "rd" | "rs1" | "rs2" | "rs3" => OperandType::IntegerRegister,
        "fd" | "fs1" | "fs2" | "fs3" | "qd" | "qs1" | "qs2" | "qs3" | "hd" | "dd" => {
            OperandType::FloatingPointRegister
        }
        "vd" | "vs1" | "vs2" | "vs3" => OperandType::VectorRegister,

        // 2. 新增的特殊格式化类型
        "csr" => OperandType::CSRAddress,
        "rm" | "frm" => OperandType::RoundMode,

        // 3. 明确的立即数类型
        // 无符号整数立即数 (移除了 "csr", "rm", "frm")
        "uimm" | "shamt" | "n" | "rnum" | "fm" | "vtypei" | "vm" | "aq" | "rl" | "bs" => {
            OperandType::UnsignedInteger
        }

        // 有符号整数立即数
        "imm" | "imm12" | "offset" => OperandType::SignedInteger,

        // FenceMode
        "pred" | "succ" => OperandType::FenceMode,

        "r1s" | "r2s" => OperandType::SavedIntegerRegister,
        "ne_r1s_r2s" => OperandType::NotEqualCompressedSavedIntegerRegisterPair, // 新增

        // 4. 根据名称模式进行推断 (作为备用逻辑)
        _ => {
            if operand_name.starts_with('x') {
                OperandType::IntegerRegister
            } else if operand_name.starts_with('f')
                || operand_name.starts_with('q')
                || operand_name.starts_with('h')
            {
                OperandType::FloatingPointRegister
            } else if operand_name.starts_with('v') {
                OperandType::VectorRegister
            } else if operand_name.contains("imm") || operand_name.contains("offset") {
                OperandType::SignedInteger
            } else {
                // 默认回退到无符号整数
                OperandType::UnsignedInteger
            }
        }
    }
}

/// 修复超级用户模式加载/存储指令的地址语法
fn fix_hypervisor_load_store_instructions(
    operands_part: &mut String,
    instruction: &mut Instruction,
) {
    match instruction.name.as_str() {
        // H-Load 指令：从 "{xd}, {xs1}" 转换为 "{xd}, 0({xs1})"
        "hlv.b" | "hlv.bu" | "hlv.d" | "hlv.h" | "hlv.hu" | "hlv.w" | "hlv.wu" | "hlvx.hu"
        | "hlvx.wu" => {
            *operands_part = "xd, 0(xs1)".to_string();
        }
        // H-Store 指令：从 "{xs1}, {xs2}" 转换为 "{xs2}, 0({xs1})"
        "hsv.b" | "hsv.d" | "hsv.h" | "hsv.w" => {
            *operands_part = "xs2, 0(xs1)".to_string();
        }
        _ => {}
    }
}

/// 修复 ISABase 错误的指令
fn fix_isabase_error_instructions(instruction: &mut Instruction) {
    // 仅RV64
    match instruction.name.as_str() {
        "ssamoswap.d" => {
            instruction.isa_bases = vec![ISABase::RV64];
            instruction.operands.iter_mut().for_each(|op| {
                op.bit_lengths.remove(&ISABase::RV32);
            });
        }
        _ => {}
    }

    // 仅RV32
    match instruction.name.as_str() {
        "ssamoswap.w" => {
            instruction.isa_bases = vec![ISABase::RV32];
            instruction.operands.iter_mut().for_each(|op| {
                op.bit_lengths.remove(&ISABase::RV64);
            });
        }
        _ => {}
    }
}

/// 修复 aes64ks1i 指令的 rnum 操作数范围限制
fn fix_aes64ks1i_rnum_range(instruction: &mut Instruction) {
    if instruction.name != "aes64ks1i" {
        return;
    }

    // 为 rnum 操作数添加范围限制 [0, 10]
    instruction
        .operands
        .iter_mut()
        .find(|op| op.name == "rnum")
        .map(|op| {
            op.restrictions = Some(OperandRestriction {
                multiple_of: None,
                min_max: Some((0, 10)),
                forbidden_values: vec![],
                odd_only: None,
            });
        });
}

/// 生成压缩指令偏移量的汇编代码
fn generate_compressed_offset_assembly_code(
    name: &str,
    operands_segment: &str,
    _instruction: &Instruction,
) -> String {
    // 移除花括号以获得操作数
    let operands_without_braces = operands_segment.replace("{", "").replace("}", "");

    let operands: Vec<&str> = operands_without_braces
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();

    match name {
        "c.sh" => {
            // c.sh {xs2}, {uimm}({xs1}) -> c.sh {xs2}, offset({xs1})
            if operands.len() >= 3 {
                let xs2 = operands[0];
                let xs1 = operands[2]; // 最后一个操作数是基址寄存器
                format!(
                    r#"{{
    let offset = if *uimm {{
        "2"
    }} else {{
        "0"
    }};
    format!("{name} {{{xs2}}}, {{offset}}({{{xs1}}})")
}}"#
                )
            } else {
                format!(r#"format!("{name} {{xs2}}, 0({{xs1}})")"#)
            }
        }
        "c.lh" | "c.lhu" => {
            // c.lh {xd}, {uimm}({xs1}) -> c.lh {xd}, offset({xs1})
            if operands.len() >= 3 {
                let xd = operands[0];
                let xs1 = operands[2]; // 最后一个操作数是基址寄存器
                format!(
                    r#"{{
    let offset = if *uimm {{
        "2"
    }} else {{
        "0"
    }};
    format!("{name} {{{xd}}}, {{offset}}({{{xs1}}})")
}}"#
                )
            } else {
                format!(r#"format!("{name} {{xd}}, 0({{xs1}})")"#)
            }
        }
        _ => unreachable!(),
    }
}

/// 修复指令的 reg_list 和 stack_adj 合并问题
fn fix_cm_instructions_reg_list_stack_adj(
    instruction: &mut Instruction,
    operands_part: &mut String,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if !matches!(
        instruction.name.as_str(),
        "cm.pop" | "cm.push" | "cm.popretz" | "cm.popret"
    ) {
        return;
    }

    // 检查是否同时包含 reg_list 和 stack_adj
    let has_reg_list = instruction.operands.iter().any(|op| op.name == "reg_list");
    let has_stack_adj = instruction.operands.iter().any(|op| op.name == "stack_adj");

    if has_reg_list && has_stack_adj {
        // 获取原始操作数的位长度和限制
        let reg_list_operand = instruction
            .operands
            .iter()
            .find(|op| op.name == "reg_list")
            .cloned();
        let stack_adj_operand = instruction
            .operands
            .iter()
            .find(|op| op.name == "stack_adj")
            .cloned();

        // 创建合并后的操作数
        if let (Some(reg_list), Some(stack_adj)) = (reg_list_operand, stack_adj_operand) {
            let mut combined_bit_lengths = HashMap::new();
            // 合并位长度：reg_list (4位) + stack_adj (2位) = 6位
            for (&isa_base, &reg_bits) in &reg_list.bit_lengths {
                if let Some(&stack_bits) = stack_adj.bit_lengths.get(&isa_base) {
                    combined_bit_lengths.insert(isa_base, reg_bits + stack_bits);
                }
            }

            let combined_operand = Operand {
                name: "saved_reg_list_with_stack_adj".to_string(),
                operand_type: Some(OperandType::SavedRegListWithStackAdj),
                bit_lengths: combined_bit_lengths,
                restrictions: None, // 复合操作数的限制由各个组成部分内部处理
            };

            // 替换操作数列表
            instruction.operands = vec![combined_operand];

            // 更新语法操作数
            final_syntax_operands.clear();
            final_syntax_operands.insert("saved_reg_list_with_stack_adj".to_string());

            // 清空操作数部分，因为会用 Rust 代码生成
            *operands_part = String::new();
        }
    }
}

/// 生成 cm.* 指令的汇编代码
fn generate_cm_instruction_assembly_code(name: &str) -> String {
    match name {
        "cm.pop" => r#"{
    let reg_list_str = saved_reg_list_with_stack_adj.get_saved_reg_list_string();
    let stack_adj = saved_reg_list_with_stack_adj.get_stack_adjustment();
    format!("cm.pop {}, {}", reg_list_str, stack_adj)
}"#
        .to_string(),
        "cm.push" => r#"{
    let reg_list_str = saved_reg_list_with_stack_adj.get_saved_reg_list_string();
    let stack_adj = saved_reg_list_with_stack_adj.get_stack_adjustment();
    format!("cm.push {}, -{}", reg_list_str, stack_adj)
}"#
        .to_string(),
        "cm.popretz" => r#"{
    let reg_list_str = saved_reg_list_with_stack_adj.get_saved_reg_list_string();
    let stack_adj = saved_reg_list_with_stack_adj.get_stack_adjustment();
    format!("cm.popretz {}, {}", reg_list_str, stack_adj)
}"#
        .to_string(),
        "cm.popret" => r#"{
    let reg_list_str = saved_reg_list_with_stack_adj.get_saved_reg_list_string();
    let stack_adj = saved_reg_list_with_stack_adj.get_stack_adjustment();
    format!("cm.popret {}, {}", reg_list_str, stack_adj)
}"#
        .to_string(),
        _ => unreachable!(),
    }
}

/// 修复 cm.mvsa01 指令的操作数 r1s 和 r2s 合并问题
fn fix_cm_mvsa01_instruction(
    instruction: &mut Instruction,
    operands_part: &mut String,
    final_syntax_operands: &mut std::collections::HashSet<String>,
) {
    if instruction.name != "cm.mvsa01" {
        return;
    }

    let has_r1s = instruction.operands.iter().any(|op| op.name == "r1s");
    let has_r2s = instruction.operands.iter().any(|op| op.name == "r2s");

    if has_r1s && has_r2s {
        // 假设 r1s 和 r2s 都是 3 位
        let mut combined_bit_lengths = HashMap::new();
        // 查找 r1s 和 r2s 的原始位长信息
        let r1s_op = instruction.operands.iter().find(|op| op.name == "r1s");
        let r2s_op = instruction.operands.iter().find(|op| op.name == "r2s");

        if let (Some(op1), Some(op2)) = (r1s_op, r2s_op) {
            for (base, len1) in &op1.bit_lengths {
                if let Some(len2) = op2.bit_lengths.get(base) {
                    combined_bit_lengths.insert(*base, len1 + len2); // 合并长度
                }
            }
        } else {
            // 如果找不到原始操作数，则使用默认值或报错
            // 为简化，这里使用默认值，实际应用中可能需要更健壮的处理
            combined_bit_lengths.insert(ISABase::RV32, 6);
            combined_bit_lengths.insert(ISABase::RV64, 6);
        }

        let merged_operand = Operand {
            name: "ne_r1s_r2s".to_string(),
            operand_type: Some(OperandType::NotEqualCompressedSavedIntegerRegisterPair),
            bit_lengths: combined_bit_lengths,
            restrictions: None, // "不相等"的约束由类型本身处理
        };

        // 移除旧操作数并添加新操作数
        instruction
            .operands
            .retain(|op| op.name != "r1s" && op.name != "r2s");
        instruction.operands.push(merged_operand);

        // 更新汇编语法部分
        // 假设原始语法是 "r1s, r2s" 或类似的，替换为新的操作数名
        // 这是一个简化的替换，实际可能需要更复杂的逻辑来处理不同的原始格式
        if operands_part.contains("r1s") && operands_part.contains("r2s") {
            *operands_part = operands_part
                .replace("r1s, r2s", "ne_r1s_r2s")
                .replace("r1s,r2s", "ne_r1s_r2s"); // 处理无空格情况
        } else {
            // 如果原始格式未知或不匹配，则直接设置为新操作数名
            *operands_part = "ne_r1s_r2s".to_string();
        }

        // 更新最终语法操作数集合
        final_syntax_operands.remove("r1s");
        final_syntax_operands.remove("r2s");
        final_syntax_operands.insert("ne_r1s_r2s".to_string());
    }
}
