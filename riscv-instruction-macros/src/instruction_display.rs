use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// 为枚举实现 Display trait
/// 支持 `#[asm("...")]` 和 `#[asm_code(...)]` 属性进行自定义格式化
pub fn impl_instruction_display(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let variants = match &ast.data {
        Data::Enum(data) => &data.variants,
        _ => return quote! { compile_error!("InstructionDisplay can only be derived for enums"); },
    };

    let match_arms = variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let fields = &variant.fields;

        let asm_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("asm"));
            
        let asm_code_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("asm_code"));

        let field_names: Vec<_> = match fields {
            Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| f.ident.as_ref().unwrap().clone())
                .collect(),
            Fields::Unit => vec![],
            Fields::Unnamed(fields) => {
                (0..fields.unnamed.len())
                    .map(|i| {
                        syn::Ident::new(&format!("field_{}", i), proc_macro2::Span::call_site())
                    })
                    .collect()
            }
        };

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

        let arm_body = if let Some(attr) = asm_code_attr {
            match &attr.meta {
                syn::Meta::List(meta_list) => {
                    let code_string = meta_list.tokens.to_string();
                    let code_content = if code_string.starts_with('"') && code_string.ends_with('"') {
                        &code_string[1..code_string.len()-1]
                    } else {
                        &code_string
                    };
                    
                    let processed_code = code_content
                        .replace("\\n", "\n")
                        .replace("\\{", "{")
                        .replace("\\}", "}")
                        .replace("\\\"", "\"");
                    
                    match processed_code.parse::<TokenStream>() {
                        Ok(code_tokens) => {
                            quote! {
                                write!(f, "{}", #code_tokens)
                            }
                        }
                        Err(_) => {
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
            match attr.parse_args::<syn::LitStr>() {
                Ok(lit_str) => {
                    let format_str = lit_str.value();
                    
                    if format_str.contains('{') {
                        quote! {
                            {
                                let result = format!(#format_str, #(#field_names = #field_names),*);
                                write!(f, "{}", result)
                            }
                        }
                    } else {
                        quote! {
                            write!(f, #format_str)
                        }
                    }
                }
                Err(_) => {
                    quote! {
                        compile_error!("malformed `asm` attribute. Expected `#[asm(\"...\")]`")
                    }
                }
            }
        } else {
            match fields {
                Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                    let field_name = &field_names[0];
                    quote! {
                        write!(f, "{}", #field_name)
                    }
                }
                Fields::Unnamed(_) => {
                    let default_str = variant.ident.to_string().to_lowercase().replace('_', ".");
                    quote! {
                        write!(f, #default_str)
                    }
                }
                _ => {
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
