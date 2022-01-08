use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for StructType {}

pub const STRUCT_TYPE: StructType = StructType{};
