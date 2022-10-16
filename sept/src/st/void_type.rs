use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct VoidType;

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
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::VoidType
    }
}

impl Stringify for VoidType {
    fn stringify(&self) -> String {
        "VoidType".into()
    }
}
