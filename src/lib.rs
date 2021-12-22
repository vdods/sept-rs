mod r#bool;
mod bool_term;
mod bool_type;
mod dyn_np_term;
mod r#false;
mod false_type;
mod non_parametric_term_trait;
mod term;
mod term_trait;
mod r#true;
mod true_type;
mod r#type;
mod type_trait;
mod void;
mod void_type;

pub use anyhow::{Error, Result};
pub use crate::{
    r#bool::{BOOL, Bool},
    bool_term::{},
    bool_type::{BOOL_TYPE, BoolType},
    dyn_np_term::DynNPTerm,
    r#false::{FALSE, False},
    false_type::{FALSE_TYPE, FalseType},
    non_parametric_term_trait::NonParametricTermTrait,
    term::{TERM, Term},
    term_trait::TermTrait,
    r#type::{TYPE, Type},
    type_trait::TypeTrait,
    r#true::{TRUE, True},
    true_type::{TRUE_TYPE, TrueType},
    void::{VOID, Void},
    void_type::{VOID_TYPE, VoidType},
};
