use crate::{
    dy,
    st::{self, ArrayType, InhabitsT},
    Result,
};
use std::fmt::Debug;

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "ArrayType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Array;

impl dy::ConstructorT for Array {
    type ConstructedType = dy::ArrayTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        // Take the parameter elements directly.
        let parameter_v: Vec<dy::Value> = parameter_t.into();
        Ok(dy::ArrayTerm::from(parameter_v))
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::DeserializableT;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl InhabitsT<ArrayType> for Array {
    fn inhabits(&self, _: &ArrayType) -> bool {
        true
    }
}

impl st::InhabitsT<st::Type> for Array {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
