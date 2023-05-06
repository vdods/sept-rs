use crate::{dy, st, Result};

#[derive(Clone, Debug)]
pub struct OrderedMapTermView<'a>(&'a dy::OrderedMapTerm);

impl<'a> OrderedMapTermView<'a> {
    pub fn new(ordered_map_term: &'a dy::OrderedMapTerm) -> Box<Self> {
        Box::new(Self(ordered_map_term))
    }
}

impl<'b> dy::QueryTrait for OrderedMapTermView<'b> {
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
        if let Some(address_char) = first_address.downcast_ref::<char>().map(|c| *c) {
            match address_char {
                // keys
                'k' => {
                    anyhow::ensure!(address_i.peek().is_some(), "OrderedMapTerm query address 'k' requires a second address (the key being addressed) but none was provided");
                    let second_address = address_i.next().unwrap();
                    // Pass on the rest of the address to the key view's impl of query.
                    dy::OrderedMapTermKeyView::new(self.0, second_address)?
                        .run_query(&mut address_i)
                }
                // values
                'v' => {
                    anyhow::ensure!(address_i.peek().is_some(), "OrderedMapTerm query address 'v' requires a second address (the key of the value being addressed) but none was provided");
                    let second_address = address_i.next().unwrap();
                    // Pass on the rest of the address to the val view's impl of query.
                    dy::OrderedMapTermValView::new(self.0, second_address)?
                        .run_query(&mut address_i)
                }
                // key/value pairs
                'p' => {
                    unimplemented!("not yet");
                }
                _ => {
                    use st::Stringifiable;
                    anyhow::bail!(
                        "OrderedMapTerm query doesn't support address: {}",
                        first_address.stringify()
                    )
                }
            }
        } else {
            use st::Stringifiable;
            anyhow::bail!(
                "OrderedMapTerm query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> dy::QueryViewTrait for OrderedMapTermView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}
