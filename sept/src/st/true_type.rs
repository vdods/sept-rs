use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct TrueType;

impl dy::Deconstruct for TrueType {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for TrueType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<st::BoolType> for TrueType {
    fn inhabits(&self, _rhs: &st::BoolType) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for TrueType {
    fn identifier() -> &'static str {
        "TrueType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> dy::DynNPTerm {
        dy::DynNPTerm::TrueType
    }
}

impl Stringify for TrueType {
    fn stringify(&self) -> String {
        "TrueType".into()
    }
}
