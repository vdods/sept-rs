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
    is_parametric = "true",
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
        write!(f, "{}", &self.stringify())
    }
}

impl dy::Editable for OrderedMapTerm {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        use dy::QueryMutTrait;
        dy::OrderedMapTermMutView::new(self)
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl Inhabits<OrderedMap> for OrderedMapTerm {
    fn inhabits(&self, _: &OrderedMap) -> bool {
        true
    }
}

impl dy::Queryable for OrderedMapTerm {
    fn query<'a>(&'a self, address_v: &[dy::Value]) -> Result<&'a dy::ValueGuts> {
        if address_v.is_empty() {
            Ok(self)
        } else {
            // Eat the first address token, interpreting it as the key.
            // TODO: Support other queries here, such as `Len` (though this would require returning
            // something like MaybeDereferencedValue since it wouldn't be an l-value (in the C++ sense, i.e.
            // a value without a memory address))
            let key = &address_v[0];
            let value = self
                .get(key)
                .ok_or_else(|| anyhow::anyhow!("OrderedMapTerm::query key not found"))?
                .as_ref();
            // Recurse with the remainder of the address.
            dy::RUNTIME_LA.read().unwrap().query(value, &address_v[1..])
        }
    }
    fn query_mut<'a>(&'a mut self, _address_v: &[dy::Value]) -> Result<&'a mut dy::ValueGuts> {
        unimplemented!("blah");
        // TODO: This should basically be the same as query, though maybe non-l-values (e.g. querying
        // `Len`) wouldn't support this.
    }
}

impl dy::QueryableDynTrait for OrderedMapTerm {
    fn make_query<'a>(&'a self) -> Box<dyn dy::QueryTrait + 'a> {
        dy::OrderedMapTermView::new(self)
    }
}

impl dy::QueryableMutDynTrait for OrderedMapTerm {
    fn make_query_mut<'a>(&'a mut self) -> Box<dyn dy::QueryMutTrait + 'a> {
        dy::OrderedMapTermMutView::new(self)
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
