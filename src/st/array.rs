use crate::{dy::{self, DynNPTerm}, st::{ArrayType, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Array;

impl dy::IntoValue for Array {}

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

impl TypeTrait for Array {}

pub const ARRAY: Array = Array{};
