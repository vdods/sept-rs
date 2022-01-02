use crate::{dy::{self, DynNPTerm, StructTerm}, st::{Inhabits, NonParametricTermTrait, Stringify, StructType, TermTrait, TypeTrait}};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Struct;

impl dy::IntoValue for Struct {}

impl Inhabits<StructType> for Struct {
    fn inhabits(&self, _: &StructType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Struct {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Struct
    }
}

impl Stringify for Struct {
    fn stringify(&self) -> String {
        "Struct".into()
    }
}

impl TermTrait for Struct {
    type AbstractTypeFnReturnType = StructType;

    fn label() -> &'static str {
        "Struct"
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

impl TypeTrait for Struct {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<StructTerm>()
    }
}

pub const STRUCT: Struct = Struct{};
