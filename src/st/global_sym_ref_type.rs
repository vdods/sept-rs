use crate::{dy::{self, DynNPTerm}, st::{NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

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

impl TypeTrait for GlobalSymRefType {}

pub const GLOBAL_SYM_REF_TYPE: GlobalSymRefType = GlobalSymRefType{};
