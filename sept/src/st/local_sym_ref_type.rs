use crate::{dy, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct LocalSymRefType {}

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
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::LocalSymRefType
    }
}

impl Stringify for LocalSymRefType {
    fn stringify(&self) -> String {
        "LocalSymRefType".into()
    }
}
