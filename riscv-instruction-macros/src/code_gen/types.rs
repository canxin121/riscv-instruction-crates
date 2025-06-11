use super::CodeGenerator;
use proc_macro2::TokenStream;
use quote::quote;

impl CodeGenerator {
    pub fn get_register_base_type_from_operand_type(
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
    pub fn get_register_display_format(&self, base_type: &str) -> &'static str {
        match base_type {
            "IntegerRegister" | "CompressedIntegerRegister" => "x{}",
            "FloatingPointRegister" | "CompressedFloatingPointRegister" => "f{}",
            "VectorRegister" => "v{}",
            _ => "x{}", // 默认使用整数寄存器格式
        }
    }

    /// 获取寄存器类型的友好名称
    pub fn get_register_type_name(&self, base_type: &str) -> &'static str {
        match base_type {
            "IntegerRegister" => "Integer register",
            "CompressedIntegerRegister" => "Compressed integer register",
            "FloatingPointRegister" => "Floating point register",
            "CompressedFloatingPointRegister" => "Compressed floating point register",
            "VectorRegister" => "Vector register",
            _ => "Register", // 默认名称
        }
    }

    /// 生成受限寄存器类型名称
    pub fn generate_restricted_register_type_name(
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
            .collect::<String>()
            + &operand_name[1..];
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
    pub fn generate_restricted_immediate_type_name(
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
    pub fn generate_restricted_register_type_def(
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
    pub fn generate_restricted_immediate_type_def(
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
}
