use crate::{dy::{IntoValue, RUNTIME}, st::{Stringify, TermTrait, TypeTrait}};
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

impl Value {
    pub fn new<T: TermTrait + 'static>(t: T) -> Self {
        let b: Box<ValueGuts> = Box::new(t);
        Self(b)
    }
    pub fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &ValueGuts = x;
        RUNTIME.read().unwrap().inhabits(self.as_ref(), x_)
    }
    pub fn inhabits(&self, t: impl AsRef<ValueGuts>) -> bool {
        RUNTIME.read().unwrap().inhabits(self.as_ref(), t.as_ref())
    }
    pub fn inhabits_type(&self, t: &impl TypeTrait) -> bool {
        let t_: &ValueGuts = t;
        RUNTIME.read().unwrap().inhabits(self.as_ref(), t_)
    }

    pub fn downcast_ref<T: TermTrait + 'static>(&self) -> Option<&T> {
        self.as_ref().downcast_ref::<T>()
    }
}
