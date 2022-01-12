use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct BoolType;

impl dy::Deconstruct for BoolType {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Value::from(self).into()
    }
}

impl st::Inhabits<Type> for BoolType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for BoolType {
    fn as_dyn_npterm(&self) -> dy::DynNPTerm {
        dy::DynNPTerm::BoolType
    }
}

impl Stringify for BoolType {
    fn stringify(&self) -> String {
        "BoolType".into()
    }
}
