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
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Float32Type;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValue,
    st::NonParametricTermTrait,
    PartialEq,
    st::TermTrait,
    st::TypeTrait,
)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Float64Type;

impl st::Inhabits<Type> for Float32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Float64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
