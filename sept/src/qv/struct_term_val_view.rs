use crate::{dy, qv, Result};

#[derive(Clone, Debug)]
pub struct StructTermValView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. an StructTerm view object.
    pub struct_term: &'a dy::StructTerm,
    pub key: &'a String,
}

impl<'a> StructTermValView<'a> {
    pub fn new(struct_term: &'a dy::StructTerm, key: &'a String) -> Result<Self> {
        let _ = struct_term.index_of_named_field(key)?;
        Ok(Self { struct_term, key })
    }
}

impl<'b> qv::QueryTrait for StructTermValView<'b> {
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
            // Otherwise, pass it on to the val.  This unwrap can't fail because key containment
            // was checked upon construction.
            let field_index = self.struct_term.index_of_named_field(self.key).unwrap();
            let val = &self.struct_term.field_decl_v[field_index].1;
            use qv::QueryableDynTrait;
            val.make_and_run_query(&mut address_token_i)
        }
    }
}

impl<'b> qv::EvalTrait for StructTermValView<'b> {
    fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        // This unwrap can't fail because key containment was checked upon construction.
        let field_index = self.struct_term.index_of_named_field(self.key).unwrap();
        let val = &self.struct_term.field_decl_v[field_index].1;
        Ok(dy::MaybeDereferencedValue::make_ref(val.as_ref()))
    }
}
