use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
// TODO: AbstractTypeType could/should actually be FormalTypeOf(GlobalSymRefType)
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct GlobalSymRefType {}

impl st::Inhabits<Type> for GlobalSymRefType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for GlobalSymRefType {}

impl NonParametricTermTrait for GlobalSymRefType {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::GlobalSymRefType
    }
}

impl Stringify for GlobalSymRefType {
    fn stringify(&self) -> String {
        "GlobalSymRefType".into()
    }
}

pub const GLOBAL_SYM_REF_TYPE: GlobalSymRefType = GlobalSymRefType{};
