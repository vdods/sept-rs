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
pub struct Sint8Type;

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
pub struct Sint16Type;

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
pub struct Sint32Type;

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
pub struct Sint64Type;

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
pub struct Uint8Type;

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
pub struct Uint16Type;

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
pub struct Uint32Type;

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
pub struct Uint64Type;

impl st::InhabitsT<Type> for Sint8Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Sint16Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Sint32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Sint64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Uint8Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Uint16Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Uint32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::InhabitsT<Type> for Uint64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
