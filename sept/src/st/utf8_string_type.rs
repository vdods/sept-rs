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
pub struct UTF8StringType;

impl st::InhabitsT<Type> for UTF8StringType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
