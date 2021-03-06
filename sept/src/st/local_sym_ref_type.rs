use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct LocalSymRefType {}

impl dy::Deconstruct for LocalSymRefType {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for LocalSymRefType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for LocalSymRefType {
    fn identifier() -> &'static str {
        "LocalSymRefType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::LocalSymRefType
    }
}

impl Stringify for LocalSymRefType {
    fn stringify(&self) -> String {
        "LocalSymRefType".into()
    }
}
