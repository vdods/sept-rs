use crate::{dy::{self, DynNPTerm}, Result, st::{self, Inhabits, NonParametricTermTrait, Stringify, TupleType}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "TupleType", is_parametric = "false", is_type = "true")]
pub struct Tuple;

impl dy::Constructor for Tuple {
    type ConstructedType = dy::TupleTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        // There's really nothing to do.  The parameters are already in the correct form,
        // because Tuple represents the collection of all TupleTerms, and there's no further
        // type checking.  Contrast with TupleTerm(...) which would type check its parameters.
        Ok(parameter_t)
    }
}

impl dy::Deconstruct for Tuple {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl Inhabits<TupleType> for Tuple {
    fn inhabits(&self, _: &TupleType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Tuple {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for Tuple {
    fn identifier() -> &'static str {
        "Tuple"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Tuple
    }
}

impl Stringify for Tuple {
    fn stringify(&self) -> String {
        "Tuple".into()
    }
}
