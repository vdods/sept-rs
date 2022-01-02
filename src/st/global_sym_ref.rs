use crate::{dy::{self, DynNPTerm, GlobalSymRefTerm}, st::{GlobalSymRefType, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::{any::Any, fmt::Debug};

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
    type AbstractTypeFnReturnType = GlobalSymRefType;

    fn label() -> &'static str {
        "GlobalSymRef"
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

impl TypeTrait for GlobalSymRef {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<GlobalSymRefTerm>()
    }
}

pub const GLOBAL_SYM_REF: GlobalSymRef = GlobalSymRef{};
