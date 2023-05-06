use crate::{dy, Result};
use std::sync::{Arc, RwLock};

// TODO: This should just be a char view that takes a Utf8StringTermLineView.
#[derive(Clone, Debug)]
pub struct Utf8StringTermLineCharView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be QueryViewTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module QueryViewTrait that has a specific type.
    string: &'a str,
    line_index: usize,
    char_index: usize,
    // TODO: Could potentially cache the substring.
}

impl<'a> Utf8StringTermLineCharView<'a> {
    pub fn new(string: &'a str, line_index: usize, char_index: usize) -> Result<Self> {
        Ok(Self {
            string,
            line_index,
            char_index,
        })
    }
}

impl<'b> dy::QueryTrait for Utf8StringTermLineCharView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        'b: 'a,
    {
        let mut address_i = address_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_i.next().unwrap();
        anyhow::bail!(
            "Utf8StringTerm query doesn't support address: {}",
            dy::RUNTIME_LA.read().unwrap().stringify(first_address)
        );
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermLineCharView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let line = self
            .string
            .split_inclusive('\n')
            .nth(self.line_index)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTerm line index out of bounds"))?;
        log::debug!(
            "Utf8StringTermLineCharView::dereferenced: self: {:?}, line: {:?}",
            self,
            line
        );
        let c = line
            .chars()
            .nth(self.char_index)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTermLineView char index out of bounds"))?;
        Ok(dy::MaybeDereferencedValue::ValueLA(Arc::new(RwLock::new(
            dy::Value::from(c).into(),
        ))))
    }
}
