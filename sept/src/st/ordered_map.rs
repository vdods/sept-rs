use crate::{
    dy,
    st::{self, Inhabits, OrderedMapType},
    Result,
};
use std::{collections::BTreeMap, fmt::Debug};

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
    AbstractTypeType = "OrderedMapType",
    is_parametric = "false",
    is_type = "true"
)]
pub struct OrderedMap;

impl dy::Constructor for OrderedMap {
    type ConstructedType = dy::OrderedMapTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        // Each parameter is expected to be a 2-element TupleTerm, which represents a key-value pair.
        let parameter_v: Vec<dy::Value> = parameter_t.into();
        let key_value_m = parameter_v.into_iter().enumerate().map(|(i, parameter)| -> Result<(dy::Value, dy::Value)> {
            anyhow::ensure!(parameter.is::<dy::TupleTerm>(), "OrderedMap.construct expected each parameter to be a TupleTerm, but {}th parameter was not", i);
            let key_value_t = parameter.downcast_into::<dy::TupleTerm>();
            anyhow::ensure!(key_value_t.len() == 2, "OrderedMap.construct expected each parameter to be a 2-element TupleTerm, but {}th parameter had {} elements", i, key_value_t.len());
            let mut key_value_v: Vec<dy::Value> = key_value_t.into();
            let value = key_value_v.pop().unwrap();
            let key = key_value_v.pop().unwrap();
            Ok((key, value))
        }).collect::<Result<BTreeMap<dy::Value, dy::Value>>>()?;
        Ok(dy::OrderedMapTerm::from(key_value_m))
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        Ok(Self::ConstructedType::deserialize(reader)?)
    }
}

impl Inhabits<OrderedMapType> for OrderedMap {
    fn inhabits(&self, _: &OrderedMapType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for OrderedMap {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}
