use crate::{dy, qv, Result};

#[derive(Clone, Debug)]
pub struct StructTermKeyView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an StructTerm view object.
    pub struct_term: &'a dy::StructTerm,
    pub key: &'a String,
}

impl<'a> StructTermKeyView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm, key: &'a String) -> Result<Self> {
        let _ = struct_term.index_of_named_field(key)?;
        Ok(Self { struct_term, key })
    }
}

impl<'b> qv::QueryTrait for StructTermKeyView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn qv::EvalTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            // If we're at the end of the address, then this is the value we're looking for.
            return Ok(self);
        } else {
            // Otherwise, pass it on to the key.
            use qv::QueryableDynTrait;
            self.key.make_and_run_query(&mut address_token_i)
        }
    }
}

impl<'b> qv::EvalTrait for StructTermKeyView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::make_ref(self.key))
    }
}
