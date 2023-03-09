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
// TODO: AbstractTypeType could/should actually be "FormalTypeOf(OrderedMapType)"
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct OrderedMapType;

impl st::Inhabits<st::Type> for OrderedMapType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
