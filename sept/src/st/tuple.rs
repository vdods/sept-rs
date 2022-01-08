use crate::{dy::{self, DynNPTerm}, st::{Inhabits, NonParametricTermTrait, Stringify, TermTrait, TupleType, TypeTrait}};
use std::fmt::Debug;

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
    type AbstractTypeType = TupleType;

    fn label() -> &'static str {
        "Tuple"
    }
    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TypeTrait for Tuple {}

pub const TUPLE: Tuple = Tuple{};
