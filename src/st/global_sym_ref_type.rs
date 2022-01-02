use crate::{dy::{self, DynNPTerm}, st::{GlobalSymRef, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalSymRefType {}

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

impl TermTrait for GlobalSymRefType {
    // TODO: This could/should actually be FormalTypeOf(GlobalSymRefType)
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "GlobalSymRefType"
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

impl TypeTrait for GlobalSymRefType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<GlobalSymRef>()
    }
}

pub const GLOBAL_SYM_REF_TYPE: GlobalSymRefType = GlobalSymRefType{};
