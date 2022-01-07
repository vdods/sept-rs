use crate::{dy::{self, DynNPTerm}, st::{LocalSymRefType, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalSymRef;

impl dy::IntoValue for LocalSymRef {}

impl Inhabits<LocalSymRefType> for LocalSymRef {
    fn inhabits(&self, _: &LocalSymRefType) -> bool {
        true
    }
}

impl NonParametricTermTrait for LocalSymRef {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::LocalSymRef
    }
}

impl Stringify for LocalSymRef {
    fn stringify(&self) -> String {
        "LocalSymRef".into()
    }
}

impl TermTrait for LocalSymRef {
    type AbstractTypeFnReturnType = LocalSymRefType;

    fn label() -> &'static str {
        "LocalSymRef"
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

impl TypeTrait for LocalSymRef {}

pub const LOCAL_SYM_REF: LocalSymRef = LocalSymRef{};
