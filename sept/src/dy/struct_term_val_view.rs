use crate::{dy, Result};

#[derive(Clone, Debug)]
pub struct StructTermValView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an StructTerm view object.
    pub struct_term: &'a dy::StructTerm,
    pub key: &'a String,
}

impl<'a> StructTermValView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm, key: &'a String) -> Result<Box<Self>> {
        let _ = struct_term.index_of_named_field(key)?;
        Ok(Box::new(Self { struct_term, key }))
    }
}

impl<'b> dy::QueryTrait for StructTermValView<'b> {
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
            // Otherwise, pass it on to the val.  This unwrap can't fail because key containment
            // was checked upon construction.
            let field_index = self.struct_term.index_of_named_field(self.key).unwrap();
            let val = &self.struct_term.field_decl_v[field_index].1;
            use dy::QueryableDynTrait;
            val.make_and_run_query(&mut address_i)
        }
    }
}

impl<'b> dy::QueryViewTrait for StructTermValView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        // This unwrap can't fail because key containment was checked upon construction.
        let field_index = self.struct_term.index_of_named_field(self.key).unwrap();
        let val = &self.struct_term.field_decl_v[field_index].1;
        Ok(dy::MaybeDereferencedValue::Ref(val.as_ref()))
    }
}
