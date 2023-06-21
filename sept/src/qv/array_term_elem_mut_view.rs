use crate::{dy, qv, Result};

#[derive(Debug)]
pub struct ArrayTermElemMutView<'a> {
    array_term: &'a mut dy::ArrayTerm,
    elem_index: usize,
}

impl<'a> ArrayTermElemMutView<'a> {
    pub fn new(array_term: &'a mut dy::ArrayTerm, elem_index: usize) -> Result<Self> {
        anyhow::ensure!(
            elem_index <= array_term.len(),
            "ArrayTermElemMutView elem_index out of bounds"
        );
        Ok(Self {
            array_term,
            elem_index,
        })
    }
}

impl<'a> qv::ApplyEditTrait for ArrayTermElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch more efficiently (look up table as in Runtime?)
        if edit.type_id() == std::any::TypeId::of::<qv::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<qv::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.elem_index <= self.array_term.len(), "ArrayTermElemMutView InsertionTerm edit had out-of-bounds elem_index (elem_index: {}, array_term len: {})", self.elem_index, self.array_term.len());
            self.array_term
                .insert(self.elem_index, insertion_term.new_data);
        } else if edit.type_id() == std::any::TypeId::of::<qv::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<qv::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.elem_index < self.array_term.len(), "ArrayTermElemMutView DeletionTerm edit had out-of-bounds elem_index (elem_index: {}, array_term len: {})", self.elem_index, self.array_term.len());
            anyhow::ensure!(
                *self.array_term.get(self.elem_index).unwrap() == deletion_term.old_data,
                "ArrayTermElemMutView DeletionTerm old_data did not match existing value"
            );
            self.array_term.remove(self.elem_index);
        } else if edit.type_id() == std::any::TypeId::of::<qv::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.elem_index < self.array_term.len(), "ArrayTermElemMutView ReplacementTerm edit had out-of-bounds elem_index (elem_index: {}, array_term len: {})", self.elem_index, self.array_term.len());
            anyhow::ensure!(
                *self.array_term.get(self.elem_index).unwrap() == replacement_term.old_data,
                "ArrayTermElemMutView ReplacementTerm old_data did not match existing value"
            );
            *self.array_term.get_mut(self.elem_index).unwrap() = replacement_term.new_data;
        } else {
            anyhow::bail!("ArrayTermElemMutView doesn't support edit: {}", edit);
        }
        Ok(())
    }
}

impl<'b> qv::QueryMutAndApplyEditTrait for ArrayTermElemMutView<'b> {
    fn query_mut_and_apply_edit<'s, 'a>(
        &'s mut self,
        address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
        edit: dy::Value,
    ) -> Result<()>
    where
        's: 'a,
    {
        let mut address_token_i = address_token_i.peekable();
        if address_token_i.peek().is_none() {
            use qv::ApplyEditTrait;
            self.apply_edit(edit)
        } else {
            anyhow::ensure!(
                self.elem_index < self.array_term.len(),
                "ArrayTermElemMutView elem_index out of bounds"
            );
            // Forward the call to the element, which is a Value.
            self.array_term
                .get_mut(self.elem_index)
                .unwrap()
                .query_mut_and_apply_edit(&mut address_token_i, edit)
        }
    }
}
