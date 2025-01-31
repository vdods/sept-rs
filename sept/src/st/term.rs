use crate::{
    dy,
    st::{self, InhabitsT, TermT, Type},
};

/// This represents the NonParametricTerm `Term` itself, not the trait TermT.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Term;

/// Everything inhabits Term.
impl<T: TermT + dy::IntoValueT + 'static> InhabitsT<Term> for T {
    fn inhabits(&self, _: &Term) -> bool {
        true
    }
}

impl InhabitsT<Type> for Term {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
