use crate::{dy, Result};

#[derive(Clone, Debug)]
pub struct StructTermKeyView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an StructTerm view object.
    pub struct_term: &'a dy::StructTerm,
    pub key: &'a String,
}

impl<'a> StructTermKeyView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm, key: &'a String) -> Result<Box<Self>> {
        let _ = struct_term.index_of_named_field(key)?;
        Ok(Box::new(Self { struct_term, key }))
    }
}

impl<'b> dy::QueryTrait for StructTermKeyView<'b> {
    fn run_query<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        if address_i.peek().is_none() {
            // If we're at the end of the address, then this is the value we're looking for.
            return Ok(self);
        } else {
            // Otherwise, pass it on to the key.
            use dy::QueryableDynTrait;
            self.key.make_and_run_query(&mut address_i)
        }
    }
}

impl<'b> dy::QueryViewTrait for StructTermKeyView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.key))
    }
}
