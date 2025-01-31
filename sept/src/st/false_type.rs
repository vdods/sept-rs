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
pub struct FalseType;

impl st::InhabitsT<Type> for FalseType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<st::BoolType> for FalseType {
    fn inhabits(&self, _rhs: &st::BoolType) -> bool {
        true
    }
}
