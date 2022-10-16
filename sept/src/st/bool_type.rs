use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct BoolType;

impl st::Inhabits<st::Type> for BoolType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for BoolType {
    fn identifier() -> &'static str {
        "BoolType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::BoolType
    }
}

impl Stringify for BoolType {
    fn stringify(&self) -> String {
        "BoolType".into()
    }
}
