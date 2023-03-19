use crate::{
    dy,
    st::{self, Inhabits, TermTrait, Type},
};

/// This represents the NonParametricTerm `Term` itself, not the trait TermTrait.
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
pub struct Term;

/// Everything inhabits Term.
impl<T: TermTrait + dy::IntoValue + 'static> Inhabits<Term> for T {
    fn inhabits(&self, _: &Term) -> bool {
        true
    }
}

impl Inhabits<Type> for Term {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
