use crate::{
    dy,
    st::{self, Type},
};
use std::fmt::Debug;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
// TODO: AbstractTypeType could/should actually be "FormalTypeOf(OrderedMapType)"
#[st_term_t(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct OrderedMapType;

impl st::InhabitsT<st::Type> for OrderedMapType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
