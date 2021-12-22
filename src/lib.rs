// #![feature(adt_const_params)] -- TODO: Would be nice to use this, so that enums can be used as const generic params

mod array;
mod array_term;
mod array_type;
mod r#bool;
mod bool_term;
mod bool_type;
mod dyn_np_term;
mod empty_type;
mod r#false;
mod false_type;
mod float;
mod float_term;
mod float_type;
mod inhabits_trait;
mod int;
mod int_term;
mod int_type;
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
    array::{ARRAY, Array},
    array_term::ArrayTerm,
    array_type::{ARRAY_TYPE, ArrayType},
    r#bool::{BOOL, Bool},
    bool_term::{},
    bool_type::{BOOL_TYPE, BoolType},
    dyn_np_term::DynNPTerm,
    empty_type::{EMPTY_TYPE, EmptyType},
    r#false::{FALSE, False},
    false_type::{FALSE_TYPE, FalseType},
    float::{FloatN, FLOAT32, Float32, FLOAT64, Float64},
    float_term::{},
    float_type::{FloatNType, FLOAT32_TYPE, Float32Type, FLOAT64_TYPE, Float64Type},
    inhabits_trait::Inhabits,
    int::{IntN, SIGNED, SINT8, Sint8, SINT16, Sint16, SINT32, Sint32, SINT64, Sint64, UNSIGNED, UINT8, Uint8, UINT16, Uint16, UINT32, Uint32, UINT64, Uint64},
    int_term::{},
    int_type::{IntNType, SINT8_TYPE, Sint8Type, SINT16_TYPE, Sint16Type, SINT32_TYPE, Sint32Type, SINT64_TYPE, Sint64Type, UINT8_TYPE, Uint8Type, UINT16_TYPE, Uint16Type, UINT32_TYPE, Uint32Type, UINT64_TYPE, Uint64Type},
    non_parametric_term_trait::NonParametricTermTrait,
    runtime::{BinaryPredicate, RUNTIME, Runtime, StringifyFn, UnaryPredicate},
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
