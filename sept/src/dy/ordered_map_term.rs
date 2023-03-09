use crate::{
    dy,
    st::{self, Inhabits, OrderedMap, Stringifiable},
    Result,
};
use std::collections::BTreeMap;

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(
    Clone,
    Debug,
    derive_more::Deref,
    derive_more::DerefMut,
    derive_more::From,
    derive_more::Into,
    dy::IntoValue,
    PartialEq,
    st::TermTrait,
)]
#[st_term_trait(
    AbstractTypeType = "OrderedMap",
    is_parametric = "self.0.len() > 0",
    is_type = "true"
)]
pub struct OrderedMapTerm(BTreeMap<dy::Value, dy::Value>);

impl dy::Deconstruct for OrderedMapTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        // Turn the BTreeMap elements (i.e. key-value pairs) into TupleTerms, and then collect that into a TupleTerm.
        // Could create MappingTerm, Mapping, and MappingType terms for better semantics.
        dy::ParametricDeconstruction::new_recursive(
            OrderedMap.into(),
            self.0
                .into_iter()
                .map(|(key, value)| dy::TupleTerm::from(vec![key, value]).into())
                .collect::<Vec<dy::Value>>()
                .into(),
        )
        .into()
    }
}

impl std::fmt::Display for OrderedMapTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl Inhabits<OrderedMap> for OrderedMapTerm {
    fn inhabits(&self, _: &OrderedMap) -> bool {
        true
    }
}

impl st::Serializable for OrderedMapTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where an OrderedMapTerm smaller than 8 bytes is encoded in exactly 8 bytes.
        let mut bytes_written = (self.len() as u64).serialize(writer)?;
        for (key, value) in self.iter() {
            bytes_written += key.serialize(writer)?;
            bytes_written += value.serialize(writer)?;
        }
        Ok(bytes_written)
    }
}

impl Stringifiable for OrderedMapTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("OrderedMap(");
        for (i, (key, value)) in self.0.iter().enumerate() {
            s.push_str(&key.stringify());
            s.push_str(" := ");
            s.push_str(&value.stringify());
            if i + 1 < self.0.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}
