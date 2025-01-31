use crate::{
    dy,
    st::{self, TermT, Type},
};

/// EmptyType is a Type that by definition has no inhabitants.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct EmptyType;

impl st::InhabitsT<Type> for EmptyType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl<T: TermT + dy::IntoValueT + 'static> st::InhabitsT<EmptyType> for T {
    /// Nothing inhabits EmptyType.
    fn inhabits(&self, _rhs: &EmptyType) -> bool {
        false
    }
}
