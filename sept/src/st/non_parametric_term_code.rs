use crate::Result;

// The repr(u8) attribute is to be compatible with the C++ implementation.
// NOTE: TermTrait and all the other things like Stringifiable are not being implemented
// here, as this enum is simply meant for serialization representation purposes.  In
// deserialization, Values containing the "real" terms would be used instead, so that
// there isn't a need to check for multiple alternate representations of various types.
// TODO: Consider versioning this type, e.g. NonParametricTermCode_1_0, and then including a version marker
// in the textifaction or binary serialization.
#[repr(u8)]
#[derive(Clone, Copy, Debug, derive_more::Display, Eq, Hash, PartialEq, int_enum::IntEnum)]
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
    AsciiChar = 0x1A, // TODO: Add UnicodeChar later and whatever else -- TODO: Maybe Ascii should be an abstract type

    // POD Type Types

    /// Sole inhabitant is Bool.
    BoolType = 0x1B,
    /// Sole inhabitant is Sint8.
    Sint8Type = 0x1C,
    /// Sole inhabitant is Sint16.
    Sint16Type = 0x1D,
    /// Sole inhabitant is Sint32.
    Sint32Type = 0x1E,
    /// Sole inhabitant is Sint64.
    Sint64Type = 0x1F,
    /// Sole inhabitant is Uint8.
    Uint8Type = 0x20,
    /// Sole inhabitant is Uint16.
    Uint16Type = 0x21,
    /// Sole inhabitant is Uint32
    Uint32Type = 0x22,
    /// Sole inhabitant is Uint64
    Uint64Type = 0x23,
    /// Sole inhabitant is Float32
    Float32Type = 0x24,
    /// Sole inhabitant is Float64
    Float64Type = 0x25,
    /// Sole inhabitant is AsciiChar.
    AsciiCharType = 0x26,

    Utf8String = 0x27,
    /// Sole inhabitant is Utf8String.
    Utf8StringType = 0x28,

    // Other Types related to POD Types

    /// Isomorphic to Union(Sint8, Sint16, Sint32, Sint64).
    // TODO: Maybe allow this to construct Sint(N) where N is the number of bits, and e.g. Sint(32)
    // would be isomorphic to Sint32.
    Sint = 0x29,
    /// Isomorphic to Union(Uint8, Uint16, Uint32, Uint64).
    // TODO: Maybe allow this to construct Uint(N) where N is the number of bits, and e.g. Uint(32)
    // would be isomorphic to Uint32.
    Uint = 0x2A,
    /// Isomorphic to Union(Float32,Float64).
    Float = 0x2B,
    /// Isomorphic to Union(Bool, Sint, Uint, Float).  Inhabitants are POD values.  Pod : PodType.
    Pod = 0x2C,

    /// Isomorphic to Union(Sint8Type, Sint16Type, Sint32Type, Sint64Type).
    SintType = 0x2D,
    /// Isomorphic to Union(Uint8Type, Uint16Type, Uint32Type, Uint64Type).
    UintType = 0x2E,
    /// Isomorphic to Union(Float32Type,Float64Type).
    FloatType = 0x2F,
    // TODO: Add CHAR types
    /// Isomorphic to Union(BoolType, SintType, UintType, FloatType) (TODO: Somehow add Pod as an inhabitant)
    PodType = 0x30,
    // TODO: Add semantic classes like Positive, Negative, NonPositive, NonNegative, Zero

    /// Inhabitants have the form Union(T1,...,TN) -- implemented as UnionTerm.
    Union = 0x31,
    /// Inhabitants have the form Intersection(T1,...,TN) -- implemented as IntersectionTerm.
    Intersection = 0x32,
    /// Inhabitants have the form Negation(T) -- implemented as NegationTerm.
    Negation = 0x33,
    /// Inhabitants have the form Difference(T,U1,...,UN) -- implemented as DifferenceTerm.
    Difference = 0x34,

    /// Sole inhabitant is Union.
    UnionType = 0x35,
    /// Sole inhabitant is Intersection.
    IntersectionType = 0x36,
    /// Sole inhabitant is Negation.
    NegationType = 0x37,
    /// Sole inhabitant is Difference.
    DifferenceType = 0x38,

    // TODO: UnionType, IntersectionType, etc.

    /// Inhabitants are ArrayES, ArrayE, ArrayS, Array.
    ArrayType = 0x39,
    /// Inhabitants have the form ArrayES(T,N) -- implemented as ArrayESTerm.
    ArrayES = 0x3A,
    /// Inhabitants have the form ArrayE(T) -- implemented as ArrayETerm.
    ArrayE = 0x3B,
    /// Inhabitants have the form ArrayS(N) -- implemented as ArraySTerm.
    ArrayS = 0x3C,
    /// Inhabitants have the form Array(...) -- implemented as ArrayTerm.
    Array = 0x3D,

    /// Inhabitants are OrderedMapDC, OrderedMapD, OrderedMapC, OrderedMap.
    OrderedMapType = 0x3E,
    /// Inhabitants have the form OrderedMapDC(Domain,Codomain) -- implemented as OrderedMapDCTerm.
    OrderedMapDC = 0x3F,
    /// Inhabitants have the form OrderedMapD(Domain) -- implemented as OrderedMapDTerm.
    OrderedMapD = 0x40,
    /// Inhabitants have the form OrderedMapC(Codomain) -- implemented as OrderedMapCTerm.
    OrderedMapC = 0x41,
    /// Inhabitants have the form OrderedMap(...) -- implemented as OrderedMapTerm.
    OrderedMap = 0x42,

    // Sole inhabitant is Tuple.
    TupleType = 0x43,
    /// Inhabitants have the form Tuple(...) -- implemented as TupleTerm.
    Tuple = 0x44,

    /// Sole inhabitant is Struct.
    StructType = 0x45,
    /// Inhabitants have the form Struct(...) -- implemented by StructTerm.  Inhabitants are specific
    /// structs.  Struct itself is a metatype which constructs structs.  An instance of a particular
    /// struct would be Struct(...)(...) and is implemented by StructTermTerm.
    Struct = 0x46,

    //
    // Reference-related terms
    //

    /// Sole inhabitant is MemRef.
    MemRefType = 0x47,
    /// Inhabitants have the form MemRef(&d), where d is Data.
    MemRef = 0x48,
    /// Sole inhabitant is GlobalSymRef.
    GlobalSymRefType = 0x49,
    /// Inhabitants have the form GlobalSymRef("<symbol-id>") -- implemented as GlobalSymRefTerm.
    GlobalSymRef = 0x4A,
    /// Sole inhabitant is LocalSymRef.
    LocalSymRefType = 0x4B,
    /// Inhabitants have the form LocalSymRef("<symbol-id>", <shared-ptr-to-symbol-table>) --
    /// implemented as LocalSymRefTerm.
    LocalSymRef = 0x4C,

    PlaceholderType = 0x4D,
    Placeholder = 0x4E,

    FreevarType = 0x4F,
    Freevar = 0x50,

    //
    // Control terms
    //

    /// Sole inhabitant is Output.
    OutputType = 0x51,
    /// Inhabitants have the form Output(V) for some value V.
    Output = 0x52,
    /// Sole inhabitant is ClearOutput.
    ClearOutputType = 0x53,
    /// Singleton.
    ClearOutput = 0x54,
    /// Sole inhabitant is EndOfFile.
    EndOfFileType = 0x55,
    /// Singleton.
    EndOfFile = 0x56,
    /// Sole inhabitant is RequestSyncInput
    RequestSyncInputType = 0x57,
    /// Inhabitants have the form RequestSyncInput(T) for some type T.
    RequestSyncInput = 0x58,

    // TODO: Ideally there could be an "Unspecified(u8)" in which the u8 value is disjoint
    // with the above values, and so it would use niche logic and not take up more storage than u8.
    // this could be used for application-specific values, though that would hinder interoperability.

    Undefined = 0xFF, // TEMP HACK
}

impl NonParametricTermCode {
    pub fn read(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<u8>()];
        reader.read_exact(&mut buffer)?;
        let n = u8::from_le_bytes(buffer);
        // This try_from uses int_enum::IntEnum trait.
        Ok(Self::try_from(n)?)
    }
    pub fn write(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all((*self as u8).to_le_bytes().as_slice())?;
        Ok(std::mem::size_of::<u8>())
    }
}
