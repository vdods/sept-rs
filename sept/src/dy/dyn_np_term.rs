// use crate::{
//     dy,
//     st::{
//         ARRAY, ARRAY_TYPE,
//         BOOL, BOOL_TYPE, EMPTY_TYPE, FALSE, FALSE_TYPE, FLOAT32, FLOAT32_TYPE, FLOAT64, FLOAT64_TYPE,
//         NonParametricTermTrait, SINT8, SINT8_TYPE, SINT16, SINT16_TYPE, SINT32, SINT32_TYPE, SINT64, SINT64_TYPE,
//         Stringify, TERM, TermTrait, TRUE, TRUE_TYPE, TYPE,
//         UINT8, UINT8_TYPE, UINT16, UINT16_TYPE, UINT32, UINT32_TYPE, UINT64, UINT64_TYPE, VOID, VOID_TYPE,
//     },
// };

// The repr(u8) attribute is to be compatible with the C++ implementation.
// NOTE: TermTrait and all the other things like Stringify are not being implemented
// here, as this enum is simply meant for serialization representation purposes.  In
// deserialization, Values containing the "real" terms would be used instead, so that
// there isn't a need to check for multiple alternate representations of various types.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DynNPTerm {
    // The most basic Types.
    Term = 0, // Literally everything is a Term (this could be called Any)
    NonParametricTerm, // Inhabitants are Terms requiring no parameters to instantiate (all members of the DynNPTerm enum)
    ParametricTerm, // Inhabitants are Terms requiring parameters to instantiate, e.g. 10.25, 'x', 999000

    // Type Types.
    Type, // A Term which has an inhabitation predicate.
    NonType, // A Term which is not a Type (this could be called Value)
    NonParametricType, // Inhabitants are types requiring no parameters to instantiate (all members of the NPType enum)
    ParametricType, // Inhabitants are types requiring parameters to instantiate, e.g. ArrayESTerm_c(T,N)

    // NonParametricTerm && NonType
    Void, // Void is a NonType that conveys no information
    True, // The truthier of the two inhabitants of Bool
    False, // The lying inhabitant of Bool

    // A few natural Types.
    VoidType, // Sole inhabitant is Void
    TrueType, // Sole inhabitant is True
    FalseType, // Sole inhabitant is False
    EmptyType, // A Type defined to have no inhabitants
    FormalTypeOf, // Constructs FormalTypeOf(x) for any term x.  Sole inhabitant of FormalTypeOf(x) is x.

    // POD Types
    Bool, // Isomorphic to Union(TrueType,FalseType)
    Sint8,
    Sint16,
    Sint32,
    Sint64,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Float32,
    Float64,
//     AsciiChar, // TODO: Add UnicodeChar later and whatever else -- TODO: Maybe Ascii should be an abstract type

    // POD Type Types
    BoolType, // Sole inhabitant is Bool
    Sint8Type, // Sole inhabitant is Sint8
    Sint16Type, // Sole inhabitant is Sint16
    Sint32Type, // Sole inhabitant is Sint32
    Sint64Type, // Sole inhabitant is Sint64
    Uint8Type, // Sole inhabitant is Uint8
    Uint16Type, // Sole inhabitant is Uint16
    Uint32Type, // Sole inhabitant is Uint32
    Uint64Type, // Sole inhabitant is Uint64
    Float32Type, // Sole inhabitant is Float32
    Float64Type, // Sole inhabitant is Float64
//     AsciiCharType, // Sole inhabitant is AsciiChar

    // Other Types related to POD Types
    SintType, // Isomorphic to Union(Sint8Type,Sint16Type,Sint32Type,Sint64Type)
    Sint, // Isomorphic to Union(Sint8,Sint16,Sint32,Sint64)
    UintType, // Isomorphic to Union(Uint8Type,Uint16Type,Uint32Type,Uint64Type)
    Uint, // Isomorphic to Union(Uint8,Uint16,Uint32,Uint64)
    FloatType, // Isomorphic to Union(Float32Type,Float64Type)
    Float, // Isomorphic to Union(Float32,Float64)
    // TODO: Add CHAR types
    PodType, // Isomorphic to Union(BoolType,SintType,UintType,FloatType) (TODO: Somehow add Pod as an inhabitant)
    Pod, // Isomorphic to Union(Bool,Sint,Uint,Float).  Inhabitants are POD values.  Pod : PodType.
    // TODO: Add stuff like Positive, Negative, NonPositive, NonNegative, Zero

    UnionType, // Inhabitant is Union.
    Union, // Inhabitants have the form Union(T1,...,TN) -- implemented by class UnionTerm_c
    Intersection, // Inhabitants have the form Intersection(T1,...,TN)
    Negation, // Inhabitants have the form Negation(T)
    Difference, // Inhabitants have the form Difference(T,U1,...,UN)

    // TODO: UnionType, IntersectionType, etc.

    ArrayType, // Inhabitants are ArrayES, ArrayE, ArrayS, Array.
    ArrayES, // Inhabitants have the form ArrayES(T,N) -- implemented by class ArrayESTerm_c
    ArrayE, // Inhabitants have the form ArrayE(T) -- implemented by class ArrayETerm_c
    ArrayS, // Inhabitants have the form ArrayS(N) -- implemented by class ArraySTerm_c
    Array, // Inhabitants have the form Array(...) -- implemented by class ArrayTerm_c

    OrderedMapType, // Inhabitants are OrderedMapDC, OrderedMapD, OrderedMapC, OrderedMap.
    OrderedMapDC, // Inhabitants have the form OrderedMapDC(Domain,Codomain) -- implemented by class OrderedMapDCTerm_c
    OrderedMapD, // Inhabitants have the form OrderedMapD(Domain) -- implemented by class OrderedMapDTerm_c
    OrderedMapC, // Inhabitants have the form OrderedMapC(Codomain) -- implemented by class OrderedMapCTerm_c
    OrderedMap, // Inhabitants have the form OrderedMap(...) -- implemented by class OrderedMapTerm_c

    TupleType, // Inhabitant is Tuple.
    Tuple, // Inhabitants have the form Tuple(...) -- implemented by class TupleTerm_c.

    StructType, // Inhabitant is Struct.
    Struct, // Inhabitants have the form Struct(...) -- implemented by StructTerm.

    MemRefType, // Inhabitant is MemRef
    MemRef, // Inhabitants have the form MemRef(&d), where d is Data
    GlobalSymRefType, // Inhabitant is GlobalSymRef
    GlobalSymRef, // Inhabitants have the form GlobalSymRef("<symbol-id>")
    LocalSymRefType, // Inhabitant is LocalSymRef
    LocalSymRef, // Inhabitants have the form LocalSymRef("<symbol-id>", <shared-ptr-to-symbol-table>)

    PlaceholderType,
    Placeholder,

    FreevarType,
    Freevar,

    //
    // Control terms
    //

    OutputType, // Sole inhabitant is Output
    Output, // Inhabitants have the form Output(V) for some value V
    ClearOutputType, // Sole inhabitant is ClearOutput
    ClearOutput, // Singleton
    EndOfFileType, // Sole inhabitant is EndOfFile
    EndOfFile, // Singleton
    RequestSyncInputType, // Sole inhabitant is RequestSyncInput
    RequestSyncInput, // Inhabitants have the form RequestSyncInput(T) for some type T

    // TODO: Ideally there could be an "Unspecified(u8)" in which the u8 value is disjoint
    // with the above values, and so it would use niche logic and not take up more storage than u8.
    // this could be used for application-specific values, though that would hinder interoperability.
}

// impl dy::IntoValue for DynNPTerm {}

// TODO: should this be Box<dyn Any>?
// impl Into<Box<&dyn TermTrait>> for DynNPTerm {
//     fn into(&self) -> Box<dyn TermTrait> {
//         match self {
// //             DynNPTerm::Term,
// //             DynNPTerm::NonParametricTerm,
// //             DynNPTerm::ParametricTerm,
// //             DynNPTerm::Type,
// //             DynNPTerm::NonType,
// //             DynNPTerm::NonParametricType,
// //             DynNPTerm::ParametricType,
// //             DynNPTerm::Void,
//             DynNPTerm::True => Box::new(TRUE),
//             DynNPTerm::False => Box::new(FALSE),
// //             DynNPTerm::VoidType,
//             DynNPTerm::TrueType => Box::new(TRUE_TYPE),
//             DynNPTerm::FalseType => Box::new(FALSE_TYPE),
// //             DynNPTerm::EmptyType,
// //             DynNPTerm::FormalTypeOf,
//             DynNPTerm::Bool => Box::new(BOOL),
//         }
//     }
// }

// impl Stringify for DynNPTerm {
//     fn stringify(&self) -> String {
//         // Not sure if it should qualify the thing, or just print the variant raw.
//         format!("DynNPTerm::{:?}", self)
//     }
// }

/*
impl TermTrait for DynNPTerm {
//     type AbstractTypeType = Value;
    type AbstractTypeType = Box<dy::ValueGuts>;

    fn label() -> &'static str {
        "DynNPTerm"
    }
    fn is_parametric(&self) -> bool {
        match self {
            DynNPTerm::Term => TERM.is_parametric(),
//             DynNPTerm::NonParametricTerm,
//             DynNPTerm::ParametricTerm,
            DynNPTerm::Type => TYPE.is_parametric(),
//             DynNPTerm::NonType,
//             DynNPTerm::NonParametricType,
//             DynNPTerm::ParametricType,
            DynNPTerm::Void => VOID.is_parametric(),
            DynNPTerm::True => TRUE.is_parametric(),
            DynNPTerm::False => FALSE.is_parametric(),
            DynNPTerm::VoidType => VOID_TYPE.is_parametric(),
            DynNPTerm::TrueType => TRUE_TYPE.is_parametric(),
            DynNPTerm::FalseType => FALSE_TYPE.is_parametric(),
            DynNPTerm::EmptyType => EMPTY_TYPE.is_parametric(),
//             DynNPTerm::FormalTypeOf,
            DynNPTerm::Bool => BOOL.is_parametric(),
            DynNPTerm::Sint8 => SINT8.is_parametric(),
            DynNPTerm::Sint16 => SINT16.is_parametric(),
            DynNPTerm::Sint32 => SINT32.is_parametric(),
            DynNPTerm::Sint64 => SINT64.is_parametric(),
            DynNPTerm::Uint8 => UINT8.is_parametric(),
            DynNPTerm::Uint16 => UINT16.is_parametric(),
            DynNPTerm::Uint32 => UINT32.is_parametric(),
            DynNPTerm::Uint64 => UINT64.is_parametric(),
            DynNPTerm::Float32 => FLOAT32.is_parametric(),
            DynNPTerm::Float64 => FLOAT64.is_parametric(),
            DynNPTerm::BoolType => BOOL_TYPE.is_parametric(),
            DynNPTerm::Sint8Type => SINT8_TYPE.is_parametric(),
            DynNPTerm::Sint16Type => SINT16_TYPE.is_parametric(),
            DynNPTerm::Sint32Type => SINT32_TYPE.is_parametric(),
            DynNPTerm::Sint64Type => SINT64_TYPE.is_parametric(),
            DynNPTerm::Uint8Type => UINT8_TYPE.is_parametric(),
            DynNPTerm::Uint16Type => UINT16_TYPE.is_parametric(),
            DynNPTerm::Uint32Type => UINT32_TYPE.is_parametric(),
            DynNPTerm::Uint64Type => UINT64_TYPE.is_parametric(),
            DynNPTerm::Float32Type => FLOAT32_TYPE.is_parametric(),
            DynNPTerm::Float64Type => FLOAT64_TYPE.is_parametric(),
//             DynNPTerm::SintType,
//             DynNPTerm::Sint,
//             DynNPTerm::UintType,
//             DynNPTerm::Uint,
//             DynNPTerm::FloatType,
//             DynNPTerm::Float,
//             DynNPTerm::PodType,
//             DynNPTerm::Pod,
//             DynNPTerm::UnionType,
//             DynNPTerm::Union,
//             DynNPTerm::Intersection,
//             DynNPTerm::Negation,
//             DynNPTerm::Difference,
            DynNPTerm::ArrayType => ARRAY_TYPE.is_parametric(),
//             DynNPTerm::ArrayES,
//             DynNPTerm::ArrayE,
//             DynNPTerm::ArrayS,
            DynNPTerm::Array => ARRAY.is_parametric(),
//             DynNPTerm::OrderedMapType,
//             DynNPTerm::OrderedMapDC,
//             DynNPTerm::OrderedMapD,
//             DynNPTerm::OrderedMapC,
//             DynNPTerm::OrderedMap,
//             DynNPTerm::TupleType,
//             DynNPTerm::Tuple,
//             DynNPTerm::MemRefType,
//             DynNPTerm::MemRef,
//             DynNPTerm::GlobalSymRefType,
//             DynNPTerm::GlobalSymRef,
//             DynNPTerm::LocalSymRefType,
//             DynNPTerm::LocalSymRef,
//             DynNPTerm::PlaceholderType,
//             DynNPTerm::Placeholder,
//             DynNPTerm::FreevarType,
//             DynNPTerm::Freevar,
//             DynNPTerm::OutputType,
//             DynNPTerm::Output,
//             DynNPTerm::ClearOutputType,
//             DynNPTerm::ClearOutput,
//             DynNPTerm::EndOfFileType,
//             DynNPTerm::EndOfFile,
//             DynNPTerm::RequestSyncInputType,
//             DynNPTerm::RequestSyncInput,
            _ => unimplemented!("sad face"),
        }
    }
    fn is_type(&self) -> bool {
        match self {
            DynNPTerm::Term => TERM.is_type(),
//             DynNPTerm::NonParametricTerm,
//             DynNPTerm::ParametricTerm,
            DynNPTerm::Type => TYPE.is_type(),
//             DynNPTerm::NonType,
//             DynNPTerm::NonParametricType,
//             DynNPTerm::ParametricType,
            DynNPTerm::Void => VOID.is_type(),
            DynNPTerm::True => TRUE.is_type(),
            DynNPTerm::False => FALSE.is_type(),
            DynNPTerm::VoidType => VOID_TYPE.is_type(),
            DynNPTerm::TrueType => TRUE_TYPE.is_type(),
            DynNPTerm::FalseType => FALSE_TYPE.is_type(),
            DynNPTerm::EmptyType => EMPTY_TYPE.is_type(),
//             DynNPTerm::FormalTypeOf,
            DynNPTerm::Bool => BOOL.is_type(),
            DynNPTerm::Sint8 => SINT8.is_type(),
            DynNPTerm::Sint16 => SINT16.is_type(),
            DynNPTerm::Sint32 => SINT32.is_type(),
            DynNPTerm::Sint64 => SINT64.is_type(),
            DynNPTerm::Uint8 => UINT8.is_type(),
            DynNPTerm::Uint16 => UINT16.is_type(),
            DynNPTerm::Uint32 => UINT32.is_type(),
            DynNPTerm::Uint64 => UINT64.is_type(),
            DynNPTerm::Float32 => FLOAT32.is_type(),
            DynNPTerm::Float64 => FLOAT64.is_type(),
            DynNPTerm::BoolType => BOOL_TYPE.is_type(),
            DynNPTerm::Sint8Type => SINT8_TYPE.is_type(),
            DynNPTerm::Sint16Type => SINT16_TYPE.is_type(),
            DynNPTerm::Sint32Type => SINT32_TYPE.is_type(),
            DynNPTerm::Sint64Type => SINT64_TYPE.is_type(),
            DynNPTerm::Uint8Type => UINT8_TYPE.is_type(),
            DynNPTerm::Uint16Type => UINT16_TYPE.is_type(),
            DynNPTerm::Uint32Type => UINT32_TYPE.is_type(),
            DynNPTerm::Uint64Type => UINT64_TYPE.is_type(),
            DynNPTerm::Float32Type => FLOAT32_TYPE.is_type(),
            DynNPTerm::Float64Type => FLOAT64_TYPE.is_type(),
//             DynNPTerm::SintType,
//             DynNPTerm::Sint,
//             DynNPTerm::UintType,
//             DynNPTerm::Uint,
//             DynNPTerm::FloatType,
//             DynNPTerm::Float,
//             DynNPTerm::PodType,
//             DynNPTerm::Pod,
//             DynNPTerm::UnionType,
//             DynNPTerm::Union,
//             DynNPTerm::Intersection,
//             DynNPTerm::Negation,
//             DynNPTerm::Difference,
            DynNPTerm::ArrayType => ARRAY_TYPE.is_type(),
//             DynNPTerm::ArrayES,
//             DynNPTerm::ArrayE,
//             DynNPTerm::ArrayS,
            DynNPTerm::Array => ARRAY.is_type(),
//             DynNPTerm::OrderedMapType,
//             DynNPTerm::OrderedMapDC,
//             DynNPTerm::OrderedMapD,
//             DynNPTerm::OrderedMapC,
//             DynNPTerm::OrderedMap,
//             DynNPTerm::TupleType,
//             DynNPTerm::Tuple,
//             DynNPTerm::MemRefType,
//             DynNPTerm::MemRef,
//             DynNPTerm::GlobalSymRefType,
//             DynNPTerm::GlobalSymRef,
//             DynNPTerm::LocalSymRefType,
//             DynNPTerm::LocalSymRef,
//             DynNPTerm::PlaceholderType,
//             DynNPTerm::Placeholder,
//             DynNPTerm::FreevarType,
//             DynNPTerm::Freevar,
//             DynNPTerm::OutputType,
//             DynNPTerm::Output,
//             DynNPTerm::ClearOutputType,
//             DynNPTerm::ClearOutput,
//             DynNPTerm::EndOfFileType,
//             DynNPTerm::EndOfFile,
//             DynNPTerm::RequestSyncInputType,
//             DynNPTerm::RequestSyncInput,
            _ => unimplemented!("sad face"),
        }
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        match self {
            DynNPTerm::Term => Value::from(TERM.abstract_type()),
//             DynNPTerm::NonParametricTerm,
//             DynNPTerm::ParametricTerm,
            DynNPTerm::Type => Value::from(TYPE.abstract_type()),
//             DynNPTerm::NonType,
//             DynNPTerm::NonParametricType,
//             DynNPTerm::ParametricType,
            DynNPTerm::Void => Value::from(VOID.abstract_type()),
            DynNPTerm::True => Value::from(TRUE.abstract_type()),
            DynNPTerm::False => Value::from(FALSE.abstract_type()),
            DynNPTerm::VoidType => Value::from(VOID_TYPE.abstract_type()),
            DynNPTerm::TrueType => Value::from(TRUE_TYPE.abstract_type()),
            DynNPTerm::FalseType => Value::from(FALSE_TYPE.abstract_type()),
            DynNPTerm::EmptyType => Value::from(EMPTY_TYPE.abstract_type()),
//             DynNPTerm::FormalTypeOf,
            DynNPTerm::Bool => Value::from(BOOL.abstract_type()),
            DynNPTerm::Sint8 => Value::from(SINT8.abstract_type()),
            DynNPTerm::Sint16 => Value::from(SINT16.abstract_type()),
            DynNPTerm::Sint32 => Value::from(SINT32.abstract_type()),
            DynNPTerm::Sint64 => Value::from(SINT64.abstract_type()),
            DynNPTerm::Uint8 => Value::from(UINT8.abstract_type()),
            DynNPTerm::Uint16 => Value::from(UINT16.abstract_type()),
            DynNPTerm::Uint32 => Value::from(UINT32.abstract_type()),
            DynNPTerm::Uint64 => Value::from(UINT64.abstract_type()),
            DynNPTerm::Float32 => Value::from(FLOAT32.abstract_type()),
            DynNPTerm::Float64 => Value::from(FLOAT64.abstract_type()),
            DynNPTerm::BoolType => Value::from(BOOL_TYPE.abstract_type()),
            DynNPTerm::Sint8Type => Value::from(SINT8_TYPE.abstract_type()),
            DynNPTerm::Sint16Type => Value::from(SINT16_TYPE.abstract_type()),
            DynNPTerm::Sint32Type => Value::from(SINT32_TYPE.abstract_type()),
            DynNPTerm::Sint64Type => Value::from(SINT64_TYPE.abstract_type()),
            DynNPTerm::Uint8Type => Value::from(UINT8_TYPE.abstract_type()),
            DynNPTerm::Uint16Type => Value::from(UINT16_TYPE.abstract_type()),
            DynNPTerm::Uint32Type => Value::from(UINT32_TYPE.abstract_type()),
            DynNPTerm::Uint64Type => Value::from(UINT64_TYPE.abstract_type()),
            DynNPTerm::Float32Type => Value::from(FLOAT32_TYPE.abstract_type()),
            DynNPTerm::Float64Type => Value::from(FLOAT64_TYPE.abstract_type()),
//             DynNPTerm::SintType,
//             DynNPTerm::Sint,
//             DynNPTerm::UintType,
//             DynNPTerm::Uint,
//             DynNPTerm::FloatType,
//             DynNPTerm::Float,
//             DynNPTerm::PodType,
//             DynNPTerm::Pod,
//             DynNPTerm::UnionType,
//             DynNPTerm::Union,
//             DynNPTerm::Intersection,
//             DynNPTerm::Negation,
//             DynNPTerm::Difference,
            DynNPTerm::ArrayType => Value::from(ARRAY_TYPE.abstract_type()),
//             DynNPTerm::ArrayES,
//             DynNPTerm::ArrayE,
//             DynNPTerm::ArrayS,
            DynNPTerm::Array => Value::from(ARRAY.abstract_type()),
//             DynNPTerm::OrderedMapType,
//             DynNPTerm::OrderedMapDC,
//             DynNPTerm::OrderedMapD,
//             DynNPTerm::OrderedMapC,
//             DynNPTerm::OrderedMap,
//             DynNPTerm::TupleType,
//             DynNPTerm::Tuple,
//             DynNPTerm::MemRefType,
//             DynNPTerm::MemRef,
//             DynNPTerm::GlobalSymRefType,
//             DynNPTerm::GlobalSymRef,
//             DynNPTerm::LocalSymRefType,
//             DynNPTerm::LocalSymRef,
//             DynNPTerm::PlaceholderType,
//             DynNPTerm::Placeholder,
//             DynNPTerm::FreevarType,
//             DynNPTerm::Freevar,
//             DynNPTerm::OutputType,
//             DynNPTerm::Output,
//             DynNPTerm::ClearOutputType,
//             DynNPTerm::ClearOutput,
//             DynNPTerm::EndOfFileType,
//             DynNPTerm::EndOfFile,
//             DynNPTerm::RequestSyncInputType,
//             DynNPTerm::RequestSyncInput,
            _ => unimplemented!("sad face"),
        }
//         match self {
//             DynNPTerm::Term => Box::new(TERM.abstract_type()),
// //             DynNPTerm::NonParametricTerm,
// //             DynNPTerm::ParametricTerm,
//             DynNPTerm::Type => Box::new(TYPE.abstract_type()),
// //             DynNPTerm::NonType,
// //             DynNPTerm::NonParametricType,
// //             DynNPTerm::ParametricType,
//             DynNPTerm::Void => Box::new(VOID.abstract_type()),
//             DynNPTerm::True => Box::new(TRUE.abstract_type()),
//             DynNPTerm::False => Box::new(FALSE.abstract_type()),
//             DynNPTerm::VoidType => Box::new(VOID_TYPE.abstract_type()),
//             DynNPTerm::TrueType => Box::new(TRUE_TYPE.abstract_type()),
//             DynNPTerm::FalseType => Box::new(FALSE_TYPE.abstract_type()),
//             DynNPTerm::EmptyType => Box::new(EMPTY_TYPE.abstract_type()),
// //             DynNPTerm::FormalTypeOf,
//             DynNPTerm::Bool => Box::new(BOOL.abstract_type()),
//             DynNPTerm::Sint8 => Box::new(SINT8.abstract_type()),
//             DynNPTerm::Sint16 => Box::new(SINT16.abstract_type()),
//             DynNPTerm::Sint32 => Box::new(SINT32.abstract_type()),
//             DynNPTerm::Sint64 => Box::new(SINT64.abstract_type()),
//             DynNPTerm::Uint8 => Box::new(UINT8.abstract_type()),
//             DynNPTerm::Uint16 => Box::new(UINT16.abstract_type()),
//             DynNPTerm::Uint32 => Box::new(UINT32.abstract_type()),
//             DynNPTerm::Uint64 => Box::new(UINT64.abstract_type()),
//             DynNPTerm::Float32 => Box::new(FLOAT32.abstract_type()),
//             DynNPTerm::Float64 => Box::new(FLOAT64.abstract_type()),
//             DynNPTerm::BoolType => Box::new(BOOL_TYPE.abstract_type()),
//             DynNPTerm::Sint8Type => Box::new(SINT8_TYPE.abstract_type()),
//             DynNPTerm::Sint16Type => Box::new(SINT16_TYPE.abstract_type()),
//             DynNPTerm::Sint32Type => Box::new(SINT32_TYPE.abstract_type()),
//             DynNPTerm::Sint64Type => Box::new(SINT64_TYPE.abstract_type()),
//             DynNPTerm::Uint8Type => Box::new(UINT8_TYPE.abstract_type()),
//             DynNPTerm::Uint16Type => Box::new(UINT16_TYPE.abstract_type()),
//             DynNPTerm::Uint32Type => Box::new(UINT32_TYPE.abstract_type()),
//             DynNPTerm::Uint64Type => Box::new(UINT64_TYPE.abstract_type()),
//             DynNPTerm::Float32Type => Box::new(FLOAT32_TYPE.abstract_type()),
//             DynNPTerm::Float64Type => Box::new(FLOAT64_TYPE.abstract_type()),
// //             DynNPTerm::SintType,
// //             DynNPTerm::Sint,
// //             DynNPTerm::UintType,
// //             DynNPTerm::Uint,
// //             DynNPTerm::FloatType,
// //             DynNPTerm::Float,
// //             DynNPTerm::PodType,
// //             DynNPTerm::Pod,
// //             DynNPTerm::UnionType,
// //             DynNPTerm::Union,
// //             DynNPTerm::Intersection,
// //             DynNPTerm::Negation,
// //             DynNPTerm::Difference,
//             DynNPTerm::ArrayType => Box::new(ARRAY_TYPE.abstract_type()),
// //             DynNPTerm::ArrayES,
// //             DynNPTerm::ArrayE,
// //             DynNPTerm::ArrayS,
//             DynNPTerm::Array => Box::new(ARRAY.abstract_type()),
// //             DynNPTerm::OrderedMapType,
// //             DynNPTerm::OrderedMapDC,
// //             DynNPTerm::OrderedMapD,
// //             DynNPTerm::OrderedMapC,
// //             DynNPTerm::OrderedMap,
// //             DynNPTerm::TupleType,
// //             DynNPTerm::Tuple,
// //             DynNPTerm::MemRefType,
// //             DynNPTerm::MemRef,
// //             DynNPTerm::GlobalSymRefType,
// //             DynNPTerm::GlobalSymRef,
// //             DynNPTerm::LocalSymRefType,
// //             DynNPTerm::LocalSymRef,
// //             DynNPTerm::PlaceholderType,
// //             DynNPTerm::Placeholder,
// //             DynNPTerm::FreevarType,
// //             DynNPTerm::Freevar,
// //             DynNPTerm::OutputType,
// //             DynNPTerm::Output,
// //             DynNPTerm::ClearOutputType,
// //             DynNPTerm::ClearOutput,
// //             DynNPTerm::EndOfFileType,
// //             DynNPTerm::EndOfFile,
// //             DynNPTerm::RequestSyncInputType,
// //             DynNPTerm::RequestSyncInput,
//             _ => unimplemented!("sad face"),
//         }
    }
}

impl NonParametricTermTrait for DynNPTerm {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        *self
    }
}
*/
