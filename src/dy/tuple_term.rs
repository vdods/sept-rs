use crate::{dy::Value, st::{Inhabits, Stringify, TermTrait, Tuple, TypeTrait}};
use std::any::Any;

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(Debug, derive_more::From, derive_more::Into, PartialEq)]
pub struct TupleTerm(Vec<Value>);

impl std::fmt::Display for TupleTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
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
