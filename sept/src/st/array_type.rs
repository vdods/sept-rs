use crate::{
    dy,
    st::{self, Type},
};
use std::fmt::Debug;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
// TODO: AbstractTypeType could/should actually be "FormalTypeOf(ArrayType)"
#[st_term_t(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct ArrayType;

impl st::InhabitsT<st::Type> for ArrayType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
