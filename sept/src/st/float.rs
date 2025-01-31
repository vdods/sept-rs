use crate::{
    dy,
    st::{self, InhabitsT, StringifiableT},
    Result,
};
use std::fmt::Debug;

/// This represents the Float32 type itself.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::Float32Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Float32;

/// This represents the Float64 type itself.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::Float64Type",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Float64;

impl dy::ConstructorT for Float32 {
    type ConstructedType = f32;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(
            parameter_t.len() == 1,
            "{}.construct expected 1 parameter, got {}",
            self.stringify(),
            parameter_t.len()
        );
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<f32>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!(
                "{}.construct expected parameter of type Float32, but got one of type {:?}",
                self.stringify(),
                parameter.type_id()
            )),
        }
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::DeserializableT;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl dy::ConstructorT for Float64 {
    type ConstructedType = f64;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(
            parameter_t.len() == 1,
            "{}.construct expected 1 parameter, got {}",
            self.stringify(),
            parameter_t.len()
        );
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<f64>() {
            Some(x) => Ok(std::mem::take(x)),
            None => Err(anyhow::anyhow!(
                "{}.construct expected parameter of type Float64, but got one of type {:?}",
                self.stringify(),
                parameter.type_id()
            )),
        }
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::DeserializableT;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl InhabitsT<st::Float32Type> for Float32 {
    fn inhabits(&self, _: &st::Float32Type) -> bool {
        true
    }
}

impl InhabitsT<st::Float64Type> for Float64 {
    fn inhabits(&self, _: &st::Float64Type) -> bool {
        true
    }
}

impl st::InhabitsT<st::Type> for Float32 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::InhabitsT<st::Type> for Float64 {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
