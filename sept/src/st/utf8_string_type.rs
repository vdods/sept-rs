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
pub struct Utf8StringType;

impl st::InhabitsT<Type> for Utf8StringType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
