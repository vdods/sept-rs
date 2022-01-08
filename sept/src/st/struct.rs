use crate::{dy::{self, DynNPTerm}, st::{self, Inhabits, NonParametricTermTrait, Stringify, StructType, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "StructType", is_parametric = "false", is_type = "true")]
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

pub const STRUCT: Struct = Struct{};
