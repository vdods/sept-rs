use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl TermTrait for GlobalSymRefType {
    // TODO: This could/should actually be FormalTypeOf(GlobalSymRefType)
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "GlobalSymRefType"
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

impl TypeTrait for GlobalSymRefType {}

pub const GLOBAL_SYM_REF_TYPE: GlobalSymRefType = GlobalSymRefType{};
