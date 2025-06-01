use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Ident};

/// Generates the implementation for the `Random` trait based on the input AST.
pub fn impl_random_derive(input_ast: &DeriveInput) -> TokenStream {
    let name: &Ident = &input_ast.ident;

    // Assume the `Random` trait is visible in the deriving context.
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

            // Generate match arms for each enum variant.
            for (idx, variant) in variants_data.iter().enumerate() {
                let variant_ident: &Ident = &variant.ident;
                let mut field_initializers = TokenStream::new();

                match &variant.fields {
                    Fields::Named(fields_named) => {
                        // Handle named fields by calling `random_with_rng()` on each field type.
                        for field in fields_named.named.iter() {
                            let field_name_ident = field
                                .ident
                                .as_ref()
                                .expect("Named field must have an identifier");
                            let field_ty = &field.ty;
                            // Assume field types also implement Random.
                            field_initializers.extend(quote! {
                                #field_name_ident: <#field_ty as #random_trait_ident>::random_with_rng(rng),
                            });
                        }
                        match_arms.extend(quote! {
                            #idx => #name::#variant_ident { #field_initializers },
                        });
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        // Handle unnamed fields (tuples) by calling `random_with_rng()` for each element.
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
                        // Handle unit variants.
                        match_arms.extend(quote! {
                            #idx => #name::#variant_ident,
                        });
                    }
                }
            }

            // Generate the impl block for enums.
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
            // Generate Random implementation for structs, assuming they implement ValidatedValue.
            quote! {
                impl #impl_generics #random_trait_ident for #name #ty_generics #where_clause {
                    type Output = Self;

                    fn random_with_rng<R: rand::Rng>(rng: &mut R) -> Self::Output {
                        // 对于有禁用值和其他约束的类型，需要特殊处理
                        const MAX_ATTEMPTS: usize = 10000; // 增加最大尝试次数
                        for _ in 0..MAX_ATTEMPTS {
                            let random_value = if let Some(multiple) = Self::MULTIPLE_OF {
                                // Generate a value that is a multiple of the required value
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

                            // Check NOT_ZERO constraint
                            if Self::NOT_ZERO && random_value == 0 {
                                continue;
                            }

                            // Check forbidden values constraint
                            if Self::FORBIDDEN.contains(&random_value) {
                                continue;
                            }

                            // Use Self::new to create the instance, which will check all constraints
                            if let Ok(instance) = Self::new(random_value) {
                                return instance;
                            }
                        }
                        // Fallback: if we can't generate a valid value after MAX_ATTEMPTS,
                        // panic with a descriptive error
                        panic!("Failed to generate a valid random value for {} after {} attempts. Check constraints.", 
                            stringify!(#name), MAX_ATTEMPTS);
                    }
                }
            }
        }
        Data::Union(_) => {
            // Random derive is not supported for unions.
            syn::Error::new_spanned(name, "Random derive macro cannot be used on unions.")
                .to_compile_error()
        }
    }
}
