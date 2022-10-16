use crate::{dy, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
// TODO: AbstractTypeType could/should actually be FormalTypeOf(GlobalSymRefType)
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct GlobalSymRefType {}

impl st::Inhabits<st::Type> for GlobalSymRefType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for GlobalSymRefType {
    fn identifier() -> &'static str {
        "GlobalSymRefType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::GlobalSymRefType
    }
}

impl Stringify for GlobalSymRefType {
    fn stringify(&self) -> String {
        "GlobalSymRefType".into()
    }
}
