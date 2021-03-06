use crate::{dy::{self, DynNPTerm}, st::{self, GlobalSymRefType, Inhabits, NonParametricTermTrait, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "GlobalSymRefType", is_parametric = "false", is_type = "true")]
pub struct GlobalSymRef;

impl dy::Deconstruct for GlobalSymRef {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl Inhabits<GlobalSymRefType> for GlobalSymRef {
    fn inhabits(&self, _: &GlobalSymRefType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for GlobalSymRef {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for GlobalSymRef {
    fn identifier() -> &'static str {
        "GlobalSymRef"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::GlobalSymRef
    }
}

impl Stringify for GlobalSymRef {
    fn stringify(&self) -> String {
        "GlobalSymRef".into()
    }
}
