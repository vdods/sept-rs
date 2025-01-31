use crate::{dy, qv, st, Result};

#[derive(Clone, Debug)]
pub struct TupleTermView<'a>(&'a dy::TupleTerm);

impl<'a> TupleTermView<'a> {
    pub fn new(tuple_term: &'a dy::TupleTerm) -> Self {
        Self(tuple_term)
    }
}

impl<'b> qv::QueryT for TupleTermView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalT + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_token_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_token_i.next().unwrap();
        if let Some(element_index) = first_address.downcast_ref::<u32>() {
            // Otherwise index into the tuple.
            anyhow::ensure!(
                *element_index as usize <= self.0.len(),
                "TupleTerm query address index out of bounds: {}",
                element_index
            );
            let element = &self.0[*element_index as usize];
            use qv::QueryableDynT;
            element.make_and_run_query(&mut address_token_i)
        } else {
            // TODO: Handle tuple length, etc.
            use st::StringifiableT;
            anyhow::bail!(
                "TupleTerm query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalT for TupleTermView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0))
    }
}
