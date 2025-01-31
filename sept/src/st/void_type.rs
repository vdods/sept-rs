use crate::{
    dy,
    st::{self, Type},
};

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValueT,
    st::NonParametricTermT,
    PartialEq,
    st::TermT,
    st::TypeT,
)]
#[st_term_t(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct VoidType;

impl st::InhabitsT<st::Type> for VoidType {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}
