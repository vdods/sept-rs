use crate::{NonParametricTermTrait, DynNPTerm, Stringify, TermTrait};

/// This represents the NonParametricTerm `Term` itself, not the trait TermTrait.
#[derive(Debug, Eq, PartialEq)]
pub struct Term;

impl Stringify for Term {
    fn stringify(&self) -> String {
        "Term".into()
    }
}

impl TermTrait for Term {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl NonParametricTermTrait for Term {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Term
    }
}

pub const TERM: Term = Term{};
