use crate::{
    dy,
    st::{self, InhabitsT, PlaceholderType},
};

/// This represents the Placeholder term itself.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT,
)]
#[st_term_t(
    AbstractTypeType = "PlaceholderType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct Placeholder;

impl InhabitsT<PlaceholderType> for Placeholder {
    fn inhabits(&self, _: &PlaceholderType) -> bool {
        true
    }
}
