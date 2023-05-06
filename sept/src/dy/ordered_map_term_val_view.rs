use crate::{dy, Result};

// NOTE: This is not necessary either
#[derive(Clone, Debug)]
pub struct OrderedMapTermValView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an OrderedMapTerm view object.
    pub ordered_map_term: &'a dy::OrderedMapTerm,
    pub key: &'a dy::Value,
}

impl<'a> OrderedMapTermValView<'a> {
    pub fn new(ordered_map_term: &'a dy::OrderedMapTerm, key: &'a dy::Value) -> Result<Box<Self>> {
        anyhow::ensure!(
            ordered_map_term.contains_key(key),
            "OrderedMapTerm doesn't contain the specified key"
        );
        Ok(Box::new(Self {
            ordered_map_term,
            key,
        }))
    }
}

impl<'b> dy::QueryTrait for OrderedMapTermValView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            // If we're at the end of the address, then this is the value we're looking for.
            return Ok(self);
        } else {
            // Otherwise, pass it on to the val.  This unwrap can't fail because key containment
            // was checked upon construction.
            let val = self.ordered_map_term.get(self.key).unwrap();
            use dy::QueryableDynTrait;
            val.make_and_run_query(&mut address_i)
        }
    }
}

impl<'b> dy::QueryViewTrait for OrderedMapTermValView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        // This unwrap can't fail because key containment was checked upon construction.
        let val = self.ordered_map_term.get(self.key).unwrap();
        Ok(dy::MaybeDereferencedValue::Ref(val.as_ref()))
    }
}
