use crate::{
    dy, qv,
    st::{self, Inhabits, OrderedMap},
    Error, Result,
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
    is_parametric = "true",
    is_type = "true"
)]
pub struct OrderedMapTerm(BTreeMap<dy::Value, dy::Value>);

impl qv::ApplyEditTrait for OrderedMapTerm {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl dy::Deconstruct for OrderedMapTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        // Turn the BTreeMap elements (i.e. key-value pairs) into TupleTerms, and then collect that into
        // a TupleTerm.  Could create MappingTerm, Mapping, and MappingType terms for better semantics.
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

impl st::Deserializable for OrderedMapTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let len = st::read_len(reader)?;
        let mut element_m = BTreeMap::new();
        for _ in 0..len {
            let key = dy::Value::deserialize(reader)?;
            let value = dy::Value::deserialize(reader)?;
            element_m.insert(key, value);
        }
        Ok(Self(element_m))
    }
}

impl std::fmt::Display for OrderedMapTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use st::Stringifiable;
        write!(f, "{}", &self.stringify())
    }
}

impl Inhabits<OrderedMap> for OrderedMapTerm {
    fn inhabits(&self, _: &OrderedMap) -> bool {
        true
    }
}

impl qv::QueryableDynTrait for OrderedMapTerm {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryTrait + 'a> {
        Box::new(qv::OrderedMapTermView::new(self))
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

impl qv::SingleQueryMut<dy::Value> for OrderedMapTerm {
    type ReturnType<'a> = qv::OrderedMapTermQueryMut<'a>;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(address_char) = address_token.downcast_ref::<char>().map(|c| *c) {
            match address_char {
                // keys
                'k' => Ok(qv::OrderedMapTermKeyMutView::new(self).into()),
                // values
                'v' => Ok(qv::OrderedMapTermValMutView::new(self).into()),
                // key/value pairs
                'p' => {
                    unimplemented!("not yet");
                }
                _ => {
                    use st::Stringifiable;
                    anyhow::bail!(
                        "OrderedMapTerm query doesn't support address: {}",
                        address_char.stringify()
                    )
                }
            }
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "OrderedMapTerm query doesn't support address: {}",
                address_token.stringify()
            );
        }
    }
}

impl st::Stringifiable for OrderedMapTerm {
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

impl st::TestValues for OrderedMapTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec![
            OrderedMapTerm::from(maplit::btreemap! {}),
            OrderedMapTerm::from(maplit::btreemap! { 4u32.into() => "blah".to_string().into() }),
            OrderedMapTerm::from(
                maplit::btreemap! { 4u32.into() => "blah".to_string().into(), 5u32.into() => "FWEEE".to_string().into() },
            ),
            OrderedMapTerm::from(
                maplit::btreemap! { 4u32.into() => "blah".to_string().into(), 5u32.into() => "FWEEE".to_string().into(), true.into() => false.into() },
            ),
            // Some nested ones for good measure
            OrderedMapTerm::from(maplit::btreemap! {
                st::Void.into() => OrderedMapTerm::from(maplit::btreemap! { 4u32.into() => "blah".to_string().into() }).into(),
                st::EmptyType.into() => OrderedMapTerm::from(maplit::btreemap! { 4u32.into() => "blah".to_string().into(), 5u32.into() => "FWEEE".to_string().into() }).into(),
            }),
        ]
    }
}
