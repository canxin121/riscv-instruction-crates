use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields};

/// Generates the implementation for the `std::fmt::Display` trait for an enum.
/// Expects `#[asm("...")]` attributes on variants for custom formatting.
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

        // Find the `#[asm(...)]` attribute on the variant.
        let asm_attr = variant
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("asm"));

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

        // Generate the body of the match arm based on the `#[asm]` attribute.
        let arm_body = if let Some(attr) = asm_attr {
            match attr.parse_args::<syn::LitStr>() {
                Ok(lit_str) => {
                    let format_str = lit_str;
                    // Use `write!` macro to format and write to the formatter `f`.
                    quote! {
                        write!(f, #format_str, #(#field_names = #field_names),*)
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
