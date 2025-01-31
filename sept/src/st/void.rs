use crate::{
    dy,
    st::{self, InhabitsT, VoidType},
};

/// This represents the Void term itself.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT,
)]
#[st_term_t(
    AbstractTypeType = "VoidType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct Void;

impl InhabitsT<VoidType> for Void {
    fn inhabits(&self, _: &VoidType) -> bool {
        true
    }
}
