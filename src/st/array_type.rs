use crate::{dy::{self, DynNPTerm}, st::{NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayType {}

impl dy::IntoValue for ArrayType {}

impl NonParametricTermTrait for ArrayType {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::ArrayType
    }
}

impl Stringify for ArrayType {
    fn stringify(&self) -> String {
        "ArrayType".into()
    }
}

impl TermTrait for ArrayType {
    // TODO: This could/should actually be FormalTypeOf(ArrayType)
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "ArrayType"
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

impl TypeTrait for ArrayType {}

pub const ARRAY_TYPE: ArrayType = ArrayType{};
