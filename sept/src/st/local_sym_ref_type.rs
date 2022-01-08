use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalSymRefType {}

impl st::Inhabits<Type> for LocalSymRefType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for LocalSymRefType {}

impl NonParametricTermTrait for LocalSymRefType {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::LocalSymRefType
    }
}

impl Stringify for LocalSymRefType {
    fn stringify(&self) -> String {
        "LocalSymRefType".into()
    }
}

impl TermTrait for LocalSymRefType {
    // TODO: This could/should actually be FormalTypeOf(LocalSymRefType)
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "LocalSymRefType"
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

impl TypeTrait for LocalSymRefType {}

pub const LOCAL_SYM_REF_TYPE: LocalSymRefType = LocalSymRefType{};
