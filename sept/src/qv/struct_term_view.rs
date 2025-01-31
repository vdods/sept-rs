use crate::{dy, qv, st, Result};

#[derive(Clone, Debug)]
pub struct StructTermView<'a>(&'a dy::StructTerm);

impl<'a> StructTermView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm) -> Self {
        Self(struct_term)
    }
}

impl<'b> qv::QueryT for StructTermView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalT + 'a>>
    where
        'b: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        use st::StringifiableT;
        let first_address = address_token_i.next().unwrap();
        if let Some(field_index) = first_address.downcast_ref::<u32>() {
            Box::new(qv::StructTermFieldElemView::new(
                self.0,
                *field_index as usize,
            )?)
            .run_query(&mut address_token_i)
        // } else if let Some(address_char) = first_address.downcast_ref::<char>().map(|c| *c) {
        //     match address_char {
        //         // field names (i.e. keys)
        //         'k' => Box::new(qv::StructTermFieldNameView::new(self.0))
        //             .run_query(&mut address_token_i),
        //         // field types
        //         'v' => Box::new(qv::StructTermFieldTypeView::new(self.0))
        //             .run_query(&mut address_token_i),
        //         // field name/value pairs (i.e. key/value pairs)
        //         'p' => {
        //             unimplemented!("not yet");
        //         }
        //         _ => {
        //             anyhow::bail!(
        //                 "StructTerm query doesn't support address: {}",
        //                 first_address.stringify()
        //             )
        //         }
        //     }
        } else {
            anyhow::bail!(
                "StructTerm query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalT for StructTermView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0))
    }
}
