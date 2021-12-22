mod r#bool;
mod bool_term;
mod bool_type;
mod dyn_np_term;
mod empty_type;
mod r#false;
mod false_type;
mod inhabits_trait;
mod non_parametric_term_trait;
mod runtime;
mod stringify_trait;
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
    empty_type::{EMPTY_TYPE, EmptyType},
    r#false::{FALSE, False},
    false_type::{FALSE_TYPE, FalseType},
    inhabits_trait::Inhabits,
    non_parametric_term_trait::NonParametricTermTrait,
    runtime::{BinaryPredicate, Runtime, StringifyFn, UnaryPredicate},
    stringify_trait::Stringify,
    term::{TERM, Term},
    term_trait::TermTrait,
    r#type::{TYPE, Type},
    type_trait::TypeTrait,
    r#true::{TRUE, True},
    true_type::{TRUE_TYPE, TrueType},
    void::{VOID, Void},
    void_type::{VOID_TYPE, VoidType},
};
