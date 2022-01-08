use crate::{dy::{self, DynNPTerm}, st::{self, LocalSymRefType, Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "LocalSymRefType", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for LocalSymRef {}

pub const LOCAL_SYM_REF: LocalSymRef = LocalSymRef{};
