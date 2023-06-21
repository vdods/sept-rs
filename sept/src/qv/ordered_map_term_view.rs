use crate::{dy, qv, st, Result};

#[derive(Clone, Debug)]
pub struct OrderedMapTermView<'a>(&'a dy::OrderedMapTerm);

impl<'a> OrderedMapTermView<'a> {
    pub fn new(ordered_map_term: &'a dy::OrderedMapTerm) -> Self {
        Self(ordered_map_term)
    }
}

impl<'b> qv::QueryTrait for OrderedMapTermView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        'b: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_token_i.next().unwrap();
        if let Some(address_char) = first_address.downcast_ref::<char>().map(|c| *c) {
            match address_char {
                // keys
                'k' => {
                    anyhow::ensure!(address_token_i.peek().is_some(), "OrderedMapTerm query address 'k' requires a second address (the key being addressed) but none was provided");
                    let second_address = address_token_i.next().unwrap();
                    // Pass on the rest of the address to the key view's impl of query.
                    Box::new(qv::OrderedMapTermKeyView::new(self.0, second_address)?)
                        .run_query(&mut address_token_i)
                }
                // values
                'v' => {
                    anyhow::ensure!(address_token_i.peek().is_some(), "OrderedMapTerm query address 'v' requires a second address (the key of the value being addressed) but none was provided");
                    let second_address = address_token_i.next().unwrap();
                    // Pass on the rest of the address to the val view's impl of query.
                    Box::new(qv::OrderedMapTermValView::new(self.0, second_address)?)
                        .run_query(&mut address_token_i)
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

impl<'b> qv::EvalTrait for OrderedMapTermView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0))
    }
}
