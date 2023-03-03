mod array;
mod array_type;
mod r#bool;
mod bool_term;
mod bool_type;
mod empty_type;
mod deserializable;
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
mod non_parametric_term_code;
mod non_parametric_term_trait;
mod serializable;
mod stringifiable;
mod r#struct;
mod struct_type;
mod term;
mod term_trait;
mod test_values;
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

pub use crate::st::{
    array::Array,
    array_type::ArrayType,
    r#bool::Bool,
    bool_term::BoolTerm,
    bool_type::BoolType,
    deserializable::{Deserializable, read_len},
    empty_type::EmptyType,
    r#false::False,
    false_type::FalseType,
    float::{Float32, Float64},
    float_term::{Float32Term, Float64Term},
    float_type::{Float32Type, Float64Type},
    global_sym_ref::GlobalSymRef,
    global_sym_ref_type::GlobalSymRefType,
    inhabits_trait::Inhabits,
    int::{Sint8, Sint16, Sint32, Sint64, Uint8, Uint16, Uint32, Uint64},
    int_term::{Sint8Term, Sint16Term, Sint32Term, Sint64Term, Uint8Term, Uint16Term, Uint32Term, Uint64Term},
    int_type::{Sint8Type, Sint16Type, Sint32Type, Sint64Type, Uint8Type, Uint16Type, Uint32Type, Uint64Type},
    local_sym_ref::LocalSymRef,
    local_sym_ref_type::LocalSymRefType,
    non_parametric_term_code::NonParametricTermCode,
    non_parametric_term_trait::NonParametricTermTrait,
    serializable::{Serializable, SerializedTopLevelCode, write_len},
    stringifiable::Stringifiable,
    r#struct::Struct,
    struct_type::StructType,
    term::Term,
    term_trait::TermTrait,
    test_values::TestValues,
    tuple::Tuple,
    tuple_type::TupleType,
    r#type::Type,
    type_trait::TypeTrait,
    r#true::True,
    true_type::TrueType,
    utf8_string::Utf8String,
    utf8_string_term::Utf8StringTerm,
    utf8_string_type::Utf8StringType,
    void::Void,
    void_type::VoidType,
};

// Trait derivation proc macros
pub use sept_derive::StNonParametricTermTrait as NonParametricTermTrait;
pub use sept_derive::StTermTrait as TermTrait;
pub use sept_derive::StTypeTrait as TypeTrait;

// // Tuple of NonParametricTerms.  This is different than any dy::Runtime-based list, since it
// // doesn't involve dy::Value at all.
// lazy_static::lazy_static! {
//     /// This is the static singleton global SymbolTable.
//     pub static ref NON_PARAMETRIC_TERMS: (Void, VoidType) = (Void, VoidType);
// }

/// This is a bit of a hack, but is a practical way to iterate over the tuple of non-parametric
/// terms in a strongly typed way, without resorting to dy::Value.
#[macro_export]
macro_rules! for_each_non_parametric_term {
    ($T:ident, $e:expr) => {
        (
            { type T = $crate::st::Term;                  $e },
//             { type T = $crate::st::NonParametricTerm;     $e },
//             { type T = $crate::st::ParametricTerm;        $e },
            { type T = $crate::st::Type;                  $e },
//             { type T = $crate::st::NonType;                  $e },
//             { type T = $crate::st::NonParametricType;                  $e },
//             { type T = $crate::st::ParametricType;                  $e },
            { type T = $crate::st::Void;                  $e },
            { type T = $crate::st::True;                  $e },
            { type T = $crate::st::False;                  $e },
            { type T = $crate::st::VoidType;                  $e },
            { type T = $crate::st::TrueType;                  $e },
            { type T = $crate::st::FalseType;                  $e },
            { type T = $crate::st::EmptyType;                  $e },
//             { type T = $crate::st::FormalTypeOf;                  $e },
            { type T = $crate::st::Bool;                  $e },
            { type T = $crate::st::Sint8;                  $e },
            { type T = $crate::st::Sint16;                  $e },
            { type T = $crate::st::Sint32;                  $e },
            { type T = $crate::st::Sint64;                  $e },
            { type T = $crate::st::Uint8;                  $e },
            { type T = $crate::st::Uint16;                  $e },
            { type T = $crate::st::Uint32;                  $e },
            { type T = $crate::st::Uint64;                  $e },
            { type T = $crate::st::Float32;                  $e },
            { type T = $crate::st::Float64;                  $e },
//             { type T = $crate::st::AsciiChar;                  $e },
            { type T = $crate::st::BoolType;                       $e },
            { type T = $crate::st::Sint8Type;                      $e },
            { type T = $crate::st::Sint16Type;                     $e },
            { type T = $crate::st::Sint32Type;                     $e },
            { type T = $crate::st::Sint64Type;                     $e },
            { type T = $crate::st::Uint8Type;                      $e },
            { type T = $crate::st::Uint16Type;                     $e },
            { type T = $crate::st::Uint32Type;                     $e },
            { type T = $crate::st::Uint64Type;                     $e },
            { type T = $crate::st::Float32Type;                    $e },
            { type T = $crate::st::Float64Type;                    $e },
//             { type T = $crate::st::AsciiCharType;                  $e },
            { type T = $crate::st::Utf8String;                  $e },
            { type T = $crate::st::Utf8StringType;                  $e },
//             { type T = $crate::st::Sint;                  $e },
//             { type T = $crate::st::Uint;                  $e },
//             { type T = $crate::st::Float;                  $e },
//             { type T = $crate::st::Pod;                  $e },
//             { type T = $crate::st::SintType;                  $e },
//             { type T = $crate::st::UintType;                  $e },
//             { type T = $crate::st::FloatType;                  $e },
//             { type T = $crate::st::PodType;                  $e },
//             { type T = $crate::st::Union;                  $e },
//             { type T = $crate::st::Intersection;                  $e },
//             { type T = $crate::st::Negation;                  $e },
//             { type T = $crate::st::Difference;                  $e },
//             { type T = $crate::st::UnionType;                  $e },
//             { type T = $crate::st::IntersectionType;                  $e },
//             { type T = $crate::st::NegationType;                  $e },
//             { type T = $crate::st::DifferenceType;                  $e },
            { type T = $crate::st::ArrayType;                  $e },
//             { type T = $crate::st::ArrayES;                  $e },
//             { type T = $crate::st::ArrayE;                  $e },
//             { type T = $crate::st::ArrayS;                  $e },
            { type T = $crate::st::Array;                  $e },
//             { type T = $crate::st::OrderedMapType;                  $e },
//             { type T = $crate::st::OrderedMapDC;                  $e },
//             { type T = $crate::st::OrderedMapD;                  $e },
//             { type T = $crate::st::OrderedMapC;                  $e },
//             { type T = $crate::st::OrderedMap;                  $e },
            { type T = $crate::st::TupleType;                  $e },
            { type T = $crate::st::Tuple;                  $e },
            { type T = $crate::st::StructType;                  $e },
            { type T = $crate::st::Struct;                  $e },
//             { type T = $crate::st::MemRefType;                  $e },
//             { type T = $crate::st::MemRef;                  $e },
            { type T = $crate::st::GlobalSymRefType;                  $e },
            { type T = $crate::st::GlobalSymRef;                  $e },
            { type T = $crate::st::LocalSymRefType;                  $e },
            { type T = $crate::st::LocalSymRef;                  $e },
//             { type T = $crate::st::PlaceholderType;                  $e },
//             { type T = $crate::st::Placeholder;                  $e },
//             { type T = $crate::st::FreevarType;                  $e },
//             { type T = $crate::st::Freevar;                  $e },
//             { type T = $crate::st::OutputType;                  $e },
//             { type T = $crate::st::Output;                  $e },
//             { type T = $crate::st::ClearOutputType;                  $e },
//             { type T = $crate::st::ClearOutput;                  $e },
//             { type T = $crate::st::EndOfFileType;                  $e },
//             { type T = $crate::st::EndOfFile;                  $e },
//             { type T = $crate::st::RequestSyncInputType;                  $e },
//             { type T = $crate::st::RequestSyncInput;                  $e },
        )
    };
}
