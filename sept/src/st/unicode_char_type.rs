use crate::{
    dy,
    st::{self, Type},
};

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
pub struct UnicodeCharType;

impl st::Inhabits<st::Type> for UnicodeCharType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
