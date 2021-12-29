use crate::{DynNPTerm, Tuple, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TupleType {}

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

impl TypeTrait for TupleType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Tuple>()
    }
}

pub const TUPLE_TYPE: TupleType = TupleType{};
