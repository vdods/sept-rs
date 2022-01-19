use crate::{dy::{self, DynNPTerm}, st::{self, Inhabits, NonParametricTermTrait, Stringify, VoidType}};

/// This represents the Void term itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "VoidType", is_parametric = "false", is_type = "false")]
pub struct Void;

impl dy::Deconstruct for Void {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl Inhabits<VoidType> for Void {
    fn inhabits(&self, _: &VoidType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Void {
    fn identifier() -> &'static str {
        "Void"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Void
    }
}

impl Stringify for Void {
    fn stringify(&self) -> String {
        "Void".into()
    }
}
