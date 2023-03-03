use crate::{dy, Result, st::{self, ArrayType, Inhabits}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "ArrayType", is_parametric = "false", is_type = "true")]
pub struct Array;

impl dy::Constructor for Array {
    type ConstructedType = dy::ArrayTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        // Take the parameter elements directly.
        let parameter_v: Vec<dy::Value> = parameter_t.into();
        Ok(dy::ArrayTerm::from(parameter_v))
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl Inhabits<ArrayType> for Array {
    fn inhabits(&self, _: &ArrayType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Array {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
