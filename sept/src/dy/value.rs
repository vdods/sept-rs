use crate::{
    dy::{self, IntoValue, RUNTIME_LA},
    parser,
    st::{self, Stringifiable, TermTrait},
    Error, Result,
};
use std::any::Any;

pub type ValueGuts = dyn Any + Send + Sync;

/// This is the central runtime data type for sept.  Its methods will call into the corresponding
/// methods of the runtime.
// This really should be named Term (or more pedantically TermTerm), but that's already taken.
// Maybe the naming scheme can be shifted around so that this can be named Term (or TermTerm).
#[derive(derive_more::Into)]
pub struct Value(Box<ValueGuts>);

impl AsMut<ValueGuts> for Value {
    fn as_mut(&mut self) -> &mut ValueGuts {
        self.0.as_mut()
    }
}

impl AsRef<ValueGuts> for Value {
    fn as_ref(&self) -> &ValueGuts {
        self.0.as_ref()
    }
}

impl Clone for Value {
    fn clone(&self) -> Self {
        Value(RUNTIME_LA.read().unwrap().clone(self.as_ref()))
    }
}

impl dy::Constructor for Value {
    type ConstructedType = Value;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        Ok(RUNTIME_LA
            .read()
            .unwrap()
            .construct(self.as_ref(), parameter_t)?)
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        Ok(RUNTIME_LA
            .read()
            .unwrap()
            .deserialize_parameters_and_construct(self.as_ref(), reader)?)
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Value(")?;
        RUNTIME_LA.read().unwrap().debug(self.as_ref(), f)?;
        write!(f, ")")?;
        Ok(())
    }
}

impl dy::Deconstruct for Value {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: Implement self-consuming deconstruct in Runtime.
        RUNTIME_LA.read().unwrap().deconstructed(self.as_ref())
    }
    fn deconstructed(&self) -> dy::Deconstruction {
        RUNTIME_LA.read().unwrap().deconstructed(self.as_ref())
    }
}

impl std::ops::Deref for Value {
    type Target = ValueGuts;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::ops::DerefMut for Value {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

// impl std::fmt::Debug for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//         write!(f, "Value({})", &self.stringify())
//     }
// }

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

/// This prevents directly nested Value-s, e.g. Value(Value(123u32)), since that's never what we want.
impl From<Box<ValueGuts>> for Value {
    fn from(b: Box<ValueGuts>) -> Self {
        if b.is::<Value>() {
            log::warn!("Some code attempted to construct a directly nested Value (i.e. `Value(Value(...))`).  Preventing that by unwrapping all but the inner Value, since directly nested Value-s are never what we want.");
            Self::from(b.downcast::<Value>().unwrap().into_inner())
        } else {
            Self(b)
        }
    }
}

impl<T: TermTrait + IntoValue + 'static> From<T> for Value {
    fn from(t: T) -> Self {
        Self(Box::new(t))
    }
}

impl std::str::FromStr for Value {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(parser::parse_value(s)?)
    }
}

impl st::Inhabits<Value> for Value {
    fn inhabits(&self, rhs: &Value) -> bool {
        RUNTIME_LA
            .read()
            .unwrap()
            .inhabits(self.as_ref(), rhs.as_ref())
    }
}

impl<T: st::TypeTrait + IntoValue + 'static> st::Inhabits<T> for Value {
    fn inhabits(&self, rhs: &T) -> bool {
        let rhs_: &ValueGuts = rhs;
        RUNTIME_LA.read().unwrap().inhabits(self.as_ref(), rhs_)
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        RUNTIME_LA.read().unwrap().eq(self.as_ref(), other.as_ref())
    }
}

impl st::Deserializable for Value {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        // First read the SerializedTopLevelCode to decide what to do.
        match st::SerializedTopLevelCode::read(reader)? {
            st::SerializedTopLevelCode::Construction => {
                // Deserialize the constructor.
                let constructor = Value::deserialize(reader)?;
                // Deserialize the parameters and construct the Value.
                use dy::Constructor;
                Ok(constructor.deserialize_parameters_and_construct(reader)?)
            }
            st::SerializedTopLevelCode::NonParametric => {
                // The NonParametricTermCode plays the role of the constructor, and there are
                // no parameters as you might have guessed.
                let non_parametric_term_code = st::NonParametricTermCode::read(reader)?;
                Ok(dy::RUNTIME_LA
                    .read()
                    .unwrap()
                    .non_parametric_term_from_code(non_parametric_term_code)?)
            }
        }
    }
}

impl st::Serializable for Value {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        log::trace!("Value::serialize; self: {:?}", self);
        let mut bytes_written = 0usize;
        if self.nondereferencing_is_parametric() {
            let constructor = self.nondereferencing_abstract_type();
            log::trace!("    constructor: {:?}", constructor);
            // This indicates this serialization is a construction.
            bytes_written += st::SerializedTopLevelCode::Construction.write(writer)?;
            // This is the constructor
            bytes_written += constructor.serialize(writer)?;
            // This is the value itself.  Note that Runtime::serialize is a non-dereferencing serialize.
            bytes_written += dy::RUNTIME_LA
                .read()
                .unwrap()
                .serialize(self.as_ref(), writer)?;
        } else {
            // This indicates this serialization is a non-parametric term.
            bytes_written += st::SerializedTopLevelCode::NonParametric.write(writer)?;
            // This is the code representing the non-parametric term itself (its "value").
            bytes_written += dy::RUNTIME_LA
                .read()
                .unwrap()
                .non_parametric_term_code(self.as_ref())?
                .write(writer)?;
        }
        Ok(bytes_written)

        //         // Because Value stores type information dynamically, it all has to be serialized.
        //         let mut bytes_written =
        //             dy::RUNTIME_LA.read().unwrap().serialize_top_level_code(self.as_ref(), writer)?;
        //         // TODO: This would be replaced with getting the constructor (assuming the value is
        //         // Constructible) and then serializing that.
        //         bytes_written +=
        //             dy::RUNTIME_LA.read().unwrap().serialize_constructor(self.as_ref(), writer)?;
        //         bytes_written +=
        //             dy::RUNTIME_LA.read().unwrap().serialize_parameters(self.as_ref(), writer)?;
        //         Ok(bytes_written)
    }

    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(dy::RUNTIME_LA.read().unwrap().serialize_top_level_code(self.as_ref(), writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(dy::RUNTIME_LA.read().unwrap().serialize_constructor(self.as_ref(), writer)?)
    //     }
    //     fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(dy::RUNTIME_LA.read().unwrap().serialize_parameters(self.as_ref(), writer)?)
    //     }
}

impl Stringifiable for Value {
    fn stringify(&self) -> String {
        RUNTIME_LA.read().unwrap().stringify(self.as_ref())
    }
}

impl TermTrait for Value {
    type AbstractTypeType = Value;

    fn is_parametric(&self) -> bool {
        RUNTIME_LA.read().unwrap().is_parametric(self.as_ref())
    }
    fn is_type(&self) -> bool {
        RUNTIME_LA.read().unwrap().is_type(self.as_ref())
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Value(RUNTIME_LA.read().unwrap().abstract_type_of(self.as_ref()))
    }
}

impl st::TypeTrait for Value {}

// TODO: These could become part of dy::TermTrait, since they reflect what's available via Runtime
impl Value {
    pub fn into_inner(self) -> Box<ValueGuts> {
        self.0
    }
    /// This will return the downcasted value, consuming self, or panic if the cast fails.
    pub fn downcast_into<T: st::TermTrait>(self) -> T {
        *self.0.downcast::<T>().unwrap()
    }
    pub fn dereferenced<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(RUNTIME_LA.read().unwrap().dereferenced(self.as_ref())?)
    }
    /// If this Value contains dy::Deconstruction, then it calls reconstruct on it, otherwise
    /// returns an error.  The call to reconstruct may return an error.
    // TODO: Figure out if Deconstruction really should be allowed to be made into a Value.
    pub fn reconstruct_in_place(&mut self) -> Result<()> {
        if self.is::<dy::Deconstruction>() {
            // This is a bit silly, but I don't want to bother with unsafe code at this point.
            // The point is to swap out self with a dummy, operate on the dummy (self's former
            // value), and then swap back into place.
            let mut dummy = Value::from(st::Void);
            std::mem::swap(self, &mut dummy);
            // Ideally we would use Box::into_inner here instead of *, but that's somehow still unstable.
            let deconstruction: dy::Deconstruction =
                *dummy.0.downcast::<dy::Deconstruction>().unwrap();
            self.0 = Box::new(deconstruction.reconstruct()?);
        } else {
            // No need to reconstruct anything.
        }
        Ok(())
    }
    fn nondereferencing_abstract_type(&self) -> Self {
        RUNTIME_LA
            .read()
            .unwrap()
            .nondereferencing_abstract_type_of(self.as_ref())
            .into()
    }
    fn nondereferencing_is_parametric(&self) -> bool {
        RUNTIME_LA
            .read()
            .unwrap()
            .nondereferencing_is_parametric(self.as_ref())
    }
}
