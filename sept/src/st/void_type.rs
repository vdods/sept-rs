use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct VoidType;

impl dy::Deconstruct for VoidType {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<st::Type> for VoidType {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for VoidType {
    fn as_dyn_npterm(&self) -> dy::DynNPTerm {
        dy::DynNPTerm::VoidType
    }
}

impl Stringify for VoidType {
    fn stringify(&self) -> String {
        "VoidType".into()
    }
}
