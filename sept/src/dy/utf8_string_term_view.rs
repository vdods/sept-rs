use crate::{dy, Result};

#[derive(Clone, Debug)]
pub struct Utf8StringTermView<'a>(&'a String);

impl<'a> Utf8StringTermView<'a> {
    pub fn new(string: &'a String) -> Box<Self> {
        Box::new(Self(string))
    }
}

impl<'b> dy::QueryTrait for Utf8StringTermView<'b> {
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
        if let Some(address_string) = first_address.downcast_ref::<String>() {
            match address_string.as_str() {
                "char" => {
                    anyhow::ensure!(address_i.peek().is_some(), "Utf8StringTerm query address 'char' requires a second address (char index) but none was provided");
                    let second_address = address_i.next().unwrap();
                    anyhow::ensure!(second_address.is::<u32>(), "Utf8StringTerm query address 'char' requires a second address (char index) of type u32 but got: {}", dy::RUNTIME_LA.read().unwrap().stringify(second_address));
                    let char_index = *second_address.downcast_ref::<u32>().unwrap();
                    let utf8_string_char_view =
                        dy::Utf8StringTermCharView::<'a>::new(self.0, char_index as usize)?;
                    // Pass on the rest of the address to the char view's impl of query.
                    Box::new(utf8_string_char_view).run_query(&mut address_i)
                }
                "line" => {
                    anyhow::ensure!(address_i.peek().is_some(), "Utf8StringTerm query address 'line' requires a second address (line index) but none was provided");
                    let second_address = address_i.next().unwrap();
                    anyhow::ensure!(second_address.is::<u32>(), "Utf8StringTerm query address 'line' requires a second address (line index) of type u32 but got: {}", dy::RUNTIME_LA.read().unwrap().stringify(second_address));
                    let line_index = *second_address.downcast_ref::<u32>().unwrap();
                    let utf8_string_line_view =
                        dy::Utf8StringTermLineView::new(self.0, line_index as usize)?;
                    // Pass on the rest of the address to the line view's impl of query.
                    Box::new(utf8_string_line_view).run_query(&mut address_i)
                }
                _ => anyhow::bail!(
                    "Utf8StringTerm query doesn't support address: {}",
                    dy::RUNTIME_LA.read().unwrap().stringify(first_address)
                ),
            }
        } else {
            anyhow::bail!(
                "Utf8StringTerm query doesn't support address: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(first_address)
            );
        }
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}
