use riscv_instruction_parser::types::{ISABase, ISAExtension, Instruction, Operand};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::Ident;

/// 指令变体，用于分析指令在不同 ISA 基础之间的共享性
#[derive(Debug, Clone)]
struct InstructionVariant {
    instruction: Instruction,
    isa_bases: Vec<ISABase>,
    is_shared: bool,
}

/// RISC-V 指令代码生成器
pub struct CodeGenerator {
    instructions: Vec<Instruction>,
}

impl CodeGenerator {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Self { instructions }
    }

    /// 生成完整的指令枚举代码
    pub fn generate_instruction_enum(&self) -> TokenStream {
        let analysis = self.analyze_instruction_sharing(&self.instructions);

        let mut restricted_register_defs = TokenStream::new();
        let mut restricted_immediate_defs = TokenStream::new();
        let mut processed_register_combinations = std::collections::HashSet::new();
        let mut processed_immediate_combinations = std::collections::HashSet::new();

        // 收集所有需要的受限类型定义
        for variants in analysis.values() {
            for variant in variants {
                for operand in &variant.instruction.operands {
                    if let Some(restriction) = &operand.restrictions {
                        // 处理寄存器类型
                        if let Some(operand_type) = &operand.operand_type {
                            match operand_type {
                                riscv_instruction_parser::types::OperandType::IntegerRegister
                                | riscv_instruction_parser::types::OperandType::FloatingPointRegister
                                | riscv_instruction_parser::types::OperandType::VectorRegister => {
                                    // 检查是否有任何限制条件
                                    if !restriction.forbidden_values.is_empty() 
                                        || restriction.multiple_of.is_some()
                                        || restriction.min_max.is_some()
                                        || restriction.odd_only.unwrap_or(false) {
                                        let isa_base = &variant.isa_bases[0]; // 使用第一个ISA基础
                                        let bit_length =
                                            operand.bit_lengths.get(isa_base).unwrap_or(&5); // 默认为5位
                                        let base_type = self
                                            .get_register_base_type_from_operand_type(
                                                operand_type,
                                                *bit_length,
                                            );
                                        let type_signature = self.create_register_type_signature(
                                            base_type,
                                            &operand.name,
                                            restriction,
                                        );
                                        if processed_register_combinations
                                            .insert(type_signature.clone())
                                        {
                                            let type_def = self
                                                .generate_restricted_register_type_def(
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
                                    let isa_base = &variant.isa_bases[0]; // 使用第一个ISA基础
                                    let bit_length =
                                        operand.bit_lengths.get(isa_base).unwrap_or(&32);
                                    let type_signature = self.create_immediate_type_signature(
                                        operand_type,
                                        &operand.name,
                                        *bit_length,
                                        restriction,
                                    );

                                    if processed_immediate_combinations
                                        .insert(type_signature.clone())
                                    {
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
            }
        }

        let imports = self.generate_imports();
        let instruction_enums = self.generate_instruction_enums(&analysis);
        let main_enum = self.generate_main_enum(&analysis);

        quote! {
            // This file is auto-generated by the RISC-V instruction code generator.
            // Author: canxin121 <q1969730106@gmail.com>
            // Do not edit this file manually.
            #imports

            #restricted_register_defs

            #restricted_immediate_defs

            #instruction_enums

            #main_enum
        }
    }

    /// 生成完全分离的指令枚举代码（不合并共享指令，完全按扩展和ISA基础分开）
    pub fn generate_separated_instruction_enum(&self) -> TokenStream {
        let mut restricted_register_defs = TokenStream::new();
        let mut restricted_immediate_defs = TokenStream::new();
        let mut processed_register_combinations = std::collections::HashSet::new();
        let mut processed_immediate_combinations = std::collections::HashSet::new();

        // 收集所有需要的受限类型定义
        for instruction in &self.instructions {
            for operand in &instruction.operands {
                if let Some(restriction) = &operand.restrictions {
                    // 处理寄存器类型
                    if let Some(operand_type) = &operand.operand_type {
                        match operand_type {
                            riscv_instruction_parser::types::OperandType::IntegerRegister
                            | riscv_instruction_parser::types::OperandType::FloatingPointRegister
                            | riscv_instruction_parser::types::OperandType::VectorRegister => {
                                // 检查是否有任何限制条件
                                if !restriction.forbidden_values.is_empty() 
                                    || restriction.multiple_of.is_some()
                                    || restriction.min_max.is_some()
                                    || restriction.odd_only.unwrap_or(false) {
                                    // 对每个ISA基础都处理
                                    for isa_base in &instruction.isa_bases {
                                        let bit_length =
                                            operand.bit_lengths.get(isa_base).unwrap_or(&5);
                                        let base_type = self
                                            .get_register_base_type_from_operand_type(
                                                operand_type,
                                                *bit_length,
                                            );
                                        let type_signature = self.create_register_type_signature(
                                            base_type,
                                            &operand.name,
                                            restriction,
                                        );
                                        if processed_register_combinations
                                            .insert(type_signature.clone())
                                        {
                                            let type_def = self
                                                .generate_restricted_register_type_def(
                                                    base_type,
                                                    &operand.name,
                                                    restriction,
                                                );
                                            restricted_register_defs.extend(type_def);
                                        }
                                    }
                                }
                            }
                            riscv_instruction_parser::types::OperandType::SignedInteger
                            | riscv_instruction_parser::types::OperandType::UnsignedInteger => {
                                // 对每个ISA基础都处理
                                for isa_base in &instruction.isa_bases {
                                    let bit_length =
                                        operand.bit_lengths.get(isa_base).unwrap_or(&32);
                                    let type_signature = self.create_immediate_type_signature(
                                        operand_type,
                                        &operand.name,
                                        *bit_length,
                                        restriction,
                                    );

                                    if processed_immediate_combinations
                                        .insert(type_signature.clone())
                                    {
                                        let type_def = self.generate_restricted_immediate_type_def(
                                            operand_type,
                                            &operand.name,
                                            *bit_length,
                                            restriction,
                                        );
                                        restricted_immediate_defs.extend(type_def);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        let imports = self.generate_imports();
        let separated_enums = self.generate_separated_enums();
        let main_enum = self.generate_separated_main_enum();

        quote! {
            // This file is auto-generated by the RISC-V instruction code generator.
            // Author: canxin121 <q1969730106@gmail.com>
            // Do not edit this file manually.
            #imports

            #restricted_register_defs

            #restricted_immediate_defs

            #separated_enums

            #main_enum
        }
    }

    /// 分析指令在 ISA 基础之间的共享性
    fn analyze_instruction_sharing(
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
    fn add_or_split_variant(
        &self,
        instruction_template: &Instruction,
        target_isa_bases: Vec<ISABase>,
        is_potentially_shared: bool,
        variants_list: &mut Vec<InstructionVariant>,
    ) {
        let has_problematic_operand =
            instruction_template.operands.iter().any(|op| {
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

    fn are_instructions_identical(&self, instructions: &[&Instruction]) -> bool {
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

    /// 生成指令枚举
    fn generate_instruction_enums(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();

        let shared_enums = self.generate_shared_instructions_enum(analysis);
        all_enums.extend(shared_enums);

        let specific_enums = self.generate_isa_specific_instructions_enum(analysis);
        all_enums.extend(specific_enums);

        let main_enums = self.generate_extension_main_enum(analysis);
        all_enums.extend(main_enums);

        all_enums
    }

    /// 生成共享指令枚举
    fn generate_shared_instructions_enum(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();
        for (extension, variants) in analysis {
            let shared_variants: Vec<_> = variants.iter().filter(|v| v.is_shared).collect();
            if shared_variants.is_empty() {
                continue;
            }

            let enum_name = Ident::new(
                &format!("{}SharedInstructions", extension),
                Span::call_site(),
            );
            let doc_comment = format!("Shared instructions for {} extension", extension);

            let variant_tokens = self.build_variants(&shared_variants, None);

            all_enums.extend(quote! {
                #[doc = #doc_comment]
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                #[rustfmt::skip]
                pub enum #enum_name {
                    #(#variant_tokens),*
                }
            });
        }
        all_enums
    }

    /// 生成 ISA 特定指令枚举
    fn generate_isa_specific_instructions_enum(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();
        for isa_base in &[ISABase::RV32, ISABase::RV64] {
            for (extension, variants) in analysis {
                let isa_specific_variants: Vec<_> = variants
                    .iter()
                    .filter(|v| !v.is_shared && v.isa_bases.contains(isa_base))
                    .collect();

                if isa_specific_variants.is_empty() {
                    continue;
                }

                let enum_name = Ident::new(
                    &format!("{}{}SpecificInstructions", isa_base, extension),
                    Span::call_site(),
                );
                let doc_comment = format!(
                    "{} specific instructions for {} extension",
                    isa_base, extension
                );

                let variant_tokens = self.build_variants(&isa_specific_variants, Some(isa_base));

                all_enums.extend(quote! {
                    #[doc = #doc_comment]
                    #[allow(non_camel_case_types)]
                    #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                    #[rustfmt::skip]
                    pub enum #enum_name {
                        #(#variant_tokens),*
                    }
                });
            }
        }
        all_enums
    }

    /// 生成扩展主枚举
    fn generate_extension_main_enum(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut main_enums = TokenStream::new();

        // 生成共享指令枚举
        let shared_variants = analysis
            .iter()
            .filter_map(|(ext, vars)| {
                if vars.iter().any(|v| v.is_shared) {
                    let ext_ident = Ident::new(&ext.to_string(), Span::call_site());
                    let enum_name =
                        Ident::new(&format!("{}SharedInstructions", ext), Span::call_site());
                    Some(quote! { #ext_ident(#enum_name) })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !shared_variants.is_empty() {
            let shared_enum_name = Ident::new("SharedInstruction", Span::call_site());
            let shared_doc = "Instructions shared across all ISA bases, grouped by extension.";
            main_enums.extend(quote! {
                #[doc = #shared_doc]
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum #shared_enum_name {
                    #(#shared_variants),*
                }
            });
        }

        // 生成 ISA 特定枚举
        for isa_base in &[ISABase::RV32, ISABase::RV64] {
            let isa_base_str = isa_base.to_string();
            let specific_enum_name = Ident::new(
                &format!("{}SpecificInstruction", isa_base),
                Span::call_site(),
            );
            let doc_comment = format!(
                "{} specific instructions, grouped by extension.",
                isa_base_str
            );

            let extension_variants = analysis
                .iter()
                .filter_map(|(ext, vars)| {
                    if vars
                        .iter()
                        .any(|v| !v.is_shared && v.isa_bases.contains(isa_base))
                    {
                        let ext_ident = Ident::new(&ext.to_string(), Span::call_site());
                        let enum_name = Ident::new(
                            &format!("{}{}SpecificInstructions", isa_base, ext),
                            Span::call_site(),
                        );
                        Some(quote! { #ext_ident(#enum_name) })
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !extension_variants.is_empty() {
                main_enums.extend(quote! {
                    #[doc = #doc_comment]
                    #[allow(non_camel_case_types)]
                    #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                    pub enum #specific_enum_name {
                        #(#extension_variants),*
                    }
                });
            }
        }

        // 生成特定指令枚举
        let specific_variants = [ISABase::RV32, ISABase::RV64]
            .iter()
            .filter_map(|isa_base| {
                let has_specific = analysis.values().any(|vars| {
                    vars.iter()
                        .any(|v| !v.is_shared && v.isa_bases.contains(isa_base))
                });
                if has_specific {
                    let isa_base_ident = Ident::new(&isa_base.to_string(), Span::call_site());
                    let enum_name = Ident::new(
                        &format!("{}SpecificInstruction", isa_base),
                        Span::call_site(),
                    );
                    Some(quote! { #isa_base_ident(#enum_name) })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !specific_variants.is_empty() {
            let specific_enum_name = Ident::new("SpecificInstruction", Span::call_site());
            let specific_doc = "ISA base specific instructions.";
            main_enums.extend(quote! {
                #[doc = #specific_doc]
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum #specific_enum_name {
                    #(#specific_variants),*
                }
            });
        }

        main_enums
    }

    /// 生成主指令枚举
    fn generate_main_enum(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut main_variants = Vec::new();

        // 检查是否有共享指令
        let has_shared = analysis
            .values()
            .any(|vars| vars.iter().any(|v| v.is_shared));
        if has_shared {
            main_variants.push(quote! {
                #[doc = "Instructions shared across ISA bases"]
                Shared(SharedInstruction)
            });
        }

        // 检查是否有特定指令
        let has_specific = analysis
            .values()
            .any(|vars| vars.iter().any(|v| !v.is_shared));
        if has_specific {
            main_variants.push(quote! {
                #[doc = "ISA base specific instructions"]
                Specific(SpecificInstruction)
            });
        }

        if !main_variants.is_empty() {
            quote! {
                /// Main RISC-V instruction enum, dispatching to shared or specific instructions.
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum RiscvInstruction {
                    #(#main_variants),*
                }
            }
        } else {
            quote! {
                /// Main RISC-V instruction enum.
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum RiscvInstruction {}
            }
        }
    }

    /// 生成完全分离的指令枚举（按扩展和ISA基础完全分开）
    fn generate_separated_enums(&self) -> TokenStream {
        let mut all_enums = TokenStream::new();
        let mut by_extension_and_isa: HashMap<(ISAExtension, ISABase), Vec<&Instruction>> =
            HashMap::new();

        // 按扩展和ISA基础分组
        for inst in &self.instructions {
            for &isa_base in &inst.isa_bases {
                by_extension_and_isa
                    .entry((inst.extension, isa_base))
                    .or_default()
                    .push(inst);
            }
        }

        // 为每个组合生成枚举
        for ((extension, isa_base), instructions) in by_extension_and_isa {
            let enum_name = Ident::new(
                &format!("{}{}Instructions", isa_base, extension),
                Span::call_site(),
            );
            let doc_comment = format!("{} {} instructions", isa_base, extension);

            let variants = instructions
                .iter()
                .map(|&inst| {
                    let variant_name = Ident::new(
                        &self.instruction_name_to_variant(&inst.name),
                        Span::call_site(),
                    );

                    let attribute_token = match &inst.assembly_syntax {
                        riscv_instruction_parser::types::AssemblySyntax::RustCode(code) => {
                            let code_str = code.as_str();
                            quote! { #[asm_code(#code_str)] }
                        }
                        riscv_instruction_parser::types::AssemblySyntax::Format(format_str) => {
                            quote! { #[asm(#format_str)] }
                        }
                    };

                    if inst.operands.is_empty() {
                        quote! {
                            #attribute_token
                            #variant_name
                        }
                    } else {
                        let operands = inst
                            .operands
                            .iter()
                            .map(|op| {
                                let op_name = Ident::new(&op.name, Span::call_site());
                                let op_type_str = self.operand_to_typed_struct(op, &isa_base);
                                let op_type: TokenStream = op_type_str
                                    .parse()
                                    .expect("Failed to parse operand type string");
                                quote! { #op_name: #op_type }
                            })
                            .collect::<Vec<_>>();

                        quote! {
                            #attribute_token
                            #variant_name { #(#operands),* }
                        }
                    }
                })
                .collect::<Vec<_>>();

            all_enums.extend(quote! {
                #[doc = #doc_comment]
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                #[rustfmt::skip]
                pub enum #enum_name {
                    #(#variants),*
                }
            });
        }

        all_enums
    }

    /// 生成分离版本的主枚举
    fn generate_separated_main_enum(&self) -> TokenStream {
        let mut by_extension_and_isa: HashMap<(ISAExtension, ISABase), Vec<&Instruction>> =
            HashMap::new();

        // 按扩展和ISA基础分组
        for inst in &self.instructions {
            for &isa_base in &inst.isa_bases {
                by_extension_and_isa
                    .entry((inst.extension, isa_base))
                    .or_default()
                    .push(inst);
            }
        }

        // 按ISA基础分组生成枚举
        let mut isa_enums = Vec::new();
        for isa_base in &[ISABase::RV32, ISABase::RV64] {
            let extensions_for_isa: Vec<_> = by_extension_and_isa
                .keys()
                .filter(|(_, base)| base == isa_base)
                .map(|(ext, _)| *ext)
                .collect::<HashSet<_>>()
                .into_iter()
                .collect();

            if !extensions_for_isa.is_empty() {
                let isa_enum_name =
                    Ident::new(&format!("{}Instruction", isa_base), Span::call_site());
                let isa_doc = format!("{} instructions grouped by extension", isa_base);

                let extension_variants = extensions_for_isa
                    .iter()
                    .map(|&ext| {
                        let ext_ident = Ident::new(&ext.to_string(), Span::call_site());
                        let enum_name = Ident::new(
                            &format!("{}{}Instructions", isa_base, ext),
                            Span::call_site(),
                        );
                        quote! { #ext_ident(#enum_name) }
                    })
                    .collect::<Vec<_>>();

                isa_enums.push(quote! {
                    #[doc = #isa_doc]
                    #[allow(non_camel_case_types)]
                    #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                    pub enum #isa_enum_name {
                        #(#extension_variants),*
                    }
                });
            }
        }

        // 生成最终的主枚举
        let main_variants = [ISABase::RV32, ISABase::RV64]
            .iter()
            .filter_map(|&isa_base| {
                let has_instructions = by_extension_and_isa
                    .keys()
                    .any(|(_, base)| *base == isa_base);
                if has_instructions {
                    let isa_ident = Ident::new(&isa_base.to_string(), Span::call_site());
                    let enum_name =
                        Ident::new(&format!("{}Instruction", isa_base), Span::call_site());
                    Some(quote! { #isa_ident(#enum_name) })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut result = TokenStream::new();
        result.extend(isa_enums.into_iter());

        if !main_variants.is_empty() {
            result.extend(quote! {
                /// Main RISC-V instruction enum, separated by ISA base.
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum RiscvInstruction {
                    #(#main_variants),*
                }
            });
        } else {
            result.extend(quote! {
                /// Main RISC-V instruction enum.
                #[allow(non_camel_case_types)]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum RiscvInstruction {}
            });
        }

        result
    }

    /// 构建枚举变体
    fn build_variants(
        &self,
        variants: &[&InstructionVariant],
        isa_base_override: Option<&ISABase>,
    ) -> Vec<TokenStream> {
        variants
            .iter()
            .map(|variant| {
                let variant_name = Ident::new(
                    &self.instruction_name_to_variant(&variant.instruction.name),
                    Span::call_site(),
                );

                let attribute_token = match &variant.instruction.assembly_syntax {
                    riscv_instruction_parser::types::AssemblySyntax::RustCode(code) => {
                        let code_str = code.as_str();
                        quote! { #[asm_code(#code_str)] }
                    }
                    riscv_instruction_parser::types::AssemblySyntax::Format(format_str) => {
                        quote! { #[asm(#format_str)] }
                    }
                };

                if variant.instruction.operands.is_empty() {
                    quote! {
                        #attribute_token
                        #variant_name
                    }
                } else {
                    let operands = variant
                        .instruction
                        .operands
                        .iter()
                        .map(|op| {
                            let op_name = Ident::new(&op.name, Span::call_site());
                            let isa_base =
                                isa_base_override.unwrap_or_else(|| &variant.isa_bases[0]);
                            let op_type_str = self.operand_to_typed_struct(op, isa_base);
                            let op_type: TokenStream = op_type_str
                                .parse()
                                .expect("Failed to parse operand type string");
                            quote! { #op_name: #op_type }
                        })
                        .collect::<Vec<_>>();

                    quote! {
                        #attribute_token
                        #variant_name { #(#operands),* }
                    }
                }
            })
            .collect()
    }

    /// 生成导入语句
    fn generate_imports(&self) -> TokenStream {
        quote! {
            use std::fmt::{self, Display};
            pub use riscv_instruction_types::*;
            use riscv_instruction_macros::{DeriveValidatedValue, DeriveInstructionDisplay, DeriveRandom};
        }
    }

    fn instruction_name_to_variant(&self, name: &str) -> String {
        name.replace('.', "_").replace('-', "_").to_uppercase()
    }

    fn operand_to_typed_struct(&self, operand: &Operand, isa_base: &ISABase) -> String {
        let bit_length = operand.bit_lengths.get(isa_base).unwrap_or_else(|| {
            panic!(
                "No bit length defined for operand '{}' in ISA base '{}'",
                operand.name, isa_base
            )
        });
        let restrictions = operand.restrictions.as_ref();

        // 必须有 operand_type 字段
        let operand_type = operand
            .operand_type
            .as_ref()
            .unwrap_or_else(|| panic!("No operand_type defined for operand '{}'", operand.name));

        match operand_type {
            riscv_instruction_parser::types::OperandType::IntegerRegister => {
                                        let base_type =
                                            self.get_register_base_type_from_operand_type(operand_type, *bit_length);

                                        if let Some(restriction) = restrictions {
                                            if !restriction.forbidden_values.is_empty() 
                                                || restriction.multiple_of.is_some()
                                                || restriction.min_max.is_some()
                                                || restriction.odd_only.unwrap_or(false) {
                                                return self.generate_restricted_register_type_name(
                                                    base_type,
                                                    &operand.name,
                                                    restriction,
                                                );
                                            }
                                        }
                                        base_type.to_string()
                                    }
            riscv_instruction_parser::types::OperandType::FloatingPointRegister => {
                                        let base_type =
                                            self.get_register_base_type_from_operand_type(operand_type, *bit_length);

                                        if let Some(restriction) = restrictions {
                                            if !restriction.forbidden_values.is_empty() 
                                                || restriction.multiple_of.is_some()
                                                || restriction.min_max.is_some()
                                                || restriction.odd_only.unwrap_or(false) {
                                                return self.generate_restricted_register_type_name(
                                                    base_type,
                                                    &operand.name,
                                                    restriction,
                                                );
                                            }
                                        }
                                        base_type.to_string()
                                    }
            riscv_instruction_parser::types::OperandType::VectorRegister => {
                                        if let Some(restriction) = restrictions {
                                            if !restriction.forbidden_values.is_empty() 
                                                || restriction.multiple_of.is_some()
                                                || restriction.min_max.is_some()
                                                || restriction.odd_only.unwrap_or(false) {
                                                return self.generate_restricted_register_type_name(
                                                    "VectorRegister",
                                                    &operand.name,
                                                    restriction,
                                                );
                                            }
                                        }
                                        "VectorRegister".to_string()
                                    }
            riscv_instruction_parser::types::OperandType::CSRAddress => "CSRAddress".to_string(),
            riscv_instruction_parser::types::OperandType::RoundMode => "RoundingMode".to_string(),
            riscv_instruction_parser::types::OperandType::FenceMode => "FenceMode".to_string(),
            riscv_instruction_parser::types::OperandType::SignedInteger => {
                                        if let Some(restriction) = restrictions {
                                            if restriction.min_max.is_some()
                                                || restriction.multiple_of.is_some()
                                                || !restriction.forbidden_values.is_empty()
                                                || restriction.odd_only.unwrap_or(false)
                                            {
                                                return self.generate_restricted_immediate_type_name(
                                                    operand_type,
                                                    &operand.name,
                                                    *bit_length,
                                                    restriction,
                                                );
                                            }
                                        }
                                        format!("Immediate<{}>", bit_length)
                                    }
            riscv_instruction_parser::types::OperandType::UnsignedInteger => {
                                        // 特殊处理1位布尔类型
                                        if *bit_length == 1 {
                                            return "bool".to_string();
                                        }

                                        if let Some(restriction) = restrictions {
                                            if restriction.min_max.is_some()
                                                || restriction.multiple_of.is_some()
                                                || !restriction.forbidden_values.is_empty()
                                                || restriction.odd_only.unwrap_or(false)
                                            {
                                                return self.generate_restricted_immediate_type_name(
                                                    operand_type,
                                                    &operand.name,
                                                    *bit_length,
                                                    restriction,
                                                );
                                            }
                                        }

                                        format!("UImmediate<{}>", bit_length)
                                    }
            riscv_instruction_parser::types::OperandType::FliConstant => "FliConstant".to_string(),
            riscv_instruction_parser::types::OperandType::SavedRegListWithStackAdj => {
                        match isa_base {
                            ISABase::RV32 => "SavedRegListWithStackAdjRv32".to_string(),
                            ISABase::RV64 => "SavedRegListWithStackAdjRv64".to_string(),
                            // ISABase::RV128 => "SavedRegListWithStackAdjRv128".to_string(), // Placeholder for future RV128 support
                        }
                    }
            riscv_instruction_parser::types::OperandType::SavedIntegerRegister => {
                        match *bit_length {
                            3 => "CompressedSavedIntegerRegister".to_string(),
                            5 => "SavedIntegerRegister".to_string(),
                            _ => unreachable!("Invalid bit length for saved integer register: {}", bit_length),
                        }
                    }
            riscv_instruction_parser::types::OperandType::NotEqualCompressedSavedIntegerRegisterPair => "NotEqualCompressedSavedIntegerRegisterPair".to_string(),
        }
    }

    /// 创建寄存器类型签名用于去重
    fn create_register_type_signature(
        &self,
        base_type: &str,
        operand_name: &str,
        restriction: &riscv_instruction_parser::types::OperandRestriction,
    ) -> String {
        format!(
            "{}_{}_{}_{:?}_{:?}_{}",
            base_type,
            operand_name,
            restriction
                .multiple_of
                .map_or("none".to_string(), |m| m.to_string()),
            restriction.min_max,
            restriction.forbidden_values,
            restriction.odd_only.unwrap_or(false)
        )
    }

    /// 生成受限寄存器类型名称
    fn generate_restricted_register_type_name(
        &self,
        base_type: &str,
        operand_name: &str,
        restriction: &riscv_instruction_parser::types::OperandRestriction,
    ) -> String {
        let mut name_parts = vec![format!("Restricted{}", base_type)];

        // 添加操作数名称以区分不同用途的寄存器
        let operand_name_capitalized = operand_name
            .chars()
            .next()
            .unwrap()
            .to_uppercase()
            .collect::<String>() + &operand_name[1..];
        name_parts.push(operand_name_capitalized);

        // 添加限制条件到类型名
        if let Some(multiple) = restriction.multiple_of {
            name_parts.push(format!("Mul{}", multiple));
        }
        if let Some((min, max)) = restriction.min_max {
            // 处理负数，将负号替换为Neg
            let min_str = if min < 0 {
                format!("Neg{}", min.abs())
            } else {
                min.to_string()
            };
            let max_str = if max < 0 {
                format!("Neg{}", max.abs())
            } else {
                max.to_string()
            };
            name_parts.push(format!("Range{}To{}", min_str, max_str));
        }
        if !restriction.forbidden_values.is_empty() {
            let forbidden_str = restriction
                .forbidden_values
                .iter()
                .map(|v| {
                    if *v < 0 {
                        format!("Neg{}", v.abs())
                    } else {
                        v.to_string()
                    }
                })
                .collect::<Vec<_>>()
                .join("_");
            name_parts.push(format!("Forbidden{}", forbidden_str));
        }
        if restriction.odd_only.unwrap_or(false) {
            name_parts.push("OddOnly".to_string());
        }

        name_parts.join("")
    }

    /// 生成受限立即数类型名称
    fn generate_restricted_immediate_type_name(
        &self,
        operand_type: &riscv_instruction_parser::types::OperandType,
        operand_name: &str,
        bit_length: u8,
        restriction: &riscv_instruction_parser::types::OperandRestriction,
    ) -> String {
        let mut name_parts = vec![format!("Restricted")];

        // 基础类型
        let base_type = match operand_type {
            riscv_instruction_parser::types::OperandType::UnsignedInteger => "UImmediate",
            riscv_instruction_parser::types::OperandType::SignedInteger => match operand_name {
                "nzimm" => "NZImmediate",
                _ => "Immediate",
            },
            _ => unreachable!("Invalid operand type for immediate"),
        };
        name_parts.push(base_type.to_string());
        name_parts.push(format!("{}", bit_length));

        // 添加限制条件到类型名
        if let Some(multiple) = restriction.multiple_of {
            name_parts.push(format!("Mul{}", multiple));
        }
        if let Some((min, max)) = restriction.min_max {
            // 处理负数，将负号替换为Neg
            let min_str = if min < 0 {
                format!("Neg{}", min.abs())
            } else {
                min.to_string()
            };
            let max_str = if max < 0 {
                format!("Neg{}", max.abs())
            } else {
                max.to_string()
            };
            name_parts.push(format!("Range{}To{}", min_str, max_str));
        }
        if !restriction.forbidden_values.is_empty() {
            let forbidden_str = restriction
                .forbidden_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");

            name_parts.push(format!("Forbidden{}", forbidden_str));
        }
        if restriction.odd_only.unwrap_or(false) {
            name_parts.push("OddOnly".to_string());
        }

        name_parts.join("")
    }

    /// 生成受限寄存器类型定义
    fn generate_restricted_register_type_def(
        &self,
        base_type: &str,
        operand_name: &str,
        restriction: &riscv_instruction_parser::types::OperandRestriction,
    ) -> TokenStream {
        let type_name = syn::Ident::new(
            &self.generate_restricted_register_type_name(base_type, operand_name, restriction),
            proc_macro2::Span::call_site(),
        );

        let display_format = self.get_register_display_format(base_type);
        let register_name = self.get_register_type_name(base_type);

        // 设置默认的 min 和 max 值
        let (min_str, max_str) = if let Some((min, max)) = restriction.min_max {
            (min.to_string(), max.to_string())
        } else {
            ("0".to_string(), "31".to_string())
        };

        let mut attrs = vec![
            quote! { min = #min_str },
            quote! { max = #max_str },
            quote! { name = #register_name },
            quote! { display = #display_format },
        ];

        if let Some(multiple) = restriction.multiple_of {
            let multiple_str = multiple.to_string();
            attrs.push(quote! { multiple_of = #multiple_str });
        }

        if !restriction.forbidden_values.is_empty() {
            let forbidden_str = restriction
                .forbidden_values
                .iter()
                .map(|v| format!("{}u8", v))
                .collect::<Vec<_>>()
                .join(",");
            attrs.push(quote! { forbidden = #forbidden_str });
        }

        if restriction.odd_only.unwrap_or(false) {
            attrs.push(quote! { odd_only = "true" });
        }

        let doc_comment = format!(
            "{} for {} with constraints: {:?}",
            register_name, operand_name, restriction
        );

        quote! {
            #[doc = #doc_comment]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
            #[validated(#(#attrs),*)]
            pub struct #type_name(u8);
        }
    }

    /// 生成受限立即数类型定义
    fn generate_restricted_immediate_type_def(
        &self,
        operand_type: &riscv_instruction_parser::types::OperandType,
        operand_name: &str,
        bit_length: u8,
        restriction: &riscv_instruction_parser::types::OperandRestriction,
    ) -> TokenStream {
        let type_name = syn::Ident::new(
            &self.generate_restricted_immediate_type_name(
                operand_type,
                operand_name,
                bit_length,
                restriction,
            ),
            proc_macro2::Span::call_site(),
        );

        let inner_type = match operand_type {
            riscv_instruction_parser::types::OperandType::UnsignedInteger => quote! { u32 },
            riscv_instruction_parser::types::OperandType::SignedInteger => quote! { i32 },
            _ => unreachable!("Invalid operand type for immediate"),
        };

        let (min_str, max_str) = if let Some((min, max)) = restriction.min_max {
            (min.to_string(), max.to_string())
        } else {
            match operand_type {
                riscv_instruction_parser::types::OperandType::UnsignedInteger => (
                    "0".to_string(),
                    format!("((1u64 << {}) - 1) as u32", bit_length),
                ),
                riscv_instruction_parser::types::OperandType::SignedInteger => (
                    format!("-(1i32 << ({} - 1))", bit_length),
                    format!("(1i32 << ({} - 1)) - 1", bit_length),
                ),
                _ => unreachable!("Invalid operand type for immediate"),
            }
        };

        let mut attrs = vec![
            quote! { min = #min_str },
            quote! { max = #max_str },
            quote! { name = #operand_name },
        ];

        if let Some(multiple) = restriction.multiple_of {
            let multiple_str = multiple.to_string();
            attrs.push(quote! { multiple_of = #multiple_str });
        }

        if !restriction.forbidden_values.is_empty() {
            let forbidden_str = restriction
                .forbidden_values
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",");
            attrs.push(quote! { forbidden = #forbidden_str });
        }

        if restriction.odd_only.unwrap_or(false) {
            attrs.push(quote! { odd_only = "true" });
        }

        let doc_comment = format!(
            "Restricted {} with bit length {} and constraints: {:?}",
            operand_name, bit_length, restriction
        );

        quote! {
            #[doc = #doc_comment]
            #[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
            #[validated(#(#attrs),*)]
            pub struct #type_name(#inner_type);
        }
    }

    /// 创建立即数类型签名用于去重
    fn create_immediate_type_signature(
        &self,
        operand_type: &riscv_instruction_parser::types::OperandType,
        operand_name: &str,
        bit_length: u8,
        restriction: &riscv_instruction_parser::types::OperandRestriction,
    ) -> String {
        format!(
            "{:?}_{}_{}_{}_{:?}_{:?}_{}",
            operand_type,
            operand_name,
            bit_length,
            restriction
                .multiple_of
                .map_or("none".to_string(), |m| m.to_string()),
            restriction.min_max,
            restriction.forbidden_values,
            restriction.odd_only.unwrap_or(false)
        )
    }

    fn get_register_base_type_from_operand_type(
        &self,
        operand_type: &riscv_instruction_parser::types::OperandType,
        bit_length: u8,
    ) -> &'static str {
        match operand_type {
            riscv_instruction_parser::types::OperandType::IntegerRegister => {
                // 根据位长度判断是否为压缩寄存器
                match bit_length {
                    3 => "CompressedIntegerRegister",
                    5 => "IntegerRegister",
                    _ => unreachable!("Invalid bit length for integer register: {}", bit_length),
                }
            }
            riscv_instruction_parser::types::OperandType::FloatingPointRegister => {
                // 根据位长度判断是否为压缩寄存器
                match bit_length {
                    3 => "CompressedFloatingPointRegister",
                    5 => "FloatingPointRegister",
                    _ => unreachable!(
                        "Invalid bit length for floating point register: {}",
                        bit_length
                    ),
                }
            }
            riscv_instruction_parser::types::OperandType::VectorRegister => "VectorRegister",
            _ => unreachable!("Invalid operand type for register"),
        }
    }

    /// 获取寄存器类型的显示格式
    fn get_register_display_format(&self, base_type: &str) -> &'static str {
        match base_type {
            "IntegerRegister" | "CompressedIntegerRegister" => "x{}",
            "FloatingPointRegister" | "CompressedFloatingPointRegister" => "f{}",
            "VectorRegister" => "v{}",
            _ => "x{}", // 默认使用整数寄存器格式
        }
    }

    /// 获取寄存器类型的友好名称
    fn get_register_type_name(&self, base_type: &str) -> &'static str {
        match base_type {
            "IntegerRegister" => "Integer register",
            "CompressedIntegerRegister" => "Compressed integer register",
            "FloatingPointRegister" => "Floating point register",
            "CompressedFloatingPointRegister" => "Compressed floating point register",
            "VectorRegister" => "Vector register",
            _ => "Register", // 默认名称
        }
    }
}
