use crate::{
    dy,
    st::{self, Inhabits, PlaceholderType},
};

/// This represents the Placeholder term itself.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_term_trait(
    AbstractTypeType = "PlaceholderType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct Placeholder;

impl Inhabits<PlaceholderType> for Placeholder {
    fn inhabits(&self, _: &PlaceholderType) -> bool {
        true
    }
}
