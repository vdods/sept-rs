use crate::{dy, Result};

#[derive(Debug)]
pub struct TupleTermMutView<'a>(&'a mut dy::TupleTerm);

impl<'a> TupleTermMutView<'a> {
    pub fn new(tuple_term: &'a mut dy::TupleTerm) -> Box<Self> {
        Box::new(Self(tuple_term))
    }
}

impl<'b> dy::QueryMutTrait for TupleTermMutView<'b> {
    fn run_query_mut<'a>(
        self: Box<Self>,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
    ) -> Result<Box<dyn dy::QueryMutViewTrait + 'a>>
    where
        Self: 'a,
    {
        let mut address_i = address_i.peekable();
        // If we're at the end of the address, then this is the value we're looking for.
        if address_i.peek().is_none() {
            return Ok(self);
        }
        let first_address = address_i.next().unwrap();
        if let Some(element_index) = first_address.downcast_ref::<u32>() {
            // Otherwise index into the array.
            anyhow::ensure!(
                *element_index as usize <= self.0.len(),
                "TupleTerm query address index out of bounds: {}",
                element_index
            );
            let element = self.0.get_mut(*element_index as usize).unwrap();
            use dy::QueryableMutDynTrait;
            element.make_and_run_query_mut(&mut address_i)
        } else {
            // TODO: Handle array length, etc.
            anyhow::bail!(
                "TupleTerm query doesn't support address: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(first_address)
            );
        }
    }
}

impl<'b> dy::Editable for TupleTermMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        // TEMP HACK -- this is rather silly, but is a quick way to get the right behavior for now.
        use dy::QueryMutTrait;
        // Re-borrow the address iterator items with a shorter lifetime.
        // let mut address_i = address_i.map(|x| &*x);
        // Note that this can't be Self, because this introduces a new, shorter lifetime.
        TupleTermMutView::new(self.0)
            // .run_query_mut(&mut address_i)?
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl<'b> dy::QueryViewTrait for TupleTermMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}

impl<'a> dy::QueryMutViewTrait for TupleTermMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: "clear" edit
        if edit.is::<dy::ReplacementTerm>() {
            let edit = edit.downcast_into::<dy::ReplacementTerm>();
            anyhow::ensure!(
                edit.old_data.is::<dy::TupleTerm>(),
                "TupleTerm ReplacementTerm edit expected old_data to be TupleTerm"
            );
            anyhow::ensure!(
                edit.new_data.is::<dy::TupleTerm>(),
                "TupleTerm ReplacementTerm edit expected new_data to be TupleTerm"
            );
            let old_tuple_term = edit.old_data.downcast_into::<dy::TupleTerm>();
            let new_tuple_term = edit.new_data.downcast_into::<dy::TupleTerm>();
            anyhow::ensure!(*self.0 == old_tuple_term, "TupleTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, old_tuple_term);
            *self.0 = new_tuple_term;
        } else {
            anyhow::bail!("TupleTerm only supports ReplacementTerm edits")
        }
        Ok(())
    }
}
