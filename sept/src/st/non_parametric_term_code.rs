// The repr(u8) attribute is to be compatible with the C++ implementation.
// NOTE: TermTrait and all the other things like Stringifiable are not being implemented
// here, as this enum is simply meant for serialization representation purposes.  In
// deserialization, Values containing the "real" terms would be used instead, so that
// there isn't a need to check for multiple alternate representations of various types.
// TODO: Consider versioning this type, e.g. NonParametricTermCode_1_0, and then including a version marker
// in the textifaction or binary serialization.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NonParametricTermCode {
    // The most basic Types.

    /// Literally everything is a Term (this could be called Any).
    Term = 0x00,
    /// Inhabitants are Terms requiring no parameters to instantiate (each variant of the
    /// NonParametricTermCode enum corresponds to such a Term).
    NonParametricTerm = 0x01,
    /// Inhabitants are Terms requiring parameters to instantiate, e.g. 10.25, 'x', 999000.
    ParametricTerm = 0x02,

    // Type Types.

    /// A Term which has an inhabitation predicate.
    Type = 0x03,
    /// A Term which is not a Type (this could be called Value).
    NonType = 0x04,
    /// Inhabitants are types requiring no parameters to instantiate (all members of the NPType enum).
    NonParametricType = 0x05,
    /// Inhabitants are types requiring parameters to instantiate, e.g. ArrayESTerm(T,N).
    ParametricType = 0x06,

    // NonParametricTerm && NonType

    /// Void is a NonType that conveys no information.
    Void = 0x07,
    /// The truthier of the two inhabitants of Bool.
    True = 0x08,
    /// The lying inhabitant of Bool.
    False = 0x09,

    // A few natural Types.

    /// Sole inhabitant is Void.
    VoidType = 0x0A,
    /// Sole inhabitant is True.
    TrueType = 0x0B,
    /// Sole inhabitant is False.
    FalseType = 0x0C,
    /// A Type defined to have no inhabitants.
    EmptyType = 0x0D,
    /// Constructs FormalTypeOf(x) for any term x.  Sole inhabitant of FormalTypeOf(x) is x.
    FormalTypeOf = 0x0E,

    // POD Types

    /// Isomorphic to Union(TrueType, FalseType).
    Bool = 0x0F,
    Sint8 = 0x10,
    Sint16 = 0x11,
    Sint32 = 0x12,
    Sint64 = 0x13,
    Uint8 = 0x14,
    Uint16 = 0x15,
    Uint32 = 0x16,
    Uint64 = 0x17,
    Float32 = 0x18,
    Float64 = 0x19,
//     AsciiChar, // TODO: Add UnicodeChar later and whatever else -- TODO: Maybe Ascii should be an abstract type

    // POD Type Types

    /// Sole inhabitant is Bool.
    BoolType,
    /// Sole inhabitant is Sint8.
    Sint8Type,
    /// Sole inhabitant is Sint16.
    Sint16Type,
    /// Sole inhabitant is Sint32.
    Sint32Type,
    /// Sole inhabitant is Sint64.
    Sint64Type,
    /// Sole inhabitant is Uint8.
    Uint8Type,
    /// Sole inhabitant is Uint16.
    Uint16Type,
    /// Sole inhabitant is Uint32
    Uint32Type,
    /// Sole inhabitant is Uint64
    Uint64Type,
    /// Sole inhabitant is Float32
    Float32Type,
    /// Sole inhabitant is Float64
    Float64Type,
//     /// Sole inhabitant is AsciiChar.
//     AsciiCharType,

    Utf8String,
    /// Sole inhabitant is Utf8String.
    Utf8StringType,

    // Other Types related to POD Types

    /// Isomorphic to Union(Sint8Type, Sint16Type, Sint32Type, Sint64Type).
    SintType,
    /// Isomorphic to Union(Sint8, Sint16, Sint32, Sint64).
    // TODO: Maybe allow this to construct Sint(N) where N is the number of bits, and e.g. Sint(32)
    // would be isomorphic to Sint32.
    Sint,
    /// Isomorphic to Union(Uint8Type, Uint16Type, Uint32Type, Uint64Type).
    UintType,
    /// Isomorphic to Union(Uint8, Uint16, Uint32, Uint64).
    // TODO: Maybe allow this to construct Uint(N) where N is the number of bits, and e.g. Uint(32)
    // would be isomorphic to Uint32.
    Uint,
    /// Isomorphic to Union(Float32Type,Float64Type).
    FloatType,
    Float, // Isomorphic to Union(Float32,Float64).
    // TODO: Add CHAR types
    /// Isomorphic to Union(BoolType, SintType, UintType, FloatType) (TODO: Somehow add Pod as an inhabitant)
    PodType,
    /// Isomorphic to Union(Bool, Sint, Uint, Float).  Inhabitants are POD values.  Pod : PodType.
    Pod,
    // TODO: Add semantic classes like Positive, Negative, NonPositive, NonNegative, Zero

    /// Sole inhabitant is Union.
    UnionType,
    /// Inhabitants have the form Union(T1,...,TN) -- implemented as UnionTerm.
    Union,
    /// Inhabitants have the form Intersection(T1,...,TN) -- implemented as IntersectionTerm.
    Intersection,
    /// Inhabitants have the form Negation(T) -- implemented as NegationTerm.
    Negation,
    /// Inhabitants have the form Difference(T,U1,...,UN) -- implemented as DifferenceTerm.
    Difference,

    // TODO: UnionType, IntersectionType, etc.

    /// Inhabitants are ArrayES, ArrayE, ArrayS, Array.
    ArrayType,
    /// Inhabitants have the form ArrayES(T,N) -- implemented as ArrayESTerm.
    ArrayES,
    /// Inhabitants have the form ArrayE(T) -- implemented as ArrayETerm.
    ArrayE,
    /// Inhabitants have the form ArrayS(N) -- implemented as ArraySTerm.
    ArrayS,
    /// Inhabitants have the form Array(...) -- implemented as ArrayTerm.
    Array,

    /// Inhabitants are OrderedMapDC, OrderedMapD, OrderedMapC, OrderedMap.
    OrderedMapType,
    /// Inhabitants have the form OrderedMapDC(Domain,Codomain) -- implemented as OrderedMapDCTerm.
    OrderedMapDC,
    /// Inhabitants have the form OrderedMapD(Domain) -- implemented as OrderedMapDTerm.
    OrderedMapD,
    /// Inhabitants have the form OrderedMapC(Codomain) -- implemented as OrderedMapCTerm.
    OrderedMapC,
    /// Inhabitants have the form OrderedMap(...) -- implemented as OrderedMapTerm.
    OrderedMap,

    // Sole inhabitant is Tuple.
    TupleType,
    /// Inhabitants have the form Tuple(...) -- implemented as TupleTerm.
    Tuple,

    /// Sole inhabitant is Struct.
    StructType,
    /// Inhabitants have the form Struct(...) -- implemented by StructTerm.  Inhabitants are specific
    /// structs.  Struct itself is a metatype which constructs structs.  An instance of a particular
    /// struct would be Struct(...)(...) and is implemented by StructTermTerm.
    Struct,

    //
    // Reference-related terms
    //

    /// Sole inhabitant is MemRef.
    MemRefType,
    /// Inhabitants have the form MemRef(&d), where d is Data.
    MemRef,
    /// Sole inhabitant is GlobalSymRef.
    GlobalSymRefType,
    /// Inhabitants have the form GlobalSymRef("<symbol-id>") -- implemented as GlobalSymRefTerm.
    GlobalSymRef,
    /// Sole inhabitant is LocalSymRef.
    LocalSymRefType,
    /// Inhabitants have the form LocalSymRef("<symbol-id>", <shared-ptr-to-symbol-table>) --
    /// implemented as LocalSymRefTerm.
    LocalSymRef,

    PlaceholderType,
    Placeholder,

    FreevarType,
    Freevar,

    //
    // Control terms
    //

    /// Sole inhabitant is Output.
    OutputType,
    /// Inhabitants have the form Output(V) for some value V.
    Output,
    /// Sole inhabitant is ClearOutput.
    ClearOutputType,
    /// Singleton.
    ClearOutput,
    /// Sole inhabitant is EndOfFile.
    EndOfFileType,
    /// Singleton.
    EndOfFile,
    /// Sole inhabitant is RequestSyncInput
    RequestSyncInputType,
    /// Inhabitants have the form RequestSyncInput(T) for some type T.
    RequestSyncInput,

    // TODO: Ideally there could be an "Unspecified(u8)" in which the u8 value is disjoint
    // with the above values, and so it would use niche logic and not take up more storage than u8.
    // this could be used for application-specific values, though that would hinder interoperability.

    Undefined, // TEMP HACK
}
