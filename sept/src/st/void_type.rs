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
    fn identifier() -> &'static str {
        "VoidType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> dy::NonParametricTermCode {
        dy::NonParametricTermCode::VoidType
    }
}

impl Stringify for VoidType {
    fn stringify(&self) -> String {
        "VoidType".into()
    }
}
