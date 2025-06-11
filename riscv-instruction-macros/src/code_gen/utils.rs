use super::{InstructionVariant, CodeGenerator};
use riscv_instruction_parser::types::{Operand, ISABase};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::Ident;

impl CodeGenerator {
    /// 生成导入语句
    pub fn generate_imports(&self) -> TokenStream {
        quote! {
            use std::fmt::{self, Display};
            pub use riscv_instruction_types::*;
            use riscv_instruction_macros::{DeriveValidatedValue, DeriveInstructionDisplay, DeriveRandom};
            use serde::{Deserialize, Serialize};
            use enum_iterator::Sequence;
        }
    }

    pub fn instruction_name_to_variant(&self, name: &str) -> String {
        name.replace('.', "_").replace('-', "_").to_uppercase()
    }

    pub fn operand_to_typed_struct(&self, operand: &Operand, isa_base: &ISABase) -> String {
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

    /// 获取变体所属的扩展
    pub fn get_extension_for_variant(&self, variant: &InstructionVariant) -> String {
        variant.instruction.extension.to_string()
    }

    /// 构建枚举变体
    pub fn build_variants(
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

                // 创建结构体名称，包含前缀避免冲突
                let struct_name = if variant.is_shared {
                    format!(
                        "{}_Shared_{}",
                        self.get_extension_for_variant(variant),
                        self.instruction_name_to_variant(&variant.instruction.name)
                    )
                } else {
                    let isa_base = isa_base_override.unwrap_or_else(|| &variant.isa_bases[0]);
                    format!(
                        "{}_{}_{}",
                        isa_base,
                        self.get_extension_for_variant(variant),
                        self.instruction_name_to_variant(&variant.instruction.name)
                    )
                };

                let struct_ident = Ident::new(&struct_name, Span::call_site());

                quote! {
                    #variant_name(#struct_ident)
                }
            })
            .collect()
    }
}