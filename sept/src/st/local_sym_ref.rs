use crate::{dy::{self, DynNPTerm}, st::{self, LocalSymRefType, Inhabits, NonParametricTermTrait, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "LocalSymRefType", is_parametric = "false", is_type = "true")]
pub struct LocalSymRef;

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
