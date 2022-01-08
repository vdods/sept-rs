use crate::{dy::{self, IntoValue, RUNTIME_LA}, st::{self, Stringify, TermTrait}};
use std::any::Any;

pub type ValueGuts = dyn Any + Send + Sync;

/// This is the central runtime data type for sept.  Its methods will call into the corresponding
/// methods of the runtime.
// This really should be named Term (or more pedantically TermTerm), but that's already taken.
// Maybe the naming scheme can be shifted around so that this can be named Term (or TermTerm).
#[derive(Debug, derive_more::From, derive_more::Into)]
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
    pub fn dereferenced<'a>(&'a self) -> anyhow::Result<dy::MaybeDereferencedValue<'a>> {
        Ok(RUNTIME_LA.read().unwrap().dereferenced(self.as_ref())?)
    }
}
