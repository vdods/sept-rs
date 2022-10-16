use crate::{dy::{self, IntoValue, RUNTIME_LA}, Error, parser, Result, st::{self, Stringify, TermTrait}};
use std::any::Any;

pub type ValueGuts = dyn Any + Send + Sync;

/// This is the central runtime data type for sept.  Its methods will call into the corresponding
/// methods of the runtime.
// This really should be named Term (or more pedantically TermTerm), but that's already taken.
// Maybe the naming scheme can be shifted around so that this can be named Term (or TermTerm).
#[derive(derive_more::From, derive_more::Into)]
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
        Ok(RUNTIME_LA.read().unwrap().construct(self.as_ref(), parameter_t)?)
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
        RUNTIME_LA.read().unwrap().inhabits(self.as_ref(), rhs.as_ref())
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

impl st::Serializable for Value {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        Ok(RUNTIME_LA.read().unwrap().serialize(self.as_ref(), writer)?)
    }
}

impl Stringify for Value {
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
            let deconstruction: dy::Deconstruction = *dummy.0.downcast::<dy::Deconstruction>().unwrap();
            self.0 = Box::new(deconstruction.reconstruct()?);
        } else {
            // No need to reconstruct anything.
        }
        Ok(())
    }
}
