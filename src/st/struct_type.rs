use crate::{dy::{self, DynNPTerm}, st::{NonParametricTermTrait, Stringify, Struct, TermTrait, Type, TypeTrait}};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StructType {}

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
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "StructType"
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

impl TypeTrait for StructType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Struct>()
    }
}

pub const STRUCT_TYPE: StructType = StructType{};
