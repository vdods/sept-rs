use crate::{
    dy,
    st::{self, Inhabits, Stringifiable, Utf8StringType},
    Result,
};

/// This represents the Utf8String type itself, not a boolean value such as true or false.
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
    AbstractTypeType = "Utf8StringType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct Utf8String;

impl dy::Constructor for Utf8String {
    type ConstructedType = String;
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
                "{}.construct expected parameter of type Utf8String, but got one of type {:?}",
                self.stringify(),
                parameter.type_id()
            )),
        }
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl st::Inhabits<st::Type> for Utf8String {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl Inhabits<Utf8StringType> for Utf8String {
    fn inhabits(&self, _: &Utf8StringType) -> bool {
        true
    }
}
