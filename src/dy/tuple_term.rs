use crate::{dy::Value, st::{Inhabits, Stringify, TermTrait, Tuple, TypeTrait}};
use std::any::Any;

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(derive_more::AsRef, Debug, derive_more::From, derive_more::Into, PartialEq)]
pub struct TupleTerm(Vec<Value>);

impl std::ops::Deref for TupleTerm {
    type Target = Vec<Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for TupleTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

// Hacky way to get Rust-syntax-tuple-valued constructors for a tuples of length 0-6.
// TODO: Maybe implement a macro to handle these.

impl From<()> for TupleTerm {
    fn from(_: ()) -> Self {
        vec![].into()
    }
}

impl<T0> From<(T0,)> for TupleTerm
where T0: TermTrait
{
    fn from(t: (T0,)) -> Self {
        vec![t.0.into()].into()
    }
}

impl<T0, T1> From<(T0, T1)> for TupleTerm
where T0: TermTrait, T1: TermTrait
{
    fn from(t: (T0, T1)) -> Self {
        vec![t.0.into(), t.1.into()].into()
    }
}

impl<T0, T1, T2> From<(T0, T1, T2)> for TupleTerm
where T0: TermTrait, T1: TermTrait, T2: TermTrait
{
    fn from(t: (T0, T1, T2)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into()].into()
    }
}

impl<T0, T1, T2, T3> From<(T0, T1, T2, T3)> for TupleTerm
where T0: TermTrait, T1: TermTrait, T2: TermTrait, T3: TermTrait
{
    fn from(t: (T0, T1, T2, T3)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into(), t.3.into()].into()
    }
}

impl<T0, T1, T2, T3, T4> From<(T0, T1, T2, T3, T4)> for TupleTerm
where T0: TermTrait, T1: TermTrait, T2: TermTrait, T3: TermTrait, T4: TermTrait
{
    fn from(t: (T0, T1, T2, T3, T4)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into(), t.3.into(), t.4.into()].into()
    }
}

impl<T0, T1, T2, T3, T4, T5> From<(T0, T1, T2, T3, T4, T5)> for TupleTerm
where T0: TermTrait, T1: TermTrait, T2: TermTrait, T3: TermTrait, T4: TermTrait, T5: TermTrait
{
    fn from(t: (T0, T1, T2, T3, T4, T5)) -> Self {
        vec![t.0.into(), t.1.into(), t.2.into(), t.3.into(), t.4.into(), t.5.into()].into()
    }
}

impl Inhabits<Tuple> for TupleTerm {
    fn inhabits(&self, _: &Tuple) -> bool {
        true
    }
}

impl Inhabits<TupleTerm> for TupleTerm {
    fn inhabits(&self, rhs: &TupleTerm) -> bool {
        if rhs.0.len() != self.0.len() {
            return false;
        }
        // TODO: Use std::iter::zip here when it's stable
        for i in 0..self.0.len() {
            if !self.0[i].inhabits(&rhs.0[i]) {
                return false;
            }
        }
        true
    }
}

impl Stringify for TupleTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Tuple(");
        for (i, element) in self.0.iter().enumerate() {
            s.push_str(&element.stringify());
            if i+1 < self.0.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl TermTrait for TupleTerm {
    type AbstractTypeFnReturnType = TupleTerm;

    fn label() -> &'static str {
        "TupleTerm"
    }
    /// A Tuple term is parametric if there is at least one parameter.
    fn is_parametric_term(&self) -> bool {
        self.0.len() > 0
    }
    /// A Tuple term is a type if all of its elements are types.
    fn is_type_term(&self) -> bool {
        self.0.iter().all(|element| element.is_type_term())
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        let mut type_element_v = Vec::new();
        for self_element in self.0.iter() {
            type_element_v.push(self_element.abstract_type());
        }
        TupleTerm(type_element_v)
    }
}

impl TypeTrait for TupleTerm {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        match x_.downcast_ref::<TupleTerm>() {
            Some(x_tuple_term) => x_tuple_term.inhabits(self),
            None => false
        }
    }
}
