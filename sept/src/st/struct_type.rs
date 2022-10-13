use crate::{dy::{self, NonParametricTermCode}, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct StructType;

impl dy::Deconstruct for StructType {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for StructType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for StructType {
    fn identifier() -> &'static str {
        "StructType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> NonParametricTermCode {
        NonParametricTermCode::StructType
    }
}

impl Stringify for StructType {
    fn stringify(&self) -> String {
        "StructType".into()
    }
}
