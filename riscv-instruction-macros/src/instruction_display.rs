use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Generates the implementation for the `std::fmt::Display` trait for an enum.
/// Expects `#[asm("...")]` or `#[asm_code(...)]` attributes on variants for custom formatting.
pub fn impl_instruction_display(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(data) => &data.variants,
        _ => return quote! { compile_error!("InstructionDisplay can only be derived for enums"); },
    };

    // Generate match arms for the Display implementation.
    let match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let fields = &variant.fields;

        // Find the `#[asm(...)]` or `#[asm_code(...)]` attribute on the variant.
        let asm_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("asm"));
            
        let asm_code_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("asm_code"));

        // Collect field names for named fields.
        let field_names: Vec<_> = match fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().clone())
                .collect(),
            Fields::Unit => vec![],
            Fields::Unnamed(fields) => {
                // Support tuple variants, especially single-field ones
                (0..fields.unnamed.len())
                    .map(|i| {
                        syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site())
                    })
                    .collect()
            }
        };

        // Create the match pattern for the variant.
        let match_pattern = match fields {
            Fields::Named(_) => quote! { Self::#variant_ident { #(#field_names),* } },
            Fields::Unit => quote! { Self::#variant_ident },
            Fields::Unnamed(fields) => {
                if fields.unnamed.len() == 1 {
                    quote! { Self::#variant_ident(#(#field_names),*) }
                } else {
                    quote! { Self::#variant_ident(#(#field_names),*) }
                }
            }
        };

        // Generate the body of the match arm based on the attributes.
        let arm_body = if let Some(attr) = asm_code_attr {
            // Handle `#[asm_code(...)]` with Rust code
            match &attr.meta {
                syn::Meta::List(meta_list) => {
                    let code_string = meta_list.tokens.to_string();
                    // 去掉两端的引号（如果有的话）
                    let code_content = if code_string.starts_with('"') && code_string.ends_with('"') {
                        &code_string[1..code_string.len()-1]
                    } else {
                        &code_string
                    };
                    
                    // 处理转义字符，将 \n 替换为真实换行，将 \\{ 和 \\} 替换为 { 和 }
                    let processed_code = code_content
                        .replace("\\n", "\n")
                        .replace("\\{", "{")
                        .replace("\\}", "}")
                        .replace("\\\"", "\"");
                    
                    // 尝试解析为 Rust 表达式
                    match processed_code.parse::<TokenStream>() {
                        Ok(code_tokens) => {
                            // 对于 Rust 代码，直接执行并返回结果
                            quote! {
                                write!(f, "{}", #code_tokens)
                            }
                        }
                        Err(_) => {
                            // 如果解析失败，尝试将其作为格式字符串处理
                            // 转义花括号以避免格式字符串错误
                            quote! {
                                write!(f, #processed_code)
                            }
                        }
                    }
                }
                _ => {
                    quote! {
                        compile_error!("malformed `asm_code` attribute. Expected `#[asm_code(...)]`")
                    }
                }
            }
        } else if let Some(attr) = asm_attr {
            // Handle `#[asm("...")]` with format string
            match attr.parse_args::<syn::LitStr>() {
                Ok(lit_str) => {
                    let format_str = lit_str.value();
                    
                    // 检查格式字符串中是否包含字段名占位符
                    if format_str.contains('{') {
                        quote! {
                            {
                                let result = format!(#format_str, #(#field_names = #field_names),*);
                                write!(f, "{}", result)
                            }
                        }
                    } else {
                        // 如果不包含占位符，直接输出字符串
                        quote! {
                            write!(f, #format_str)
                        }
                    }
                }
                Err(_) => {
                    // Handle malformed `#[asm]` attribute.
                    quote! {
                        compile_error!("malformed `asm` attribute. Expected `#[asm(\"...\")]`")
                    }
                }
            }
        } else {
            // Handle different field types for default formatting
            match fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    // For single-field tuple variants, delegate to the inner field's Display
                    let field_name = &field_names[0];
                    quote! {
                        write!(f, "{}", #field_name)
                    }
                }
                Fields::Unnamed(_) => {
                    // For multi-field tuple variants, use default format with field values
                    let default_str = variant.ident.to_string().to_lowercase().replace('_', ".");
                    quote! {
                        write!(f, #default_str)
                    }
                }
                _ => {
                    // Default formatting for named and unit variants
                    let default_str = variant.ident.to_string().to_lowercase().replace('_', ".");
                    quote! {
                        write!(f, #default_str)
                    }
                }
            }
        };

        quote! {
            #match_pattern => {
                #arm_body
            }
        }
    });

    // Generate the `impl std::fmt::Display` block.
    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#match_arms),*
                }
            }
        }
    }
}
