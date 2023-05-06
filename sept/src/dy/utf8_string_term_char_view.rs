use crate::{dy, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Utf8StringTermCharView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be QueryViewTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module QueryViewTrait that has a specific type.
    string: &'a str,
    char_index: usize,
    // TODO: Could potentially cache the char
}

impl<'a> Utf8StringTermCharView<'a> {
    pub fn new(string: &'a str, char_index: usize) -> Result<Self> {
        Ok(Self { string, char_index })
    }
}

impl<'b> dy::QueryTrait for Utf8StringTermCharView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_i.next().unwrap();
        anyhow::bail!(
            "Utf8StringTermCharView query doesn't support address: {}",
            dy::RUNTIME_LA.read().unwrap().stringify(first_address)
        );
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermCharView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let c = self
            .string
            .chars()
            .nth(self.char_index as usize)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTerm char index out of bounds"))?;
        log::debug!(
            "Utf8StringTermCharView::dereferenced: self: {:?}, c: {:?}",
            self,
            c
        );
        Ok(dy::MaybeDereferencedValue::ValueLA(Arc::new(RwLock::new(
            dy::Value::from(c).into(),
        ))))
    }
}
