use crate::{
    dy,
    st::{self, InhabitsT, UnicodeCharType},
    Result,
};

/// This represents the UnicodeChar type itself, not a boolean value such as true or false.
#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    dy::IntoValueT,
    st::NonParametricTermT,
    PartialEq,
    st::TermT,
    st::TypeT,
)]
#[st_term_t(
    AbstractTypeType = "UnicodeCharType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct UnicodeChar;

impl dy::ConstructorT for UnicodeChar {
    type ConstructedType = UnicodeChar;
    fn construct(&self, _parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        unimplemented!("TODO");
        // anyhow::ensure!(
        //     parameter_t.len() == 1,
        //     "{}.construct expected 1 parameter, got {}",
        //     self.stringify(),
        //     parameter_t.len()
        // );
        // let mut parameter_v: Vec<dy::Value> = parameter_t.into();
        // let mut parameter: dy::Value = parameter_v.pop().unwrap();
        // match parameter.downcast_mut::<char>() {
        //     Some(string) => Ok(std::mem::take(string)),
        //     None => Err(anyhow::anyhow!(
        //         "{}.construct expected parameter of type UnicodeChar, but got one of type {:?}",
        //         self.stringify(),
        //         parameter.type_id()
        //     )),
        // }
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::DeserializableT;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl InhabitsT<UnicodeCharType> for UnicodeChar {
    fn inhabits(&self, _: &UnicodeCharType) -> bool {
        true
    }
}

impl st::InhabitsT<st::Type> for UnicodeChar {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
