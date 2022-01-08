use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for LocalSymRefType {}

pub const LOCAL_SYM_REF_TYPE: LocalSymRefType = LocalSymRefType{};
