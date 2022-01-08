use crate::{dy::{self, DynNPTerm}, st::{self, Inhabits, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};

/// This represents the NonParametricTerm `Term` itself, not the trait TermTrait.
#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Term;

impl dy::IntoValue for Term {}

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

impl NonParametricTermTrait for Term {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Term
    }
}

impl Stringify for Term {
    fn stringify(&self) -> String {
        "Term".into()
    }
}

impl TypeTrait for Term {}

pub const TERM: Term = Term{};
