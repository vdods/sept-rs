use crate::{dy, qv, Result};

#[derive(Debug)]
pub struct TupleTermElemMutView<'a> {
    tuple_term: &'a mut dy::TupleTerm,
    elem_index: usize,
}

impl<'a> TupleTermElemMutView<'a> {
    pub fn new(tuple_term: &'a mut dy::TupleTerm, elem_index: usize) -> Result<Self> {
        anyhow::ensure!(
            elem_index <= tuple_term.len(),
            "TupleTermElemMutView elem_index out of bounds"
        );
        Ok(Self {
            tuple_term,
            elem_index,
        })
    }
}

impl<'a> qv::ApplyEditT for TupleTermElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch more efficiently (look up table as in Runtime?)
        if edit.type_id() == std::any::TypeId::of::<qv::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<qv::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.elem_index <= self.tuple_term.len(), "TupleTermElemMutView InsertionTerm edit had out-of-bounds elem_index (elem_index: {}, tuple_term len: {})", self.elem_index, self.tuple_term.len());
            self.tuple_term
                .insert(self.elem_index, insertion_term.new_data);
        } else if edit.type_id() == std::any::TypeId::of::<qv::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<qv::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.elem_index < self.tuple_term.len(), "TupleTermElemMutView DeletionTerm edit had out-of-bounds elem_index (elem_index: {}, tuple_term len: {})", self.elem_index, self.tuple_term.len());
            anyhow::ensure!(
                *self.tuple_term.get(self.elem_index).unwrap() == deletion_term.old_data,
                "TupleTermElemMutView DeletionTerm old_data did not match existing value"
            );
            self.tuple_term.remove(self.elem_index);
        } else if edit.type_id() == std::any::TypeId::of::<qv::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(self.elem_index < self.tuple_term.len(), "TupleTermElemMutView ReplacementTerm edit had out-of-bounds elem_index (elem_index: {}, tuple_term len: {})", self.elem_index, self.tuple_term.len());
            anyhow::ensure!(
                *self.tuple_term.get(self.elem_index).unwrap() == replacement_term.old_data,
                "TupleTermElemMutView ReplacementTerm old_data did not match existing value"
            );
            *self.tuple_term.get_mut(self.elem_index).unwrap() = replacement_term.new_data;
        } else {
            anyhow::bail!("TupleTermElemMutView doesn't support edit: {}", edit);
        }
        Ok(())
    }
}

// impl<'b> qv::QueryMutT for TupleTermElemMutView<'b> {
//     fn run_query_mut<'a>(
//         self: Box<Self>,
//         address_token_i: &mut dyn std::iter::Iterator<Item = &'a dy::Value>,
//     ) -> Result<Box<dyn qv::ApplyEditT + 'a>>
//     where
//         Self: 'a,
//     {
//         let mut address_token_i = address_token_i.peekable();
//         // If we're at the end of the address, then this is the value we're looking for.
//         if address_token_i.peek().is_none() {
//             return Ok(self);
//         }
//         let first_address = address_token_i.next().unwrap();
//         anyhow::bail!(
//             "TupleTermElemMutView query doesn't support address: {}",
//             dy::RUNTIME_LA.read().unwrap().stringify(first_address)
//         );
//     }
// }

impl<'b> qv::QueryMutAndApplyEditT for TupleTermElemMutView<'b> {
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
            use qv::ApplyEditT;
            self.apply_edit(edit)
        } else {
            anyhow::ensure!(
                self.elem_index < self.tuple_term.len(),
                "TupleTermElemMutView elem_index out of bounds"
            );
            // Forward the call to the element, which is a Value.
            self.tuple_term
                .get_mut(self.elem_index)
                .unwrap()
                .query_mut_and_apply_edit(&mut address_token_i, edit)
        }
    }
}

// impl<'b> qv::EvalT for TupleTermElemMutView<'b> {
//     fn eval<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
//         if let Some(elem) = self.tuple_term.get(self.elem_index) {
//             Ok(dy::MaybeDereferencedValue::make_ref(elem))
//         } else {
//             Ok(dy::MaybeDereferencedValue::make_value_la(Arc::new(RwLock::new(
//                 st::Void.into(),
//             ))))
//         }
//     }
// }
