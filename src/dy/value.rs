use crate::{dy::{IntoValue, RUNTIME}, st::{self, Stringify, TermTrait}};
use std::any::Any;

pub type ValueGuts = dyn Any + Send + Sync;

/// This is the central runtime data type for sept.  Its methods will call into the corresponding
/// methods of the runtime.
// This really should be named Term (or more pedantically TermTerm), but that's already taken.
// Maybe the naming scheme can be shifted around so that this can be named Term (or TermTerm).
#[derive(derive_more::From, derive_more::Into)]
pub struct Value(Box<ValueGuts>);

impl AsRef<ValueGuts> for Value {
    fn as_ref(&self) -> &ValueGuts {
        self.0.as_ref()
    }
}

impl std::ops::Deref for Value {
    type Target = ValueGuts;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Value({})", &self.stringify())
    }
}

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
        RUNTIME.read().unwrap().inhabits(self.as_ref(), rhs.as_ref())
    }
}

impl<T: st::TypeTrait + IntoValue + 'static> st::Inhabits<T> for Value {
    fn inhabits(&self, rhs: &T) -> bool {
        let rhs_: &ValueGuts = rhs;
        RUNTIME.read().unwrap().inhabits(self.as_ref(), rhs_)
    }
}

impl PartialEq<Value> for Value {
    fn eq(&self, other: &Value) -> bool {
        RUNTIME.read().unwrap().eq(self.as_ref(), other.as_ref())
    }
}

impl Stringify for Value {
    fn stringify(&self) -> String {
        RUNTIME.read().unwrap().stringify(self.as_ref())
    }
}

impl TermTrait for Value {
    type AbstractTypeFnReturnType = Value;

    fn label() -> &'static str {
        "Value"
    }
    fn is_parametric_term(&self) -> bool {
        RUNTIME.read().unwrap().is_parametric_term(self.as_ref())
    }
    fn is_type_term(&self) -> bool {
        RUNTIME.read().unwrap().is_type_term(self.as_ref())
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Value(RUNTIME.read().unwrap().abstract_type_of(self.as_ref()))
    }
}

impl st::TypeTrait for Value {}
