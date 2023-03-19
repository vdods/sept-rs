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
// TODO: AbstractTypeType could/should actually be FormalTypeOf(GlobalSymRefType)
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct GlobalSymRefType {}

impl st::Inhabits<st::Type> for GlobalSymRefType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
