use crate::{dy::{self, DynNPTerm}, st::{self, GlobalSymRefType, Inhabits, NonParametricTermTrait, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "GlobalSymRefType", is_parametric = "false", is_type = "true")]
pub struct GlobalSymRef;

impl Inhabits<GlobalSymRefType> for GlobalSymRef {
    fn inhabits(&self, _: &GlobalSymRefType) -> bool {
        true
    }
}

impl NonParametricTermTrait for GlobalSymRef {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::GlobalSymRef
    }
}

impl Stringify for GlobalSymRef {
    fn stringify(&self) -> String {
        "GlobalSymRef".into()
    }
}

pub const GLOBAL_SYM_REF: GlobalSymRef = GlobalSymRef{};
