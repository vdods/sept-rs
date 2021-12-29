use crate::{ArrayTerm, ArrayType, DynNPTerm, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Array;

impl Inhabits<ArrayType> for Array {
    fn inhabits(&self, _: &ArrayType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Array {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Array
    }
}

impl Stringify for Array {
    fn stringify(&self) -> String {
        "Array".into()
    }
}

impl TermTrait for Array {
    type AbstractTypeFnReturnType = ArrayType;

    fn label() -> &'static str {
        "Array"
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

impl TypeTrait for Array {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<ArrayTerm>()
    }
}

pub const ARRAY: Array = Array{};
