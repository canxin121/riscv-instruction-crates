use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

/// 为 Random trait 生成实现代码
pub fn impl_random_derive(input_ast: &DeriveInput) -> TokenStream {
    let name: &Ident = &input_ast.ident;
    let random_trait_ident = quote!(Random);
    let generics = &input_ast.generics;
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    match &input_ast.data {
        Data::Enum(data_enum) => {
            let variants_data = &data_enum.variants;

            if variants_data.is_empty() {
                return syn::Error::new_spanned(
                    name,
                    "Cannot derive Random for an enum with no variants",
                )
                .to_compile_error();
            }

            let num_variants = variants_data.len();
            let mut match_arms = TokenStream::new();

            for (idx, variant) in variants_data.iter().enumerate() {
                let variant_ident: &Ident = &variant.ident;
                let mut field_initializers = TokenStream::new();

                match &variant.fields {
                    Fields::Named(fields_named) => {
                        for field in fields_named.named.iter() {
                            let field_name_ident = field
                                .ident
                                .as_ref()
                                .expect("Named field must have an identifier");
                            let field_ty = &field.ty;
                            field_initializers.extend(quote! {
                                #field_name_ident: <#field_ty as #random_trait_ident>::random_with_rng(rng),
                            });
                        }
                        match_arms.extend(quote! {
                            #idx => #name::#variant_ident { #field_initializers },
                        });
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        let mut tuple_fields = TokenStream::new();
                        for field in fields_unnamed.unnamed.iter() {
                            let field_ty = &field.ty;
                            tuple_fields.extend(quote! {
                                <#field_ty as #random_trait_ident>::random_with_rng(rng),
                            });
                        }
                        match_arms.extend(quote! {
                            #idx => #name::#variant_ident(#tuple_fields),
                        });
                    }
                    Fields::Unit => {
                        match_arms.extend(quote! {
                            #idx => #name::#variant_ident,
                        });
                    }
                }
            }

            quote! {
                impl #impl_generics #random_trait_ident for #name #ty_generics #where_clause {
                    type Output = Self;

                    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
                        let variant_idx = rng.random_range(0..#num_variants);
                        match variant_idx {
                            #match_arms
                            _=> unreachable!("Variant index out of bounds, this should never happen if num_variants is correct."),
                        }
                    }
                }
            }
        }
        Data::Struct(_data_struct) => {
            quote! {
                impl #impl_generics #random_trait_ident for #name #ty_generics #where_clause {
                    type Output = Self;

                    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
                        const MAX_ATTEMPTS: usize = 10000;
                        for _ in 0..MAX_ATTEMPTS {
                            let random_value = if let Some(multiple) = Self::MULTIPLE_OF {
                                let min_multiple = if Self::MIN >= 0 {
                                    (Self::MIN + multiple - 1) / multiple
                                } else {
                                    Self::MIN / multiple
                                };
                                let max_multiple = Self::MAX / multiple;
                                if min_multiple <= max_multiple {
                                    let multiple_factor = rng.random_range(min_multiple..=max_multiple);
                                    multiple_factor * multiple
                                } else {
                                    rng.random_range(Self::MIN..=Self::MAX)
                                }
                            } else {
                                rng.random_range(Self::MIN..=Self::MAX)
                            };

                            if Self::NOT_ZERO && random_value == 0 {
                                continue;
                            }

                            if Self::FORBIDDEN.contains(&random_value) {
                                continue;
                            }

                            if let Ok(instance) = Self::new(random_value) {
                                return instance;
                            }
                        }
                        panic!("Failed to generate a valid random value for {} after {} attempts. Check constraints.", 
                            stringify!(#name), MAX_ATTEMPTS);
                    }
                }
            }
        }
        Data::Union(_) => {
            syn::Error::new_spanned(name, "Random derive macro cannot be used on unions.")
                .to_compile_error()
        }
    }
}
