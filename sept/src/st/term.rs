use crate::{dy::{self, NonParametricTermCode}, st::{self, Inhabits, NonParametricTermTrait, Stringify, TermTrait, Type}};

/// This represents the NonParametricTerm `Term` itself, not the trait TermTrait.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Term;

impl dy::Deconstruct for Term {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

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
    fn identifier() -> &'static str {
        "Term"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> NonParametricTermCode {
        NonParametricTermCode::Term
    }
}

impl Stringify for Term {
    fn stringify(&self) -> String {
        "Term".into()
    }
}
