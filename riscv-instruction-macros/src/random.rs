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
                        // Handle named fields by calling `random()` on each field type.
                        for field in fields_named.named.iter() {
                            let field_name_ident = field
                                .ident
                                .as_ref()
                                .expect("Named field must have an identifier");
                            let field_ty = &field.ty;
                            // Assume field types also implement Random.
                            field_initializers.extend(quote! {
                                #field_name_ident: <#field_ty as #random_trait_ident>::random(),
                            });
                        }
                        match_arms.extend(quote! {
                            #idx => #name::#variant_ident { #field_initializers },
                        });
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        // Handle unnamed fields (tuples) by calling `random()` for each element.
                        let mut tuple_fields = TokenStream::new();
                        for field in fields_unnamed.unnamed.iter() {
                            let field_ty = &field.ty;
                            tuple_fields.extend(quote! {
                                <#field_ty as #random_trait_ident>::random(),
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
                    fn random() -> Self {
                        use rand::Rng;
                        let mut rng = rand::thread_rng();
                        let variant_idx = rng.gen_range(0..#num_variants);
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
            // Assume the `ValidatedValue` trait is visible.

            // Generate the impl block for structs.
            quote! {
                impl #impl_generics #random_trait_ident for #name #ty_generics #where_clause {
                    fn random() -> Self {
                        use rand::Rng;
                        // Assume Self implements ValidatedValue<T>
                        // and Self::MIN and Self::MAX are T type values (T supports gen_range)
                        // and Self::new(value: T) method exists.
                        // ValidatedValue trait needs to be imported or in the same module for Self::MIN, Self::MAX, Self::new to resolve.

                        let mut rng = rand::thread_rng();
                        let random_value = rng.gen_range(Self::MIN..=Self::MAX);

                        // Use Self::new to create the instance.
                        // This is expected to always succeed as random_value is within the MIN/MAX range.
                        Self::new(random_value)
                            .expect("Failed to create new validated value from a randomly generated value within MIN/MAX bounds. Check ValidatedValue MIN/MAX, new() implementation, and ensure T is suitable for rand::Rng::gen_range.")
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
