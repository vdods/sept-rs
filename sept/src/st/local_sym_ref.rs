use crate::{dy, st::{self, LocalSymRefType, Inhabits, NonParametricTermTrait, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
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

impl NonParametricTermTrait for LocalSymRef {
    fn identifier() -> &'static str {
        "LocalSymRef"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::LocalSymRef
    }
}

impl Stringify for LocalSymRef {
    fn stringify(&self) -> String {
        "LocalSymRef".into()
    }
}
