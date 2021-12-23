use crate::{Inhabits, NonParametricTermTrait, DynNPTerm, Stringify, TermTrait, Type};

/// This represents the NonParametricTerm `Term` itself, not the trait TermTrait.
#[derive(Debug, Eq, PartialEq)]
pub struct Term;

/// Everything inhabits Term.
impl<T: TermTrait> Inhabits<Term> for T {
    fn inhabits(&self, _: &Term) -> bool {
        true
    }
}

impl Stringify for Term {
    fn stringify(&self) -> String {
        "Term".into()
    }
}

impl TermTrait for Term {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "Term"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl NonParametricTermTrait for Term {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Term
    }
}

pub const TERM: Term = Term{};
