use crate::{dy, qv, st, Result};

#[derive(Clone, Debug)]
pub struct UTF8StringTermView<'a>(&'a String);

impl<'a> UTF8StringTermView<'a> {
    pub fn new(string: &'a String) -> Self {
        Self(string)
    }
}

impl<'b> qv::QueryT for UTF8StringTermView<'b> {
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
        use st::StringifiableT;
        if let Some(address_string) = first_address.downcast_ref::<String>() {
            match address_string.as_str() {
                "char" => Box::new(qv::UTF8StringTermCharView::new(self.0))
                    .run_query(&mut address_token_i),
                "line" => Box::new(qv::UTF8StringTermLineView::new(self.0))
                    .run_query(&mut address_token_i),
                _ => anyhow::bail!(
                    "UTF8StringTerm query doesn't support address: {}",
                    first_address.stringify()
                ),
            }
        } else {
            anyhow::bail!(
                "UTF8StringTerm query doesn't support address: {}",
                first_address.stringify()
            );
        }
    }
}

impl<'b> qv::EvalT for UTF8StringTermView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.0))
    }
}
