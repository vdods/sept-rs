use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct TupleType {}

impl dy::IntoValue for TupleType {}

impl st::Inhabits<Type> for TupleType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for TupleType {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::TupleType
    }
}

impl Stringify for TupleType {
    fn stringify(&self) -> String {
        "TupleType".into()
    }
}

pub const TUPLE_TYPE: TupleType = TupleType{};
