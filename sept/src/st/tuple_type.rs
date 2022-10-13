use crate::{dy::{self, NonParametricTermCode}, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct TupleType {}

impl dy::Deconstruct for TupleType {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for TupleType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for TupleType {
    fn identifier() -> &'static str {
        "TupleType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> NonParametricTermCode {
        NonParametricTermCode::TupleType
    }
}

impl Stringify for TupleType {
    fn stringify(&self) -> String {
        "TupleType".into()
    }
}
