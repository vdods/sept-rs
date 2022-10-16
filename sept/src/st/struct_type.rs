use crate::{dy, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct StructType;

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
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::StructType
    }
}

impl Stringify for StructType {
    fn stringify(&self) -> String {
        "StructType".into()
    }
}
