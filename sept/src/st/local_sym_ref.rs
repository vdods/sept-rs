use crate::{
    dy,
    st::{self, InhabitsT, LocalSymRefType},
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
#[st_term_t(
    AbstractTypeType = "LocalSymRefType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct LocalSymRef;

impl InhabitsT<LocalSymRefType> for LocalSymRef {
    fn inhabits(&self, _: &LocalSymRefType) -> bool {
        true
    }
}

impl st::InhabitsT<st::Type> for LocalSymRef {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
