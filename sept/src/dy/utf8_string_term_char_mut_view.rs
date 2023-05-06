use crate::{dy, st, Result};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Utf8StringTermCharMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be QueryViewTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module QueryViewTrait that has a specific type.
    string: &'a mut String,
    char_index: usize,
    // TODO: Could potentially cache the char.
}

impl<'a> Utf8StringTermCharMutView<'a> {
    pub fn new(string: &'a mut String, char_index: usize) -> Result<Box<Self>> {
        anyhow::ensure!(
            char_index <= string.chars().count(),
            "Utf8StringTermCharMutView char_index out of bounds"
        );
        Ok(Box::new(Self { string, char_index }))
    }
}

impl<'b> dy::QueryMutTrait for Utf8StringTermCharMutView<'b> {
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
        anyhow::bail!(
            "Utf8StringTermCharView query doesn't support address: {}",
            dy::RUNTIME_LA.read().unwrap().stringify(first_address)
        );
    }
}

impl<'b> dy::Editable for Utf8StringTermCharMutView<'b> {
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
        Utf8StringTermCharMutView::new(self.string, self.char_index)?
            // .run_query_mut(&mut address_i)?
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermCharMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let c = self
            .string
            .chars()
            .nth(self.char_index as usize)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTerm char index out of bounds"))?;
        log::debug!(
            "Utf8StringTermCharView::dereferenced: self: {:?}, c: {:?}",
            self,
            c
        );
        Ok(dy::MaybeDereferencedValue::ValueLA(Arc::new(RwLock::new(
            dy::Value::from(c).into(),
        ))))
    }
}

impl<'a> dy::QueryMutViewTrait for Utf8StringTermCharMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch more efficiently (look up table as in Runtime?)
        if edit.type_id() == std::any::TypeId::of::<dy::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<dy::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                insertion_term.new_data.is::<char>(),
                "Utf8StringTermCharMutView expected InsertionTerm edit to have new_data of type char"
            );
            let new_char = insertion_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.char_index <= self.string.len(), "Utf8StringTermCharMutView InsertionTerm edit had out-of-bounds char_index (char_index: {}, string len: {})", self.char_index, self.string.len());
            st::replace_single_char_in_string(
                &mut *self.string,
                self.char_index,
                None,
                &format!("{}", new_char),
            )?;
        } else if edit.type_id() == std::any::TypeId::of::<dy::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<dy::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                deletion_term.old_data.is::<char>(),
                "Utf8StringTermCharMutView expected DeletionTerm edit to have old_data of type char"
            );
            let old_char = deletion_term.old_data.downcast_into::<char>();
            anyhow::ensure!(self.char_index < self.string.len(), "Utf8StringTermCharMutView DeletionTerm edit had out-of-bounds char_index (char_index: {}, string len: {})", self.char_index, self.string.len());
            st::replace_single_char_in_string(
                &mut *self.string,
                self.char_index,
                Some(old_char),
                "",
            )?;
        } else if edit.type_id() == std::any::TypeId::of::<dy::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<dy::ReplacementTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                replacement_term.old_data.is::<char>(),
                "Utf8StringTermCharMutView expected ReplacementTerm edit to have old_data of type char"
            );
            let old_char = replacement_term.old_data.downcast_into::<char>();
            anyhow::ensure!(
                replacement_term.new_data.is::<char>(),
                "Utf8StringTermCharMutView expected ReplacementTerm edit to have new_data of type char"
            );
            let new_char = replacement_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.char_index < self.string.len(), "Utf8StringTermCharMutView ReplacementTerm edit had out-of-bounds char_index (char_index: {}, string len: {})", self.char_index, self.string.len());
            st::replace_single_char_in_string(
                &mut *self.string,
                self.char_index,
                Some(old_char),
                &format!("{}", new_char),
            )?;
        } else {
            anyhow::bail!("Utf8StringTermCharMutView doesn't support edit: {}", edit);
        }
        Ok(())
    }
}
