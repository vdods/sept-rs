use crate::{dy, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone, Debug)]
pub struct Utf8StringTermLineView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be QueryViewTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module QueryViewTrait that has a specific type.
    string: &'a str,
    line_index: usize,
    // TODO: Could potentially cache the substring.
}

impl<'a> Utf8StringTermLineView<'a> {
    pub fn new(string: &'a str, line_index: usize) -> Result<Self> {
        Ok(Self { string, line_index })
    }
}

impl<'b> dy::QueryTrait for Utf8StringTermLineView<'b> {
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
        match first_address.downcast_ref::<String>().map(String::as_str) {
            Some("char") => {
                anyhow::ensure!(address_i.peek().is_some(), "Utf8StringTermLineView query address 'char' requires a second address (char index) but none was provided");
                let second_address = address_i.next().unwrap();
                anyhow::ensure!(second_address.is::<u32>(), "Utf8StringTermLineView query address 'char' requires a second address (char index) of type u32 but got: {}", dy::RUNTIME_LA.read().unwrap().stringify(second_address));
                let char_index = *second_address.downcast_ref::<u32>().unwrap();
                let utf8_string_line_char_view = dy::Utf8StringTermLineCharView::new(
                    self.string,
                    self.line_index,
                    char_index as usize,
                )?;
                // Pass on the rest of the address to the char view's impl of query_mut.
                Box::new(utf8_string_line_char_view).run_query(&mut address_i)
            }
            _ => anyhow::bail!(
                "Utf8StringTerm query doesn't support address: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(first_address)
            ),
        }
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermLineView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let line = self
            .string
            .split_inclusive('\n')
            .nth(self.line_index as usize)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTerm line index out of bounds"))?;
        log::debug!(
            "Utf8StringTermLineView::dereferenced: self: {:?}, line: {:?}",
            self,
            line
        );
        Ok(dy::MaybeDereferencedValue::ValueLA(Arc::new(RwLock::new(
            dy::Value::from(line.to_string()).into(),
        ))))
    }
}
