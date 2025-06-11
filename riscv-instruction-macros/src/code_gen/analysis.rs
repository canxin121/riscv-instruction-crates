use super::{CodeGenerator, InstructionVariant};
use proc_macro2::TokenStream;
use riscv_instruction_parser::types::{ISABase, ISAExtension, Instruction, Operand};
use std::collections::{HashMap, HashSet};

impl CodeGenerator {
    /// 为分析模式生成所有受限类型定义
    pub fn generate_all_restricted_type_definitions_for_analysis(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> (TokenStream, TokenStream) {
        let mut restricted_register_defs = TokenStream::new();
        let mut restricted_immediate_defs = TokenStream::new();
        let mut processed_register_combinations = HashSet::new();
        let mut processed_immediate_combinations = HashSet::new();

        for variants in analysis.values() {
            for variant in variants {
                if variant.instruction.operands.is_empty() || variant.isa_bases.is_empty() {
                    continue;
                }
                // 使用第一个ISA基础作为共享/分析指令的代表
                let isa_base = &variant.isa_bases[0];

                for operand in &variant.instruction.operands {
                    self.process_operand_for_restricted_type_defs(
                        operand,
                        isa_base,
                        &mut processed_register_combinations,
                        &mut processed_immediate_combinations,
                        &mut restricted_register_defs,
                        &mut restricted_immediate_defs,
                    );
                }
            }
        }
        (restricted_register_defs, restricted_immediate_defs)
    }

    /// 为分离模式生成所有受限类型定义
    pub fn generate_all_restricted_type_definitions_for_separated(
        &self,
        instructions: &[Instruction],
    ) -> (TokenStream, TokenStream) {
        let mut restricted_register_defs = TokenStream::new();
        let mut restricted_immediate_defs = TokenStream::new();
        let mut processed_register_combinations = HashSet::new();
        let mut processed_immediate_combinations = HashSet::new();

        for instruction in instructions {
            for operand in &instruction.operands {
                // 对于分离的枚举，我们必须考虑指令支持的每个ISA基础
                for isa_base in &instruction.isa_bases {
                    self.process_operand_for_restricted_type_defs(
                        operand,
                        isa_base,
                        &mut processed_register_combinations,
                        &mut processed_immediate_combinations,
                        &mut restricted_register_defs,
                        &mut restricted_immediate_defs,
                    );
                }
            }
        }
        (restricted_register_defs, restricted_immediate_defs)
    }

    /// 处理单个操作数以生成受限类型定义（如果需要）
    pub fn process_operand_for_restricted_type_defs(
        &self,
        operand: &Operand,
        isa_base: &ISABase,
        processed_register_combinations: &mut HashSet<String>,
        processed_immediate_combinations: &mut HashSet<String>,
        restricted_register_defs: &mut TokenStream,
        restricted_immediate_defs: &mut TokenStream,
    ) {
        if let Some(restriction) = &operand.restrictions {
            if let Some(operand_type) = &operand.operand_type {
                match operand_type {
                    riscv_instruction_parser::types::OperandType::IntegerRegister
                    | riscv_instruction_parser::types::OperandType::FloatingPointRegister
                    | riscv_instruction_parser::types::OperandType::VectorRegister => {
                        if !restriction.forbidden_values.is_empty()
                            || restriction.multiple_of.is_some()
                            || restriction.min_max.is_some()
                            || restriction.odd_only.unwrap_or(false)
                        {
                            let bit_length = operand.bit_lengths.get(isa_base).unwrap_or(&5); // 默认为5位
                            let base_type = self.get_register_base_type_from_operand_type(
                                operand_type,
                                *bit_length,
                            );
                            // 使用生成的类型名称作为去重键
                            let type_name_key = self.generate_restricted_register_type_name(
                                base_type,
                                &operand.name,
                                restriction,
                            );
                            if processed_register_combinations.insert(type_name_key) {
                                let type_def = self.generate_restricted_register_type_def(
                                    base_type,
                                    &operand.name,
                                    restriction,
                                );
                                restricted_register_defs.extend(type_def);
                            }
                        }
                    }
                    riscv_instruction_parser::types::OperandType::SignedInteger
                    | riscv_instruction_parser::types::OperandType::UnsignedInteger => {
                        let bit_length = operand.bit_lengths.get(isa_base).unwrap_or(&32);
                        // 使用生成的类型名称作为去重键
                        let type_name_key = self.generate_restricted_immediate_type_name(
                            operand_type,
                            &operand.name,
                            *bit_length,
                            restriction,
                        );

                        if processed_immediate_combinations.insert(type_name_key) {
                            let type_def = self.generate_restricted_immediate_type_def(
                                operand_type,
                                &operand.name,
                                *bit_length,
                                restriction,
                            );
                            restricted_immediate_defs.extend(type_def);
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    /// 分析指令在 ISA 基础之间的共享性
    pub fn analyze_instruction_sharing(
        &self,
        instructions: &[Instruction],
    ) -> HashMap<ISAExtension, Vec<InstructionVariant>> {
        let mut analysis: HashMap<ISAExtension, Vec<InstructionVariant>> = HashMap::new();
        let mut by_extension: HashMap<ISAExtension, Vec<&Instruction>> = HashMap::new();

        for inst in instructions {
            by_extension.entry(inst.extension).or_default().push(inst);
        }

        for (extension, instructions_in_ext) in by_extension {
            let mut variants_for_ext = Vec::new();
            let mut processed_names = HashSet::new();

            for &inst_ref in &instructions_in_ext {
                if processed_names.contains(&inst_ref.name) {
                    continue;
                }
                processed_names.insert(&inst_ref.name);

                let same_name_instructions: Vec<&Instruction> = instructions_in_ext
                    .iter()
                    .filter(|&&i| i.name == inst_ref.name)
                    .copied()
                    .collect();

                if same_name_instructions.len() == 1 {
                    // Single definition for this name in this extension
                    let single_inst = same_name_instructions[0];
                    let potential_share = single_inst.isa_bases.len() > 1;
                    self.add_or_split_variant(
                        single_inst,
                        single_inst.isa_bases.clone(),
                        potential_share,
                        &mut variants_for_ext,
                    );
                } else {
                    // Multiple definitions for this name (e.g., RV32 and RV64 versions)
                    let is_identical = self.are_instructions_identical(&same_name_instructions);
                    if is_identical {
                        // They are identical, try to merge them
                        let mut merged_isa_bases = Vec::new();
                        for smi_inst in &same_name_instructions {
                            for &isa_base in &smi_inst.isa_bases {
                                if !merged_isa_bases.contains(&isa_base) {
                                    merged_isa_bases.push(isa_base);
                                }
                            }
                        }
                        merged_isa_bases.sort(); // Canonical order

                        let potential_share = merged_isa_bases.len() > 1;
                        self.add_or_split_variant(
                            inst_ref, // Use inst_ref as the template for the merged instruction
                            merged_isa_bases,
                            potential_share,
                            &mut variants_for_ext,
                        );
                    } else {
                        // Not identical, add them separately
                        for &different_inst_def in &same_name_instructions {
                            let potential_share_for_diff = different_inst_def.isa_bases.len() > 1;
                            self.add_or_split_variant(
                                different_inst_def,
                                different_inst_def.isa_bases.clone(),
                                potential_share_for_diff,
                                &mut variants_for_ext,
                            );
                        }
                    }
                }
            }
            analysis.insert(extension, variants_for_ext);
        }
        analysis
    }

    /// 辅助函数，用于添加指令变体，并在必要时（例如，对于跨RV32/RV64共享且包含特定操作数的指令）进行拆分
    pub fn add_or_split_variant(
        &self,
        instruction_template: &Instruction,
        target_isa_bases: Vec<ISABase>,
        is_potentially_shared: bool,
        variants_list: &mut Vec<InstructionVariant>,
    ) {
        let has_problematic_operand = instruction_template.operands.iter().any(|op| {
            op.operand_type.as_ref().map_or(false, |opt| {
                matches!(
                    opt,
                    riscv_instruction_parser::types::OperandType::SavedRegListWithStackAdj
                )
            })
        });

        let needs_split = has_problematic_operand
            && is_potentially_shared
            && target_isa_bases.contains(&ISABase::RV32)
            && target_isa_bases.contains(&ISABase::RV64);

        if needs_split {
            // 如果目标ISA基础包含RV32，则创建RV32特定的变体
            if target_isa_bases.iter().any(|b| *b == ISABase::RV32) {
                let mut rv32_inst_details = instruction_template.clone();
                rv32_inst_details.isa_bases = vec![ISABase::RV32];
                variants_list.push(InstructionVariant {
                    instruction: rv32_inst_details,
                    isa_bases: vec![ISABase::RV32],
                    is_shared: false,
                });
            }

            // 如果目标ISA基础包含RV64，则创建RV64特定的变体
            if target_isa_bases.iter().any(|b| *b == ISABase::RV64) {
                let mut rv64_inst_details = instruction_template.clone();
                rv64_inst_details.isa_bases = vec![ISABase::RV64];
                variants_list.push(InstructionVariant {
                    instruction: rv64_inst_details,
                    isa_bases: vec![ISABase::RV64],
                    is_shared: false,
                });
            }
        } else {
            let mut inst_details = instruction_template.clone();
            inst_details.isa_bases = target_isa_bases.clone();
            variants_list.push(InstructionVariant {
                instruction: inst_details,
                isa_bases: target_isa_bases.clone(),
                is_shared: is_potentially_shared && target_isa_bases.len() > 1,
            });
        }
    }

    pub fn are_instructions_identical(&self, instructions: &[&Instruction]) -> bool {
        if instructions.len() <= 1 {
            return true;
        }
        let first = instructions[0];
        for &inst in &instructions[1..] {
            if first.operands.len() != inst.operands.len() {
                return false;
            }
            for (op1, op2) in first.operands.iter().zip(inst.operands.iter()) {
                if op1.name != op2.name {
                    return false;
                }
                for &isa_base in &[ISABase::RV32, ISABase::RV64] {
                    if op1.bit_lengths.get(&isa_base) != op2.bit_lengths.get(&isa_base) {
                        return false;
                    }
                }
            }
            if first.assembly_syntax != inst.assembly_syntax {
                return false;
            }
        }
        true
    }
}
