use crate::{dy, st::{self, Stringify, TermTrait, Type}};

/// EmptyType is a Type that by definition has no inhabitants.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct EmptyType;

impl st::Inhabits<Type> for EmptyType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl<T: TermTrait + dy::IntoValue + 'static> st::Inhabits<EmptyType> for T {
    /// Nothing inhabits EmptyType.
    fn inhabits(&self, _rhs: &EmptyType) -> bool {
        false
    }
}

impl st::NonParametricTermTrait for EmptyType {
    fn identifier() -> &'static str {
        "EmptyType"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::EmptyType
    }
}

impl Stringify for EmptyType {
    fn stringify(&self) -> String {
        "EmptyType".into()
    }
}
