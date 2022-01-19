use crate::{dy::{self, DynNPTerm}, st::{self, LocalSymRefType, Inhabits, NonParametricTermTrait, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "LocalSymRefType", is_parametric = "false", is_type = "true")]
pub struct LocalSymRef;

impl dy::Deconstruct for LocalSymRef {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

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
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::LocalSymRef
    }
}

impl Stringify for LocalSymRef {
    fn stringify(&self) -> String {
        "LocalSymRef".into()
    }
}
