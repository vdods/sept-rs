use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StructType {}

impl st::Inhabits<Type> for StructType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for StructType {}

impl NonParametricTermTrait for StructType {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::StructType
    }
}

impl Stringify for StructType {
    fn stringify(&self) -> String {
        "StructType".into()
    }
}

impl TermTrait for StructType {
    // TODO: This could/should actually be FormalTypeOf(StructType)
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "StructType"
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

impl TypeTrait for StructType {}

pub const STRUCT_TYPE: StructType = StructType{};
