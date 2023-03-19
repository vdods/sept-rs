use crate::{
    dy,
    st::{self, Inhabits, TupleType},
    Result,
};
use std::fmt::Debug;

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
#[st_term_trait(
    AbstractTypeType = "TupleType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Tuple;

impl dy::Constructor for Tuple {
    type ConstructedType = dy::TupleTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        // There's really nothing to do.  The parameters are already in the correct form,
        // because Tuple represents the collection of all TupleTerms, and there's no further
        // type checking.  Contrast with TupleTerm(...) which would type check its parameters.
        Ok(parameter_t)
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
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
