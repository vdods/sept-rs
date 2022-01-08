use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArrayType {}

impl dy::IntoValue for ArrayType {}

impl st::Inhabits<Type> for ArrayType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

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
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "ArrayType"
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

impl TypeTrait for ArrayType {}

pub const ARRAY_TYPE: ArrayType = ArrayType{};
