use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned as _,
    Data, DeriveInput, LitBool, LitStr, Meta, Token,
};

/// Represents the arguments parsed from the `#[validated(...)]` attribute.
#[derive(Default)]
pub struct ValidatedArgs {
    min: Option<LitStr>,
    max: Option<LitStr>,
    name: Option<LitStr>,
    display: Option<LitStr>,
    not_zero: Option<LitBool>,
}

/// Implements parsing logic for the `ValidatedArgs` struct from attribute tokens.
impl Parse for ValidatedArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut args = Self::default();

        // Parse attribute arguments as a comma-separated list of `syn::Meta`.
        // `syn::Meta` can be a Path (`not_zero`), NameValue (`min = "..."`), or List (`key(...)`).
        let metas = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

        for meta in metas {
            match meta {
                // Handle NameValue attributes like `min = "..."`, `max = "..."`, etc.
                Meta::NameValue(nv) => {
                    if nv.path.is_ident("min") {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                args.min = Some(lit_str.clone());
                            }
                        }
                    } else if nv.path.is_ident("max") {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                args.max = Some(lit_str.clone());
                            }
                        }
                    } else if nv.path.is_ident("name") {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                args.name = Some(lit_str.clone());
                            }
                        }
                    } else if nv.path.is_ident("display") {
                        if let syn::Expr::Lit(expr_lit) = &nv.value {
                            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                                args.display = Some(lit_str.clone());
                            }
                        }
                    }
                }
                // Handle Path attributes like `not_zero`.
                Meta::Path(path) => {
                    if path.is_ident("not_zero") {
                        args.not_zero = Some(LitBool::new(true, path.span()));
                    }
                }
                _ => {
                    // Reject other Meta types (e.g., List).
                    return Err(syn::Error::new_spanned(
                        meta,
                        "Unsupported attribute format",
                    ));
                }
            }
        }

        Ok(args)
    }
}

/// Generates the implementation for the `ValidatedValue` and `Display` traits.
pub fn impl_validated_value(ast: &DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // Split generics for use in the impl block.
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // Parse arguments from the `#[validated(...)]` attribute.
    let attr = ast
        .attrs
        .iter()
        .find(|a| a.path().is_ident("validated"))
        .expect("`validated` attribute is required for deriving `ValidatedValue`");
    let args: ValidatedArgs = attr
        .parse_args()
        .expect("Failed to parse `validated` attribute arguments");

    // Extract the inner type from the newtype struct.
    let inner_type = if let Data::Struct(s) = &ast.data {
        if let syn::Fields::Unnamed(f) = &s.fields {
            if f.unnamed.len() == 1 {
                f.unnamed.first().unwrap().ty.clone()
            } else {
                panic!("DeriveValidatedValue only supports newtype structs with one field");
            }
        } else {
            panic!("DeriveValidatedValue only supports newtype structs");
        }
    } else {
        panic!("DeriveValidatedValue can only be used on structs");
    };

    // Parse min and max expressions from attribute arguments.
    let min_expr: TokenStream = args
        .min
        .as_ref()
        .expect("`min` is required")
        .value()
        .parse()
        .unwrap();
    let max_expr: TokenStream = args
        .max
        .as_ref()
        .expect("`max` is required")
        .value()
        .parse()
        .unwrap();

    // Determine the name to use in error messages, defaulting to snake_case of the struct name.
    let name_in_msg = args.name.as_ref().map(|n| n.value()).unwrap_or_else(|| {
        name.to_string().to_snake_case().replace('_', " ")
    });
    let error_msg = format!("{} {{}} exceeds range ({{}} to {{}})", name_in_msg);

    // Generate code for the `not_zero` check if the attribute is present.
    let zero_check = if args.not_zero.as_ref().map_or(false, |b| b.value) {
        let err_msg = format!("{} cannot be zero", name_in_msg);
        quote! {
            if value == 0 {
                return Err(#err_msg.to_string());
            }
        }
    } else {
        quote! {}
    };

    // Determine the format string for the `Display` implementation.
    let display_format = args
        .display
        .as_ref()
        .map(|d| d.value())
        .unwrap_or("{}".to_string());

    // Generate the `impl` blocks for `ValidatedValue` and `Display`.
    quote! {
        // Implement the ValidatedValue trait.
        impl #impl_generics ValidatedValue<#inner_type> for #name #ty_generics #where_clause {
            const MIN: #inner_type = #min_expr;
            const MAX: #inner_type = #max_expr;
            type Error = String;

            fn new(value: #inner_type) -> Result<Self, Self::Error> {
                #zero_check
                if value >= Self::MIN && value <= Self::MAX {
                    Ok(Self(value))
                } else {
                    Err(format!(
                        #error_msg,
                        value,
                        Self::MIN,
                        Self::MAX
                    ))
                }
            }

            fn get(&self) -> #inner_type {
                self.0
            }

            fn set(&mut self, value: #inner_type) -> Result<(), Self::Error> {
                *self = Self::new(value)?;
                Ok(())
            }
        }

        // Implement the Display trait.
        impl #impl_generics std::fmt::Display for #name #ty_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, #display_format, self.0)
            }
        }
    }
}
