use crate::{
    dy,
    st::{self, Type},
};
use std::fmt::Debug;

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
pub struct Float32Type;

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
pub struct Float64Type;

impl st::InhabitsT<Type> for Float32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Float64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
