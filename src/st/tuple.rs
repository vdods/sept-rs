use crate::{dy::{self, DynNPTerm, TupleTerm}, st::{Inhabits, NonParametricTermTrait, Stringify, TermTrait, TupleType, TypeTrait}};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Tuple;

impl dy::IntoValue for Tuple {}

impl Inhabits<TupleType> for Tuple {
    fn inhabits(&self, _: &TupleType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Tuple {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Tuple
    }
}

impl Stringify for Tuple {
    fn stringify(&self) -> String {
        "Tuple".into()
    }
}

impl TermTrait for Tuple {
    type AbstractTypeFnReturnType = TupleType;

    fn label() -> &'static str {
        "Tuple"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TypeTrait for Tuple {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<TupleTerm>()
    }
}

pub const TUPLE: Tuple = Tuple{};
