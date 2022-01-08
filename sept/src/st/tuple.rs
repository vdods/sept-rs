use crate::{dy::{self, DynNPTerm}, st::{self, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TupleType, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "TupleType", is_parametric = "false", is_type = "true")]
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

pub const TUPLE: Tuple = Tuple{};
