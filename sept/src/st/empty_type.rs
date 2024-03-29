use crate::{
    dy,
    st::{self, TermTrait, Type},
};

/// EmptyType is a Type that by definition has no inhabitants.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
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
