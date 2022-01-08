use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
// TODO: AbstractTypeType could/should actually be "FormalTypeOf(ArrayType)"
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct ArrayType {}

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

pub const ARRAY_TYPE: ArrayType = ArrayType{};
