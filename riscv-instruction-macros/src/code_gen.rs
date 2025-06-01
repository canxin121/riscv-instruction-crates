use crate::instruction_types::{ISABase, ISAExtension, Instruction, Operand};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::Ident;

// InstructionVariant 结构体保持不变，因为它只用于数据分析
#[derive(Debug, Clone)]
struct InstructionVariant {
    instruction: Instruction,
    isa_bases: Vec<ISABase>,
    is_shared: bool,
}

pub struct CodeGenerator {
    rvc_instructions: Vec<Instruction>,
    standard_instructions: Vec<Instruction>,
}

impl CodeGenerator {
    // new 函数保持不变
    pub fn new(
        rvc_instructions: Vec<Instruction>,
        standard_instructions: Vec<Instruction>,
    ) -> Self {
        Self {
            rvc_instructions,
            standard_instructions,
        }
    }

    /// 主生成函数，现在返回 TokenStream
    pub fn generate_instruction_enum(&self) -> TokenStream {
        // 分别分析 RVC 和 Standard 指令的共享性
        let rvc_analysis = self.analyze_instruction_sharing_for_type(&self.rvc_instructions);
        let standard_analysis =
            self.analyze_instruction_sharing_for_type(&self.standard_instructions);

        // 收集所有需要的受限寄存器类型定义
        let mut restricted_register_defs = TokenStream::new();
        let mut processed_combinations = std::collections::HashSet::new();

        // 扫描所有指令以找到需要的受限寄存器类型
        for analysis in [&rvc_analysis, &standard_analysis] {
            for variants in analysis.values() {
                for variant in variants {
                    for operand in &variant.instruction.operands {
                        if let Some(restriction) = &operand.restrictions {
                            if !restriction.forbidden_values.is_empty() {
                                let base_type = match operand.name.as_str() {
                                    "rd" | "rs1" | "rs2" | "rs3" => "IntegerRegister",
                                    "rdp" | "rs1p" | "rs2p" | "rs3p" => "CompressedIntegerRegister",
                                    "fd" | "fs1" | "fs2" | "fs3" => "FloatingPointRegister",
                                    "fdp" | "fs1p" | "fs2p" | "fs3p" => {
                                        "CompressedFloatingPointRegister"
                                    }
                                    _ if operand.name.starts_with("rs")
                                        || operand.name.starts_with("rd") =>
                                    {
                                        "IntegerRegister"
                                    }
                                    _ if operand.name.starts_with("fs")
                                        || operand.name.starts_with("fd") =>
                                    {
                                        "FloatingPointRegister"
                                    }
                                    _ => continue,
                                };

                                let type_signature =
                                    (base_type, restriction.forbidden_values.clone());
                                if processed_combinations.insert(type_signature.clone()) {
                                    let type_name = self.generate_restricted_register_type(
                                        base_type,
                                        &restriction.forbidden_values,
                                    );

                                    // 生成类型定义
                                    let type_ident =
                                        syn::Ident::new(&type_name, proc_macro2::Span::call_site());
                                    let forbidden_list = restriction
                                        .forbidden_values
                                        .iter()
                                        .map(|v| v.to_string())
                                        .collect::<Vec<_>>()
                                        .join(",");

                                    let (max_val, display_prefix) = match base_type {
                                        "IntegerRegister" => ("31", "x{}"),
                                        "FloatingPointRegister" => ("31", "f{}"),
                                        "CompressedIntegerRegister" => ("15", "x{}"),
                                        "CompressedFloatingPointRegister" => ("15", "f{}"),
                                        _ => ("31", "x{}"),
                                    };

                                    // 为压缩寄存器类型生成特殊的 Display 实现
                                    let (display_attr, display_impl) = if base_type
                                        .contains("Compressed")
                                    {
                                        let prefix = if display_prefix.starts_with("x") {
                                            "x"
                                        } else {
                                            "f"
                                        };
                                        (
                                            quote! { skip_display, },
                                            quote! {
                                                impl std::fmt::Display for #type_ident {
                                                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                                        write!(f, "{}{}", #prefix, self.0)
                                                    }
                                                }
                                            },
                                        )
                                    } else {
                                        (quote! { display = #display_prefix, }, quote! {})
                                    };

                                    // 生成受限寄存器类型定义
                                    restricted_register_defs.extend(quote! {
                                        #[derive(Debug, Clone, Copy, PartialEq, Eq, DeriveValidatedValue, DeriveRandom)]
                                        #[validated(min = "0", max = #max_val, name = #type_name, #display_attr forbidden = #forbidden_list)]
                                        pub struct #type_ident(u8);

                                        #display_impl
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        // 调用各个子生成函数
        let imports = self.generate_imports();
        let rvc_enums = self.generate_instruction_type_enums(&rvc_analysis, "RVC");
        let standard_enums = self.generate_instruction_type_enums(&standard_analysis, "Standard");
        let main_enum = self.generate_top_level_enum(&rvc_analysis, &standard_analysis);

        // 使用 quote! 宏将所有 TokenStream 片段组合成一个文件
        quote! {
            // This file is auto-generated by the RISC-V instruction code generator.
            // Author: canxin121 <q1969730106@gmail.com>
            // Do not edit this file manually.
            #imports

            // --- Generated Restricted Register Types ---
            #restricted_register_defs

            // --- RVC Instruction Enums ---
            #rvc_enums

            // --- Standard Instruction Enums ---
            #standard_enums

            // --- Top Level Enums ---
            #main_enum
        }
    }

    /// 为特定指令类型分析指令共享性
    fn analyze_instruction_sharing_for_type(
        &self,
        instructions: &[Instruction],
    ) -> HashMap<ISAExtension, Vec<InstructionVariant>> {
        let mut analysis: HashMap<ISAExtension, Vec<InstructionVariant>> = HashMap::new();
        let mut by_extension: HashMap<ISAExtension, Vec<&Instruction>> = HashMap::new();

        for inst in instructions {
            by_extension.entry(inst.extension).or_default().push(inst);
        }

        for (extension, instructions) in by_extension {
            let mut variants = Vec::new();
            let mut processed_names = HashSet::new();

            for &inst in &instructions {
                if processed_names.contains(&inst.name) {
                    continue;
                }
                processed_names.insert(&inst.name);

                let same_name_instructions: Vec<&Instruction> = instructions
                    .iter()
                    .filter(|&&i| i.name == inst.name)
                    .copied()
                    .collect();

                if same_name_instructions.len() == 1 {
                    variants.push(InstructionVariant {
                        instruction: inst.clone(),
                        isa_bases: inst.isa_bases.clone(),
                        is_shared: inst.isa_bases.len() > 1,
                    });
                } else {
                    let is_identical = self.are_instructions_identical(&same_name_instructions);
                    if is_identical {
                        let mut merged_isa_bases = Vec::new();
                        for inst in &same_name_instructions {
                            for &isa_base in &inst.isa_bases {
                                if !merged_isa_bases.contains(&isa_base) {
                                    merged_isa_bases.push(isa_base);
                                }
                            }
                        }
                        let mut merged_inst = inst.clone();
                        merged_inst.isa_bases = merged_isa_bases.clone();
                        variants.push(InstructionVariant {
                            instruction: merged_inst,
                            isa_bases: merged_isa_bases.clone(),
                            is_shared: merged_isa_bases.len() > 1,
                        });
                    } else {
                        for &different_inst in &same_name_instructions {
                            variants.push(InstructionVariant {
                                instruction: different_inst.clone(),
                                isa_bases: different_inst.isa_bases.clone(),
                                is_shared: false,
                            });
                        }
                    }
                }
            }
            analysis.insert(extension, variants);
        }
        analysis
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

    /// 生成特定指令类型的所有枚举 (RVC 或 Standard)
    fn generate_instruction_type_enums(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
        type_prefix: &str,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();

        // 生成共享指令枚举
        let shared_enums = self.generate_shared_instructions_enum_for_type(analysis, type_prefix);
        all_enums.extend(shared_enums);

        // 生成 ISA 特定指令枚举
        let specific_enums =
            self.generate_isa_specific_instructions_enum_for_type(analysis, type_prefix);
        all_enums.extend(specific_enums);

        // 生成该类型的主枚举 (Shared + Specific)
        let main_enums = self.generate_type_main_enum(analysis, type_prefix);
        all_enums.extend(main_enums);

        all_enums
    }

    /// 生成特定类型的共享指令枚举
    fn generate_shared_instructions_enum_for_type(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
        type_prefix: &str,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();
        for (extension, variants) in analysis {
            let shared_variants: Vec<_> = variants.iter().filter(|v| v.is_shared).collect();
            if shared_variants.is_empty() {
                continue;
            }

            let enum_name = Ident::new(
                &format!("{}{}SharedInstructions", type_prefix, extension),
                Span::call_site(),
            );
            let doc_comment = format!(
                "{} shared instructions for {} extension",
                type_prefix, extension
            );

            let variant_tokens = self.build_variants(&shared_variants, None);

            all_enums.extend(quote! {
                #[doc = #doc_comment]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                #[rustfmt::skip]
                pub enum #enum_name {
                    #(#variant_tokens),*
                }
            });
        }
        all_enums
    }

    /// 生成特定类型的 ISA 特定指令枚举
    fn generate_isa_specific_instructions_enum_for_type(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
        type_prefix: &str,
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
                    &format!(
                        "{}{}{}SpecificInstructions",
                        type_prefix, isa_base, extension
                    ),
                    Span::call_site(),
                );
                let doc_comment = format!(
                    "{} {} specific instructions for {} extension",
                    type_prefix, isa_base, extension
                );

                let variant_tokens = self.build_variants(&isa_specific_variants, Some(isa_base));

                all_enums.extend(quote! {
                    #[doc = #doc_comment]
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

    /// 生成特定类型的主枚举 (Shared + Specific)
    fn generate_type_main_enum(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
        type_prefix: &str,
    ) -> TokenStream {
        let mut main_enums = TokenStream::new();

        // --- TypeSharedInstruction Enum ---
        let shared_variants = analysis
            .iter()
            .filter_map(|(ext, vars)| {
                if vars.iter().any(|v| v.is_shared) {
                    let ext_ident = Ident::new(&ext.to_string(), Span::call_site());
                    let enum_name = Ident::new(
                        &format!("{}{}SharedInstructions", type_prefix, ext),
                        Span::call_site(),
                    );
                    Some(quote! { #ext_ident(#enum_name) })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !shared_variants.is_empty() {
            let shared_enum_name = Ident::new(
                &format!("{}SharedInstruction", type_prefix),
                Span::call_site(),
            );
            let shared_doc = format!(
                "{} instructions shared across all ISA bases, grouped by extension.",
                type_prefix
            );
            main_enums.extend(quote! {
                #[doc = #shared_doc]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum #shared_enum_name {
                    #(#shared_variants),*
                }
            });
        }

        // --- ISA-specific Enums ---
        for isa_base in &[ISABase::RV32, ISABase::RV64] {
            let isa_base_str = isa_base.to_string();
            let specific_enum_name = Ident::new(
                &format!("{}{}SpecificInstruction", type_prefix, isa_base),
                Span::call_site(),
            );
            let doc_comment = format!(
                "{} {} specific instructions, grouped by extension.",
                type_prefix, isa_base_str
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
                            &format!("{}{}{}SpecificInstructions", type_prefix, isa_base, ext),
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
                    #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                    pub enum #specific_enum_name {
                        #(#extension_variants),*
                    }
                });
            }
        }

        // --- TypeSpecificInstruction Enum ---
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
                        &format!("{}{}SpecificInstruction", type_prefix, isa_base),
                        Span::call_site(),
                    );
                    Some(quote! { #isa_base_ident(#enum_name) })
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        if !specific_variants.is_empty() {
            let specific_enum_name = Ident::new(
                &format!("{}SpecificInstruction", type_prefix),
                Span::call_site(),
            );
            let specific_doc = format!("{} ISA base specific instructions.", type_prefix);
            main_enums.extend(quote! {
                #[doc = #specific_doc]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum #specific_enum_name {
                    #(#specific_variants),*
                }
            });
        }

        // --- Complete Type Instruction Enum ---
        let type_enum_name = Ident::new(&format!("{}Instruction", type_prefix), Span::call_site());
        let type_doc = format!(
            "{} RISC-V instructions, dispatching to shared or specific instructions.",
            type_prefix
        );

        let mut type_variants = Vec::new();
        if !shared_variants.is_empty() {
            let shared_enum_name = Ident::new(
                &format!("{}SharedInstruction", type_prefix),
                Span::call_site(),
            );
            type_variants.push(quote! {
                #[doc = "Instructions shared across ISA bases"]
                Shared(#shared_enum_name)
            });
        }
        if !specific_variants.is_empty() {
            let specific_enum_name = Ident::new(
                &format!("{}SpecificInstruction", type_prefix),
                Span::call_site(),
            );
            type_variants.push(quote! {
                #[doc = "ISA base specific instructions"]
                Specific(#specific_enum_name)
            });
        }

        if !type_variants.is_empty() {
            main_enums.extend(quote! {
                #[doc = #type_doc]
                #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
                pub enum #type_enum_name {
                    #(#type_variants),*
                }
            });
        }

        main_enums
    }

    /// 生成最顶层的指令枚举
    fn generate_top_level_enum(
        &self,
        rvc_analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
        standard_analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut top_level_variants = Vec::new();

        // 检查是否有 Standard 指令
        if !standard_analysis.is_empty() {
            top_level_variants.push(quote! {
                #[doc = "Standard RISC-V instructions"]
                Standard(StandardInstruction)
            });
        }

        // 检查是否有 RVC 指令
        if !rvc_analysis.is_empty() {
            top_level_variants.push(quote! {
                #[doc = "RISC-V Compressed instructions"]
                RVC(RVCInstruction)
            });
        }

        quote! {
            /// Main RISC-V instruction enum, dispatching to Standard or RVC instructions.
            #[derive(Debug, Clone, PartialEq, DeriveInstructionDisplay, DeriveRandom)]
            pub enum RiscvInstruction {
                #(#top_level_variants),*
            }
        }
    }

    /// 辅助函数，用于构建枚举的变体列表
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

                // 根据 assembly_syntax 类型来处理
                let attribute_token = match &variant.instruction.assembly_syntax {
                    crate::instruction_types::AssemblySyntax::RustCode(code) => {
                        // 对于 RustCode，需要确保代码被正确处理
                        // 不需要额外的字符串字面量包装，直接使用代码
                        let code_str = code.as_str();
                        quote! { #[asm_code(#code_str)] }
                    }
                    crate::instruction_types::AssemblySyntax::Format(format_str) => {
                        // 对于 Format，用字符串字面量
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
                            // 如果提供了 override，使用它；否则，从变体中取第一个
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

    /// 生成 use 语句
    fn generate_imports(&self) -> TokenStream {
        quote! {
            use std::fmt::{self, Display};
            // Import all types from the types crate
            pub use riscv_instruction_types::*;
            use riscv_instruction_macros::{DeriveValidatedValue, DeriveInstructionDisplay, DeriveRandom};
        }
    }

    // instruction_name_to_variant 和 operand_to_typed_struct 保持不变，
    // 它们是纯粹的字符串处理帮助函数。
    fn instruction_name_to_variant(&self, name: &str) -> String {
        name.replace('.', "_").replace('-', "_").to_uppercase()
    }

    fn operand_to_typed_struct(&self, operand: &Operand, isa_base: &ISABase) -> String {
        let bit_length = operand.bit_lengths.get(isa_base).unwrap_or(&32);
        let restrictions = operand.restrictions.as_ref();

        match operand.name.as_str() {
            // 整数寄存器类型
            "rd" | "rs1" | "rs2" | "rs3" => {
                if let Some(restriction) = restrictions {
                    if !restriction.forbidden_values.is_empty() {
                        let forbidden_values = &restriction.forbidden_values;
                        return self.generate_restricted_register_type(
                            "IntegerRegister",
                            forbidden_values,
                        );
                    }
                }
                "IntegerRegister".to_string()
            }
            // 压缩整数寄存器类型 (限制为 x8-x15)
            "rdp" | "rs1p" | "rs2p" | "rs3p" => {
                if let Some(restriction) = restrictions {
                    if !restriction.forbidden_values.is_empty() {
                        let forbidden_values = &restriction.forbidden_values;
                        return self.generate_restricted_register_type(
                            "CompressedIntegerRegister",
                            forbidden_values,
                        );
                    }
                }
                "CompressedIntegerRegister".to_string()
            }

            // 浮点寄存器类型
            "fd" | "fs1" | "fs2" | "fs3" => {
                if let Some(restriction) = restrictions {
                    if !restriction.forbidden_values.is_empty() {
                        let forbidden_values = &restriction.forbidden_values;
                        return self.generate_restricted_register_type(
                            "FloatingPointRegister",
                            forbidden_values,
                        );
                    }
                }
                "FloatingPointRegister".to_string()
            }
            // 压缩浮点寄存器类型 (限制为 f8-f15)
            "fdp" | "fs1p" | "fs2p" | "fs3p" => {
                if let Some(restriction) = restrictions {
                    if !restriction.forbidden_values.is_empty() {
                        let forbidden_values = &restriction.forbidden_values;
                        return self.generate_restricted_register_type(
                            "CompressedFloatingPointRegister",
                            forbidden_values,
                        );
                    }
                }
                "CompressedFloatingPointRegister".to_string()
            }

            // 立即数类型 - 处理各种立即数变体
            "imm" | "custom_imm" | "rimm" => {
                if let Some(restriction) = restrictions {
                    // 检查是否同时有自定义范围和非零约束
                    if let Some((min, max)) = restriction.min_max {
                        if restriction.not_zero {
                            return format!("NonZeroRangeImmediate<{}, {}>", min, max);
                        } else {
                            return format!("RangeImmediate<{}, {}>", min, max);
                        }
                    }
                    // 检查是否有倍数限制
                    if let Some(multiple) = restriction.multiple_of {
                        if restriction.not_zero {
                            return format!("MultipleOfNZImmediate<{}, {}>", bit_length, multiple);
                        } else {
                            return format!("MultipleOfImmediate<{}, {}>", bit_length, multiple);
                        }
                    }
                    // 检查是否有非零限制
                    if restriction.not_zero {
                        return format!("NZImmediate<{}>", bit_length);
                    }
                }
                format!("Immediate<{}>", bit_length)
            }
            "uimm" => {
                if let Some(restriction) = restrictions {
                    // 检查是否有自定义范围
                    if let Some((min, max)) = restriction.min_max {
                        // 对于无符号立即数，如果有自定义范围，我们需要确保范围是非负的
                        if min >= 0 {
                            if restriction.not_zero {
                                return format!("NonZeroRangeImmediate<{}, {}>", min, max);
                            } else {
                                return format!("RangeImmediate<{}, {}>", min, max);
                            }
                        }
                    }
                    if let Some(multiple) = restriction.multiple_of {
                        format!("MultipleOfUImmediate<{}, {}>", bit_length, multiple)
                    } else {
                        format!("UImmediate<{}>", bit_length)
                    }
                } else {
                    format!("UImmediate<{}>", bit_length)
                }
            }
            "nzimm" => {
                if let Some(restriction) = restrictions {
                    if let Some((min, max)) = restriction.min_max {
                        return format!("NonZeroRangeImmediate<{}, {}>", min, max);
                    }
                    if let Some(multiple) = restriction.multiple_of {
                        format!("MultipleOfNZImmediate<{}, {}>", bit_length, multiple)
                    } else {
                        format!("NZImmediate<{}>", bit_length)
                    }
                } else {
                    format!("NZImmediate<{}>", bit_length)
                }
            }
            "nzuimm" => {
                if let Some(restriction) = restrictions {
                    if let Some((min, max)) = restriction.min_max {
                        if min >= 0 {
                            return format!("NonZeroRangeImmediate<{}, {}>", min, max);
                        }
                    }
                    if let Some(multiple) = restriction.multiple_of {
                        format!("MultipleOfNZUImmediate<{}, {}>", bit_length, multiple)
                    } else {
                        format!("NZUImmediate<{}>", bit_length)
                    }
                } else {
                    format!("NZUImmediate<{}>", bit_length)
                }
            }

            // 移位量
            "shamt" => format!("ShiftAmount<{}>", bit_length),

            // 特殊寄存器和控制
            "csr" => "CSRAddress".to_string(),
            "rm" => "RoundingMode".to_string(),
            "aq" | "rl" => "bool".to_string(),
            "pred" | "succ" | "fm" => "FenceMode".to_string(),

            // 其他以特定前缀开头的操作数
            name => {
                // 首先检查是否有限制条件，如果有则按立即数处理
                if let Some(restriction) = restrictions {
                    if let Some((min, max)) = restriction.min_max {
                        if restriction.not_zero {
                            return format!("NonZeroRangeImmediate<{}, {}>", min, max);
                        } else {
                            return format!("RangeImmediate<{}, {}>", min, max);
                        }
                    }
                    if let Some(multiple) = restriction.multiple_of {
                        if restriction.not_zero {
                            return format!("MultipleOfNZImmediate<{}, {}>", bit_length, multiple);
                        } else {
                            return format!("MultipleOfImmediate<{}, {}>", bit_length, multiple);
                        }
                    }
                    if restriction.not_zero {
                        return format!("NZImmediate<{}>", bit_length);
                    }
                }

                // 根据名称前缀进行分类，同时考虑禁用值
                if name.starts_with("rs") || name.starts_with("rd") {
                    if let Some(restriction) = restrictions {
                        if !restriction.forbidden_values.is_empty() {
                            let forbidden_values = &restriction.forbidden_values;
                            return self.generate_restricted_register_type(
                                "IntegerRegister",
                                forbidden_values,
                            );
                        }
                    }
                    "IntegerRegister".to_string()
                } else if name.starts_with("fs") || name.starts_with("fd") {
                    if let Some(restriction) = restrictions {
                        if !restriction.forbidden_values.is_empty() {
                            let forbidden_values = &restriction.forbidden_values;
                            return self.generate_restricted_register_type(
                                "FloatingPointRegister",
                                forbidden_values,
                            );
                        }
                    }
                    "FloatingPointRegister".to_string()
                } else if name.ends_with("imm") || name.contains("imm") {
                    // 任何包含 "imm" 的操作数都被视为立即数
                    if let Some(restriction) = restrictions {
                        if let Some((min, max)) = restriction.min_max {
                            if restriction.not_zero {
                                return format!("NonZeroRangeImmediate<{}, {}>", min, max);
                            } else {
                                return format!("RangeImmediate<{}, {}>", min, max);
                            }
                        }
                        if restriction.not_zero {
                            return format!("NZImmediate<{}>", bit_length);
                        }
                    }
                    format!("Immediate<{}>", bit_length)
                } else {
                    // 最后的回退选项
                    "u32".to_string()
                }
            }
        }
    }

    /// 生成受限寄存器类型
    fn generate_restricted_register_type(
        &self,
        base_type: &str,
        forbidden_values: &[u8],
    ) -> String {
        // 对禁用值进行排序以确保一致性
        let mut sorted_forbidden = forbidden_values.to_vec();
        sorted_forbidden.sort();

        // 根据禁用值的数量和内容生成可读的名称
        let forbidden_desc = if sorted_forbidden.len() == 1 {
            format!("Except{}", sorted_forbidden[0])
        } else if sorted_forbidden.len() <= 3 {
            // 对于少量禁用值，直接列出
            let values_str = sorted_forbidden
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("And");
            format!("Except{}", values_str)
        } else if sorted_forbidden.len() <= 5 {
            // 对于中等数量的禁用值，使用更紧凑的格式
            let values_str = sorted_forbidden
                .iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("_");
            format!("ExceptRegs{}", values_str)
        } else {
            // 对于大量禁用值，使用范围或模式描述
            let count = sorted_forbidden.len();
            if self.is_consecutive_range(&sorted_forbidden) {
                format!(
                    "ExceptRange{}To{}",
                    sorted_forbidden[0],
                    sorted_forbidden[sorted_forbidden.len() - 1]
                )
            } else {
                format!("ExceptMultiple{}Regs", count)
            }
        };

        format!("{}{}", base_type, forbidden_desc)
    }

    /// 检查是否为连续范围
    fn is_consecutive_range(&self, values: &[u8]) -> bool {
        if values.len() < 2 {
            return false;
        }

        for i in 1..values.len() {
            if values[i] != values[i - 1] + 1 {
                return false;
            }
        }
        true
    }
}
