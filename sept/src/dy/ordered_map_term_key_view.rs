use crate::{dy, Result};

#[derive(Clone, Debug)]
pub struct OrderedMapTermKeyView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an OrderedMapTerm view object.
    pub ordered_map_term: &'a dy::OrderedMapTerm,
    pub key: &'a dy::Value,
}

impl<'a> OrderedMapTermKeyView<'a> {
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

// impl<'b> dy::QueryTrait<'b> for OrderedMapTermKeyView<'b> {
impl<'b> dy::QueryTrait for OrderedMapTermKeyView<'b> {
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
            // Otherwise, pass it on to the key.
            use dy::QueryableDynTrait;
            self.key.make_and_run_query(&mut address_i)
        }
    }
}

impl<'b> dy::QueryViewTrait for OrderedMapTermKeyView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.key.as_ref()))
    }
}
