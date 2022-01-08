use crate::{dy::{self, DynNPTerm}, st::{GlobalSymRefType, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalSymRef;

impl dy::IntoValue for GlobalSymRef {}

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

impl TermTrait for GlobalSymRef {
    type AbstractTypeType = GlobalSymRefType;

    fn label() -> &'static str {
        "GlobalSymRef"
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

impl TypeTrait for GlobalSymRef {}

pub const GLOBAL_SYM_REF: GlobalSymRef = GlobalSymRef{};
