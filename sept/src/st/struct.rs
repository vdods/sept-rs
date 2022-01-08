use crate::{dy::{self, DynNPTerm}, st::{Inhabits, NonParametricTermTrait, Stringify, StructType, TermTrait, TypeTrait}};
use std::fmt::Debug;

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
    type AbstractTypeType = StructType;

    fn label() -> &'static str {
        "Struct"
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

impl TypeTrait for Struct {}

pub const STRUCT: Struct = Struct{};
