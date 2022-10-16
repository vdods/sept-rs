use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Utf8StringType;

impl st::Inhabits<Type> for Utf8StringType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for Utf8StringType {
    fn identifier() -> &'static str {
        "Utf8StringType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Utf8StringType
    }
}

impl Stringify for Utf8StringType {
    fn stringify(&self) -> String {
        "Utf8StringType".into()
    }
}
