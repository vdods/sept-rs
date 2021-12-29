use crate::{dy::DynNPTerm, st::{Inhabits, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};

/// This represents the NonParametricTerm `Term` itself, not the trait TermTrait.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Term;

/// Everything inhabits Term.
impl<T: TermTrait> Inhabits<Term> for T {
    fn inhabits(&self, _: &Term) -> bool {
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

impl TypeTrait for Term {
    /// All terms inhabit Term.
    fn has_inhabitant(&self, _: &impl TermTrait) -> bool {
        true
    }
}

pub const TERM: Term = Term{};
