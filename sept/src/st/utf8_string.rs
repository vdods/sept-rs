use crate::{
    dy,
    st::{self, InhabitsT, StringifiableT},
    Result,
};

/// This represents the UTF8String type itself, not a boolean value such as true or false.
#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT, st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "st::UTF8StringType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct UTF8String;

impl dy::ConstructorT for UTF8String {
    type ConstructedType = st::UTF8StringTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        anyhow::ensure!(
            parameter_t.len() == 1,
            "{}.construct expected 1 parameter, got {}",
            self.stringify(),
            parameter_t.len()
        );
        let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        let mut parameter: dy::Value = parameter_v.pop().unwrap();
        match parameter.downcast_mut::<String>() {
            Some(string) => Ok(std::mem::take(string)),
            None => Err(anyhow::anyhow!(
                "{}.construct expected parameter of type UTF8String, but got one of type {:?}",
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

impl st::InhabitsT<st::Type> for UTF8String {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl InhabitsT<st::UTF8StringType> for UTF8String {
    fn inhabits(&self, _: &st::UTF8StringType) -> bool {
        true
    }
}
