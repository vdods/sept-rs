mod array;
mod array_type;
mod r#bool;
mod bool_term;
mod bool_type;
mod empty_type;
mod r#false;
mod false_type;
mod float;
mod float_term;
mod float_type;
mod global_sym_ref;
mod global_sym_ref_type;
mod inhabits_trait;
mod int;
mod int_term;
mod int_type;
mod local_sym_ref;
mod local_sym_ref_type;
mod non_parametric_term_trait;
mod stringify_trait;
mod r#struct;
mod struct_type;
mod term;
mod term_trait;
mod r#true;
mod true_type;
mod tuple;
mod tuple_type;
mod r#type;
mod type_trait;
mod utf8_string;
mod utf8_string_term;
mod utf8_string_type;
mod void;
mod void_type;

pub use anyhow::{Error, Result};
pub use crate::st::{
    array::Array,
    array_type::ArrayType,
    r#bool::Bool,
    bool_term::{},
    bool_type::BoolType,
    empty_type::EmptyType,
    r#false::False,
    false_type::FalseType,
    float::{Float32, Float64},
    float_term::{},
    float_type::{Float32Type, Float64Type},
    global_sym_ref::GlobalSymRef,
    global_sym_ref_type::GlobalSymRefType,
    inhabits_trait::Inhabits,
    int::{Sint8, Sint16, Sint32, Sint64, Uint8, Uint16, Uint32, Uint64},
    int_term::{},
    int_type::{Sint8Type, Sint16Type, Sint32Type, Sint64Type, Uint8Type, Uint16Type, Uint32Type, Uint64Type},
    local_sym_ref::LocalSymRef,
    local_sym_ref_type::LocalSymRefType,
    non_parametric_term_trait::NonParametricTermTrait,
    stringify_trait::Stringify,
    r#struct::Struct,
    struct_type::StructType,
    term::Term,
    term_trait::TermTrait,
    tuple::Tuple,
    tuple_type::TupleType,
    r#type::Type,
    type_trait::TypeTrait,
    r#true::True,
    true_type::TrueType,
    utf8_string::Utf8String,
    utf8_string_term::{},
    utf8_string_type::Utf8StringType,
    void::Void,
    void_type::VoidType,
};

// Trait derivation proc macros
pub use sept_derive::StTermTrait as TermTrait;
pub use sept_derive::StTypeTrait as TypeTrait;
