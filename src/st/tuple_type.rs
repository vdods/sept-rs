use crate::{dy::{self, DynNPTerm}, st::{NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TupleType {}

impl dy::IntoValue for TupleType {}

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

impl TermTrait for TupleType {
    // TODO: This could/should actually be FormalTypeOf(TupleType)
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "TupleType"
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

impl TypeTrait for TupleType {}

pub const TUPLE_TYPE: TupleType = TupleType{};
