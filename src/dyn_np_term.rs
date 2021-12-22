use crate::{BOOL, FALSE, FALSE_TYPE, NonParametricTermTrait, Stringify, TERM, TermTrait, TRUE, TRUE_TYPE, TYPE, VOID, VOID_TYPE};

// The repr(u8) attribute is to be compatible with the C++ implementation.
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

impl Stringify for DynNPTerm {
    fn stringify(&self) -> String {
        // Not sure if it should qualify the thing, or just print the variant raw.
        format!("DynNPTerm::{:?}", self)
    }
}

impl TermTrait for DynNPTerm {
    fn is_parametric_term(&self) -> bool {
        match self {
            DynNPTerm::Term => TERM.is_parametric_term(),
//             DynNPTerm::NonParametricTerm,
//             DynNPTerm::ParametricTerm,
            DynNPTerm::Type => TYPE.is_parametric_term(),
//             DynNPTerm::NonType,
//             DynNPTerm::NonParametricType,
//             DynNPTerm::ParametricType,
            DynNPTerm::Void => VOID.is_parametric_term(),
            DynNPTerm::True => TRUE.is_parametric_term(),
            DynNPTerm::False => FALSE.is_parametric_term(),
            DynNPTerm::VoidType => VOID_TYPE.is_parametric_term(),
            DynNPTerm::TrueType => TRUE_TYPE.is_parametric_term(),
            DynNPTerm::FalseType => FALSE_TYPE.is_parametric_term(),
//             DynNPTerm::EmptyType,
//             DynNPTerm::FormalTypeOf,
            DynNPTerm::Bool => BOOL.is_parametric_term(),
//             DynNPTerm::Sint8,
//             DynNPTerm::Sint16,
//             DynNPTerm::Sint32,
//             DynNPTerm::Sint64,
//             DynNPTerm::Uint8,
//             DynNPTerm::Uint16,
//             DynNPTerm::Uint32,
//             DynNPTerm::Uint64,
//             DynNPTerm::Float32,
//             DynNPTerm::Float64,
//             DynNPTerm::BoolType,
//             DynNPTerm::Sint8Type,
//             DynNPTerm::Sint16Type,
//             DynNPTerm::Sint32Type,
//             DynNPTerm::Sint64Type,
//             DynNPTerm::Uint8Type,
//             DynNPTerm::Uint16Type,
//             DynNPTerm::Uint32Type,
//             DynNPTerm::Uint64Type,
//             DynNPTerm::Float32Type,
//             DynNPTerm::Float64Type,
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
//             DynNPTerm::ArrayType,
//             DynNPTerm::ArrayES,
//             DynNPTerm::ArrayE,
//             DynNPTerm::ArrayS,
//             DynNPTerm::Array,
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
    fn is_type_term(&self) -> bool {
        match self {
            DynNPTerm::Term => TERM.is_type_term(),
//             DynNPTerm::NonParametricTerm,
//             DynNPTerm::ParametricTerm,
            DynNPTerm::Type => TYPE.is_type_term(),
//             DynNPTerm::NonType,
//             DynNPTerm::NonParametricType,
//             DynNPTerm::ParametricType,
            DynNPTerm::Void => VOID.is_type_term(),
            DynNPTerm::True => TRUE.is_type_term(),
            DynNPTerm::False => FALSE.is_type_term(),
            DynNPTerm::VoidType => VOID_TYPE.is_type_term(),
            DynNPTerm::TrueType => TRUE_TYPE.is_type_term(),
            DynNPTerm::FalseType => FALSE_TYPE.is_type_term(),
//             DynNPTerm::EmptyType,
//             DynNPTerm::FormalTypeOf,
            DynNPTerm::Bool => BOOL.is_type_term(),
            _ => unimplemented!("sad face"),
        }
    }
}

impl NonParametricTermTrait for DynNPTerm {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        *self
    }
}
