use crate::{
    dy,
    st::{self, InhabitsT},
};

/// This represents the NonParametricType `Type` itself, not the trait TypeT.
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
/// The abstract type of Type is Type itself.  This may or may not cause problems,
/// but let's go with it for now!  Really, it should be FormalTypeOf(Type).
#[st_term_t(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Type;

impl InhabitsT<Type> for Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
