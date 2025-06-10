use super::{CodeGenerator, InstructionVariant};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use riscv_instruction_parser::types::{ISABase, ISAExtension, Instruction};
use std::collections::{HashMap, HashSet};
use syn::Ident;

impl CodeGenerator {
    pub fn generate_instruction_enums(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();

        // 首先生成所有指令结构体
        let instruction_structs = self.generate_instruction_structs(analysis);
        all_enums.extend(instruction_structs);

        let shared_enums = self.generate_shared_instructions_enum(analysis);
        all_enums.extend(shared_enums);

        let specific_enums = self.generate_isa_specific_instructions_enum(analysis);
        all_enums.extend(specific_enums);

        let main_enums = self.generate_extension_main_enum(analysis);
        all_enums.extend(main_enums);

        all_enums
    }

    /// 生成共享指令枚举
    pub fn generate_shared_instructions_enum(
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
    pub fn generate_isa_specific_instructions_enum(
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
    pub fn generate_extension_main_enum(
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
    pub fn generate_main_enum(
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
    pub fn generate_separated_enums(&self) -> TokenStream {
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

                    let struct_name = format!(
                        "{}_{}_{}",
                        isa_base,
                        extension,
                        self.instruction_name_to_variant(&inst.name)
                    );
                    let struct_ident = Ident::new(&struct_name, Span::call_site());

                    quote! {
                        #variant_name(#struct_ident)
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
    pub fn generate_separated_main_enum(&self) -> TokenStream {
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

    /// 生成扩展枚举和对应的随机生成方法
    pub fn generate_extension_enums_with_random(&self) -> TokenStream {
        let analysis = self.analyze_instruction_sharing(&self.instructions);

        let extension_enums = self.generate_extension_enums(&analysis);
        let extension_impls = self.generate_extension_random_impls(&analysis);

        quote! {
            #extension_enums
            #extension_impls
        }
    }

    /// 生成扩展枚举定义
    fn generate_extension_enums(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();

        // 收集所有扩展并按ISA基础分组
        let mut rv32_extensions = HashSet::new();
        let mut rv64_extensions = HashSet::new();

        for (extension, variants) in analysis {
            for variant in variants {
                for &isa_base in &variant.isa_bases {
                    match isa_base {
                        ISABase::RV32 => {
                            rv32_extensions.insert(*extension);
                        }
                        ISABase::RV64 => {
                            rv64_extensions.insert(*extension);
                        }
                    }
                }
            }
        }

        // 生成 RV32Extensions 枚举
        if !rv32_extensions.is_empty() {
            let mut rv32_variants = rv32_extensions.into_iter().collect::<Vec<_>>();
            rv32_variants.sort();

            let rv32_enum_variants = rv32_variants.iter().map(|ext| {
                let ident = Ident::new(&ext.to_string(), Span::call_site());
                quote! { #ident }
            });

            all_enums.extend(quote! {
                /// Available extensions for RV32 ISA base
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                #[allow(non_camel_case_types)]
                pub enum RV32Extensions {
                    #(#rv32_enum_variants),*
                }
            });
        }

        // 生成 RV64Extensions 枚举
        if !rv64_extensions.is_empty() {
            let mut rv64_variants = rv64_extensions.into_iter().collect::<Vec<_>>();
            rv64_variants.sort();

            let rv64_enum_variants = rv64_variants.iter().map(|ext| {
                let ident = Ident::new(&ext.to_string(), Span::call_site());
                quote! { #ident }
            });

            all_enums.extend(quote! {
                /// Available extensions for RV64 ISA base
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                #[allow(non_camel_case_types)]
                pub enum RV64Extensions {
                    #(#rv64_enum_variants),*
                }
            });
        }

        all_enums
    }

    /// 生成扩展枚举的随机生成实现
    fn generate_extension_random_impls(
        &self,
        analysis: &HashMap<ISAExtension, Vec<InstructionVariant>>,
    ) -> TokenStream {
        let mut all_impls = TokenStream::new();

        // 收集所有扩展并按ISA基础分组
        let mut rv32_extensions = HashSet::new();
        let mut rv64_extensions = HashSet::new();

        for (extension, variants) in analysis {
            for variant in variants {
                for &isa_base in &variant.isa_bases {
                    match isa_base {
                        ISABase::RV32 => {
                            rv32_extensions.insert(*extension);
                        }
                        ISABase::RV64 => {
                            rv64_extensions.insert(*extension);
                        }
                    }
                }
            }
        }

        // 为 RV32Extensions 生成实现
        if !rv32_extensions.is_empty() {
            let mut rv32_variants = rv32_extensions.into_iter().collect::<Vec<_>>();
            rv32_variants.sort();

            let rv32_match_arms = rv32_variants.iter().map(|ext| {
                let ext_ident = Ident::new(&ext.to_string(), Span::call_site());

                // 检查该扩展是否有共享指令和特定指令
                let variants = analysis.get(ext).unwrap();
                let has_shared = variants.iter().any(|v| v.is_shared);
                let has_rv32_specific = variants
                    .iter()
                    .any(|v| !v.is_shared && v.isa_bases.contains(&ISABase::RV32));

                if has_shared && has_rv32_specific {
                    // 既有共享又有特定的指令
                    let specific_enum_name = Ident::new(
                        &format!("RV32{}SpecificInstructions", ext),
                        Span::call_site(),
                    );
                    let shared_enum_name =
                        Ident::new(&format!("{}SharedInstructions", ext), Span::call_site());
                    quote! {
                        RV32Extensions::#ext_ident => {
                            if rng.random() {
                                RiscvInstruction::Shared(SharedInstruction::#ext_ident(
                                    #shared_enum_name::random_with_rng(rng)
                                ))
                            } else {
                                RiscvInstruction::Specific(SpecificInstruction::RV32(
                                    RV32SpecificInstruction::#ext_ident(
                                        #specific_enum_name::random_with_rng(rng)
                                    )
                                ))
                            }
                        }
                    }
                } else if has_shared {
                    // 只有共享指令
                    let shared_enum_name =
                        Ident::new(&format!("{}SharedInstructions", ext), Span::call_site());
                    quote! {
                        RV32Extensions::#ext_ident => {
                            RiscvInstruction::Shared(SharedInstruction::#ext_ident(
                                #shared_enum_name::random_with_rng(rng)
                            ))
                        }
                    }
                } else if has_rv32_specific {
                    // 只有特定指令
                    let specific_enum_name = Ident::new(
                        &format!("RV32{}SpecificInstructions", ext),
                        Span::call_site(),
                    );
                    quote! {
                        RV32Extensions::#ext_ident => {
                            RiscvInstruction::Specific(SpecificInstruction::RV32(
                                RV32SpecificInstruction::#ext_ident(
                                    #specific_enum_name::random_with_rng(rng)
                                )
                            ))
                        }
                    }
                } else {
                    // 不应该发生
                    quote! {
                        RV32Extensions::#ext_ident => unreachable!("No instructions for extension")
                    }
                }
            });

            all_impls.extend(quote! {
                impl RV32Extensions {
                    /// Generate a random instruction for this extension
                    pub fn random_instruction<R: rand::Rng>(&self, rng: &mut R) -> RiscvInstruction {
                        match self {
                            #(#rv32_match_arms),*
                        }
                    }
                }
            });
        }

        // 为 RV64Extensions 生成实现
        if !rv64_extensions.is_empty() {
            let mut rv64_variants = rv64_extensions.into_iter().collect::<Vec<_>>();
            rv64_variants.sort();

            let rv64_match_arms = rv64_variants.iter().map(|ext| {
                let ext_ident = Ident::new(&ext.to_string(), Span::call_site());

                // 检查该扩展是否有共享指令和特定指令
                let variants = analysis.get(ext).unwrap();
                let has_shared = variants.iter().any(|v| v.is_shared);
                let has_rv64_specific = variants
                    .iter()
                    .any(|v| !v.is_shared && v.isa_bases.contains(&ISABase::RV64));

                if has_shared && has_rv64_specific {
                    // 既有共享又有特定的指令
                    let specific_enum_name = Ident::new(
                        &format!("RV64{}SpecificInstructions", ext),
                        Span::call_site(),
                    );
                    let shared_enum_name =
                        Ident::new(&format!("{}SharedInstructions", ext), Span::call_site());
                    quote! {
                        RV64Extensions::#ext_ident => {
                            if rng.random() {
                                RiscvInstruction::Shared(SharedInstruction::#ext_ident(
                                    #shared_enum_name::random_with_rng(rng)
                                ))
                            } else {
                                RiscvInstruction::Specific(SpecificInstruction::RV64(
                                    RV64SpecificInstruction::#ext_ident(
                                        #specific_enum_name::random_with_rng(rng)
                                    )
                                ))
                            }
                        }
                    }
                } else if has_shared {
                    // 只有共享指令
                    let shared_enum_name =
                        Ident::new(&format!("{}SharedInstructions", ext), Span::call_site());
                    quote! {
                        RV64Extensions::#ext_ident => {
                            RiscvInstruction::Shared(SharedInstruction::#ext_ident(
                                #shared_enum_name::random_with_rng(rng)
                            ))
                        }
                    }
                } else if has_rv64_specific {
                    // 只有特定指令
                    let specific_enum_name = Ident::new(
                        &format!("RV64{}SpecificInstructions", ext),
                        Span::call_site(),
                    );
                    quote! {
                        RV64Extensions::#ext_ident => {
                            RiscvInstruction::Specific(SpecificInstruction::RV64(
                                RV64SpecificInstruction::#ext_ident(
                                    #specific_enum_name::random_with_rng(rng)
                                )
                            ))
                        }
                    }
                } else {
                    // 不应该发生
                    quote! {
                        RV64Extensions::#ext_ident => unreachable!("No instructions for extension")
                    }
                }
            });

            all_impls.extend(quote! {
                impl RV64Extensions {
                    /// Generate a random instruction for this extension
                    pub fn random_instruction<R: rand::Rng>(&self, rng: &mut R) -> RiscvInstruction {
                        match self {
                            #(#rv64_match_arms),*
                        }
                    }
                }
            });
        }

        all_impls
    }

    /// 生成分离模式的扩展枚举和对应的随机生成方法
    pub fn generate_separated_extension_enums_with_random(&self) -> TokenStream {
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

        let extension_enums = self.generate_separated_extension_enums(&by_extension_and_isa);
        let extension_impls = self.generate_separated_extension_random_impls(&by_extension_and_isa);

        quote! {
            #extension_enums
            #extension_impls
        }
    }

    /// 生成分离模式的扩展枚举定义
    fn generate_separated_extension_enums(
        &self,
        by_extension_and_isa: &HashMap<(ISAExtension, ISABase), Vec<&Instruction>>,
    ) -> TokenStream {
        let mut all_enums = TokenStream::new();

        // 收集扩展并按ISA基础分组
        let mut rv32_extensions = HashSet::new();
        let mut rv64_extensions = HashSet::new();

        for ((extension, isa_base), _) in by_extension_and_isa {
            match isa_base {
                ISABase::RV32 => {
                    rv32_extensions.insert(*extension);
                }
                ISABase::RV64 => {
                    rv64_extensions.insert(*extension);
                }
            }
        }

        // 生成 RV32Extensions 枚举
        if !rv32_extensions.is_empty() {
            let mut rv32_variants = rv32_extensions.into_iter().collect::<Vec<_>>();
            rv32_variants.sort();

            let rv32_enum_variants = rv32_variants.iter().map(|ext| {
                let ident = Ident::new(&ext.to_string(), Span::call_site());
                quote! { #ident }
            });

            all_enums.extend(quote! {
                /// Available extensions for RV32 ISA base
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                #[allow(non_camel_case_types)]
                pub enum RV32Extensions {
                    #(#rv32_enum_variants),*
                }
            });
        }

        // 生成 RV64Extensions 枚举
        if !rv64_extensions.is_empty() {
            let mut rv64_variants = rv64_extensions.into_iter().collect::<Vec<_>>();
            rv64_variants.sort();

            let rv64_enum_variants = rv64_variants.iter().map(|ext| {
                let ident = Ident::new(&ext.to_string(), Span::call_site());
                quote! { #ident }
            });

            all_enums.extend(quote! {
                /// Available extensions for RV64 ISA base
                #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
                #[allow(non_camel_case_types)]
                pub enum RV64Extensions {
                    #(#rv64_enum_variants),*
                }
            });
        }

        all_enums
    }

    /// 生成分离模式扩展枚举的随机生成实现
    fn generate_separated_extension_random_impls(
        &self,
        by_extension_and_isa: &HashMap<(ISAExtension, ISABase), Vec<&Instruction>>,
    ) -> TokenStream {
        let mut all_impls = TokenStream::new();

        // 收集扩展并按ISA基础分组
        let mut rv32_extensions = HashSet::new();
        let mut rv64_extensions = HashSet::new();

        for ((extension, isa_base), _) in by_extension_and_isa {
            match isa_base {
                ISABase::RV32 => {
                    rv32_extensions.insert(*extension);
                }
                ISABase::RV64 => {
                    rv64_extensions.insert(*extension);
                }
            }
        }

        // 为 RV32Extensions 生成实现
        if !rv32_extensions.is_empty() {
            let mut rv32_variants = rv32_extensions.into_iter().collect::<Vec<_>>();
            rv32_variants.sort();

            let rv32_match_arms = rv32_variants.iter().map(|ext| {
                let ext_ident = Ident::new(&ext.to_string(), Span::call_site());
                let enum_name = Ident::new(&format!("RV32{}Instructions", ext), Span::call_site());

                quote! {
                    RV32Extensions::#ext_ident => {
                        RiscvInstruction::RV32(RV32Instruction::#ext_ident(
                            #enum_name::random_with_rng(rng)
                        ))
                    }
                }
            });

            all_impls.extend(quote! {
                impl RV32Extensions {
                    /// Generate a random instruction for this extension
                    pub fn random_instruction<R: rand::Rng>(&self, rng: &mut R) -> RiscvInstruction {
                        match self {
                            #(#rv32_match_arms),*
                        }
                    }
                }
            });
        }

        // 为 RV64Extensions 生成实现
        if !rv64_extensions.is_empty() {
            let mut rv64_variants = rv64_extensions.into_iter().collect::<Vec<_>>();
            rv64_variants.sort();

            let rv64_match_arms = rv64_variants.iter().map(|ext| {
                let ext_ident = Ident::new(&ext.to_string(), Span::call_site());
                let enum_name = Ident::new(&format!("RV64{}Instructions", ext), Span::call_site());

                quote! {
                    RV64Extensions::#ext_ident => {
                        RiscvInstruction::RV64(RV64Instruction::#ext_ident(
                            #enum_name::random_with_rng(rng)
                        ))
                    }
                }
            });

            all_impls.extend(quote! {
                impl RV64Extensions {
                    /// Generate a random instruction for this extension
                    pub fn random_instruction<R: rand::Rng>(&self, rng: &mut R) -> RiscvInstruction {
                        match self {
                            #(#rv64_match_arms),*
                        }
                    }
                }
            });
        }

        all_impls
    }
}
