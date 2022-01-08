#![allow(non_snake_case)]

use darling::FromDeriveInput;
use quote::quote;

#[derive(FromDeriveInput, Default)]
#[darling(default, attributes(st_term_trait))]
struct StTermTraitArguments {
    AbstractTypeType: String,
    abstract_type_expr: Option<String>,
    is_parametric: String,
    is_type: String,
}

/// This will derive sept::st::TermTrait; trait implementation details should be given via `st_term_trait`, e.g.
/// ```
/// #[derive(sept::st::TermTrait)]
/// #[st_term_trait(AbstractTypeType = "<type>")] // Defines return type of `fn abstract_type(&self)`
/// #[st_term_trait(abstract_type_expr = "<expr>")] // Optional; default is "Self::AbstractTypeType{}"
/// #[st_term_trait(is_parametric = "<expr>")]
/// #[st_term_trait(is_type = "<expr>")]
/// pub struct FancyTerm;
/// ```
#[proc_macro_derive(StTermTrait, attributes(st_term_trait))]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input);
    let term_trait_arguments = StTermTraitArguments::from_derive_input(&input).expect("Wrong arguments");
    #[allow(unused_variables)]
    let StTermTraitArguments { AbstractTypeType, is_parametric, is_type, abstract_type_expr } = term_trait_arguments;
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
        },
        None => quote! {
            fn abstract_type(&self) -> Self::AbstractTypeType {
                Self::AbstractTypeType{}
            }
        }
    };

    let output = quote! {
        impl TermTrait for #ident {
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
