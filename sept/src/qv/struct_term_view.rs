use crate::{dy, qv, st, Result};

#[derive(Clone, Debug)]
pub struct StructTermView<'a>(&'a dy::StructTerm);

impl<'a> StructTermView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm) -> Self {
        Self(struct_term)
    }
}

impl<'b> qv::QueryTrait for StructTermView<'b> {
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
        use st::Stringifiable;
        let first_address = address_token_i.next().unwrap();
        if let Some(address_char) = first_address.downcast_ref::<char>().map(|c| *c) {
            match address_char {
                // field names (i.e. keys)
                'k' => {
                    anyhow::ensure!(address_token_i.peek().is_some(), "StructTerm query address 'k' requires a second address (the field name being addressed) but none was provided");
                    let second_address = address_token_i.next().unwrap();
                    anyhow::ensure!(second_address.is::<String>(), "StructTerm query address 'k' requires a second address (field name) of type String but got: {}", second_address.stringify());
                    let field_name = second_address.downcast_ref::<String>().unwrap();
                    // Pass on the rest of the address to the key view's impl of query.
                    Box::new(qv::StructTermKeyView::new(self.0, field_name)?)
                        .run_query(&mut address_token_i)
                }
                // field values
                'v' => {
                    anyhow::ensure!(address_token_i.peek().is_some(), "StructTerm query address 'v' requires a second address (the field name of the value being addressed) but none was provided");
                    let second_address = address_token_i.next().unwrap();
                    anyhow::ensure!(second_address.is::<String>(), "StructTerm query address 'k' requires a second address (field name) of type String but got: {}", second_address.stringify());
                    let field_name = second_address.downcast_ref::<String>().unwrap();
                    // Pass on the rest of the address to the val view's impl of query.
                    Box::new(qv::StructTermValView::new(self.0, field_name)?)
                        .run_query(&mut address_token_i)
                }
                // field name/value pairs (i.e. key/value pairs)
                'p' => {
                    unimplemented!("not yet");
                }
                _ => {
                    anyhow::bail!(
                        "StructTerm query doesn't support address: {}",
                        first_address.stringify()
                    )
                }
            }
        } else {
            anyhow::bail!(
                "StructTerm query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalTrait for StructTermView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0))
    }
}
