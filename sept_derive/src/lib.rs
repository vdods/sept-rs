#![allow(non_snake_case)]

use darling::FromDeriveInput;
use quote::quote;

//
// proc_macro for deriving sept::st::TermT (it's re-exported in that crate).
//

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(st_term_t))]
struct StTermTArguments {
    AbstractTypeType: String,
    abstract_type_expr: Option<String>,
    is_parametric: String,
    is_type: String,
}

/// This will derive sept::st::TermT; trait implementation details should be given via
/// `st_term_t`, e.g.
/// ```ignore
/// #[derive(sept::st::TermT)]
/// #[st_term_t(AbstractTypeType = "<type>")] // Defines return type of `fn abstract_type(&self)`
/// #[st_term_t(abstract_type_expr = "<expr>")] // Optional; default is "Self::AbstractTypeType{}"
/// #[st_term_t(is_parametric = "<expr>")]
/// #[st_term_t(is_type = "<expr>")]
/// pub struct FancyTerm;
/// ```
#[proc_macro_derive(StTermT, attributes(st_term_t))]
pub fn derive_st_term_t(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    let term_trait_arguments =
        StTermTArguments::from_derive_input(&input).expect("Wrong arguments");
    #[allow(unused_variables)]
    let StTermTArguments {
        AbstractTypeType,
        is_parametric,
        is_type,
        abstract_type_expr,
    } = term_trait_arguments;
    let AbstractTypeType: syn::Type = syn::parse_str(&AbstractTypeType).unwrap();
    let is_parametric: syn::Expr = syn::parse_str(&is_parametric).unwrap();
    let is_type: syn::Expr = syn::parse_str(&is_type).unwrap();
    let syn::DeriveInput { ident, .. } = input;

    let abstract_type_fn = match abstract_type_expr {
        Some(abstract_type_expr) => {
            let abstract_type_expr: syn::Expr = syn::parse_str(&abstract_type_expr).unwrap();
            quote! {
                fn abstract_type(&self) -> Self::AbstractTypeType {
                    #abstract_type_expr
                }
            }
        }
        None => quote! {
            fn abstract_type(&self) -> Self::AbstractTypeType {
                Self::AbstractTypeType{}
            }
        },
    };

    let output = quote! {
        impl st::TermT for #ident {
            type AbstractTypeType = #AbstractTypeType;

            fn is_parametric(&self) -> bool {
                #is_parametric
            }
            fn is_type(&self) -> bool {
                #is_type
            }
            #abstract_type_fn
        }
    };
    output.into()
}

//
// proc_macro for deriving sept::st::NonParametricTermT (it's re-exported in that crate).
//

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(st_non_parametric_term_t))]
struct StNonParametricTermTArguments {
    code: Option<String>,
}

/// This will derive sept::st::NonParametricTermT; trait implementation details should be
/// given via `st_non_parametric_term_t`, e.g.
/// ```ignore
/// #[derive(sept::st::NonParametricTermT)]
/// #[st_non_parametric_term_t(code = "<expr>")]
/// pub struct FancyTerm;
/// ```
#[proc_macro_derive(StNonParametricTermT, attributes(st_non_parametric_term_t))]
pub fn derive_st_non_parametric_term_t(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    let non_parametric_term_trait_arguments =
        StNonParametricTermTArguments::from_derive_input(&input).expect("Wrong arguments");
    #[allow(unused_variables)]
    let StNonParametricTermTArguments { code } = non_parametric_term_trait_arguments;
    let syn::DeriveInput { ident, .. } = input;
    let non_parametric_term_code = match code {
        Some(c) => {
            let blah: syn::Expr = syn::parse_str(&c).unwrap();
            quote!(st::NonParametricTermCode::#blah)
        }
        None => {
            quote!(st::NonParametricTermCode::#ident)
        }
    };

    let output = quote! {
        impl st::NonParametricTermT for #ident {
            const IDENTIFIER: &'static str = stringify!(#ident);
            const NON_PARAMETRIC_TERM_CODE: st::NonParametricTermCode = #non_parametric_term_code;
            fn instantiate() -> Self {
                Self{}
            }
        }
    };
    output.into()
}

//
// proc_macro for deriving sept::st::TypeT (it's re-exported in that crate).
//

/// This will derive sept::st::TypeT, which for now has no additional attributes, e.g.
/// ```ignore
/// #[derive(sept::st::TypeT)]
/// pub struct FancyType;
/// ```
#[proc_macro_derive(StTypeT)]
pub fn derive_st_type_t(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    let syn::DeriveInput { ident, .. } = input;

    let output = quote! {
        impl st::TypeT for #ident {}
    };
    output.into()
}

//
// proc_macro for deriving sept::dy::IntoValueT (it's re-exported in that crate).
//

/// This will derive sept::dy::IntoValueT, which for now has no additional attributes, e.g.
/// ```ignore
/// #[derive(sept::dy::IntoValueT)]
/// pub struct FancyType;
/// ```
#[proc_macro_derive(DyIntoValueT)]
pub fn derive_dy_into_value_t(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    let syn::DeriveInput { ident, .. } = input;

    let output = quote! {
        impl dy::IntoValueT for #ident {}
    };
    output.into()
}
