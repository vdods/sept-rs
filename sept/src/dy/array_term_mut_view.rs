use crate::{dy, Result};

#[derive(Debug)]
pub struct ArrayTermMutView<'a>(&'a mut dy::ArrayTerm);

impl<'a> ArrayTermMutView<'a> {
    pub fn new(array_term: &'a mut dy::ArrayTerm) -> Box<Self> {
        Box::new(Self(array_term))
    }
}

impl<'b> dy::QueryMutTrait for ArrayTermMutView<'b> {
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
                "ArrayTerm query address index out of bounds: {}",
                element_index
            );
            let element = self.0.get_mut(*element_index as usize).unwrap();
            dy::ValueMutView::new(element).run_query_mut(&mut address_i)
        } else {
            // TODO: Handle array length, etc.
            anyhow::bail!(
                "ArrayTerm query doesn't support address: {}",
                dy::RUNTIME_LA.read().unwrap().stringify(first_address)
            );
        }
    }
}

impl<'b> dy::QueryViewTrait for ArrayTermMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        Ok(dy::MaybeDereferencedValue::Ref(self.0))
    }
}

impl<'a> dy::QueryMutViewTrait for ArrayTermMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: "clear" edit
        if edit.is::<dy::ReplacementTerm>() {
            let edit = edit.downcast_into::<dy::ReplacementTerm>();
            anyhow::ensure!(
                edit.old_data.is::<dy::ArrayTerm>(),
                "ArrayTerm ReplacementTerm edit expected old_data to be ArrayTerm"
            );
            anyhow::ensure!(
                edit.new_data.is::<dy::ArrayTerm>(),
                "ArrayTerm ReplacementTerm edit expected new_data to be ArrayTerm"
            );
            let old_array_term = edit.old_data.downcast_into::<dy::ArrayTerm>();
            let new_array_term = edit.new_data.downcast_into::<dy::ArrayTerm>();
            anyhow::ensure!(*self.0 == old_array_term, "ArrayTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self.0, old_array_term);
            *self.0 = new_array_term;
        } else {
            anyhow::bail!("ArrayTerm only supports ReplacementTerm edits")
        }
        Ok(())
    }
}
