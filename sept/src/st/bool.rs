use crate::{
    dy,
    st::{self, BoolType, InhabitsT, StringifiableT},
    Result,
};

/// This represents the Bool type itself, not a boolean value such as true or false.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "BoolType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Bool;

impl dy::ConstructorT for Bool {
    type ConstructedType = bool;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(
            parameter_t.len() == 1,
            "{}.construct expected 1 parameter, got {}",
            self.stringify(),
            parameter_t.len()
        );
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<bool>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!(
                "{}.construct expected parameter of type Bool, but got one of type {:?}",
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

impl InhabitsT<BoolType> for Bool {
    fn inhabits(&self, _: &BoolType) -> bool {
        true
    }
}

impl st::InhabitsT<st::Type> for Bool {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
