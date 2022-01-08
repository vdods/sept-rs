use crate::{dy::{self, DynNPTerm}, st::{self, ArrayType, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "ArrayType", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for Array {}

pub const ARRAY: Array = Array{};
