use crate::{dy, st::{self, LocalSymRefType, Inhabits, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "LocalSymRefType", is_parametric = "false", is_type = "true")]
pub struct LocalSymRef;

impl Inhabits<LocalSymRefType> for LocalSymRef {
    fn inhabits(&self, _: &LocalSymRefType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for LocalSymRef {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl Stringify for LocalSymRef {
    fn stringify(&self) -> String {
        "LocalSymRef".into()
    }
}
