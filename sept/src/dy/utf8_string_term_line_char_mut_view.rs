use crate::{dy, st, Result};
use std::sync::{Arc, RwLock};

#[derive(Debug)]
pub struct Utf8StringTermLineCharMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be QueryViewTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module QueryViewTrait that has a specific type.
    string: &'a mut String,
    line_index: usize,
    char_index: usize,
    // TODO: Could potentially cache the substring.
}

impl<'a> Utf8StringTermLineCharMutView<'a> {
    pub fn new(string: &'a mut String, line_index: usize, char_index: usize) -> Result<Box<Self>> {
        // TODO: Range checking
        Ok(Box::new(Self {
            string,
            line_index,
            char_index,
        }))
    }
    /// Simultaneously compute the line count and the char indices at the beginning of each line (and the very end).
    fn string_stats(&self) -> (usize, Vec<usize>) {
        let mut line_count = 0usize;
        let mut char_index = 0usize;
        let mut line_char_index_v = Vec::new();
        for line in self.string.split_inclusive('\n') {
            line_count += 1;
            line_char_index_v.push(char_index);
            char_index += line.chars().count();
        }
        line_char_index_v.push(char_index);
        (line_count, line_char_index_v)
    }
}

impl<'b> dy::QueryMutTrait for Utf8StringTermLineCharMutView<'b> {
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
            "Utf8StringTermLineCharView query doesn't support address: {}",
            dy::RUNTIME_LA.read().unwrap().stringify(first_address)
        );
    }
}

impl<'b> dy::Editable for Utf8StringTermLineCharMutView<'b> {
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
        Utf8StringTermLineCharMutView::new(self.string, self.line_index, self.char_index)?
            // .run_query_mut(&mut address_i)?
            .run_query_mut(address_i)?
            .apply_edit(edit)
    }
}

impl<'b> dy::QueryViewTrait for Utf8StringTermLineCharMutView<'b> {
    fn queried_value<'a>(&'a self) -> Result<dy::MaybeDereferencedValue<'a>> {
        let line = self
            .string
            .split_inclusive('\n')
            .nth(self.line_index as usize)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTerm line index out of bounds"))?;
        log::debug!(
            "Utf8StringTermLineCharView::dereferenced: self: {:?}, line: {:?}",
            self,
            line
        );
        let c = line
            .chars()
            .nth(self.char_index)
            .ok_or_else(|| anyhow::anyhow!("Utf8StringTermLineView char index out of bounds"))?;
        Ok(dy::MaybeDereferencedValue::ValueLA(Arc::new(RwLock::new(
            dy::Value::from(c).into(),
        ))))
    }
}

impl<'a> dy::QueryMutViewTrait for Utf8StringTermLineCharMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        let (line_count, line_char_index_v) = self.string_stats();

        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch efficiently.
        if edit.type_id() == std::any::TypeId::of::<dy::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<dy::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                insertion_term.new_data.is::<char>(),
                "Utf8StringTermLineCharMutView expected InsertionTerm edit to have new_data of type UnicodeChar"
            );
            let new_char = insertion_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.line_index <= line_count, "Utf8StringTermLineCharMutView InsertionTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index] + self.char_index,
                "",
                new_char.to_string().as_str(),
            )?;
        } else if edit.type_id() == std::any::TypeId::of::<dy::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<dy::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                deletion_term.old_data.is::<char>(),
                "Utf8StringTermLineCharMutView expected DeletionTerm edit to have old_data of type UnicodeChar"
            );
            let old_char = deletion_term.old_data.downcast_into::<char>();
            anyhow::ensure!(self.line_index < line_count, "Utf8StringTermLineCharMutView DeletionTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index] + self.char_index,
                old_char.to_string().as_str(),
                "",
            )?;
        } else if edit.type_id() == std::any::TypeId::of::<dy::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<dy::ReplacementTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                replacement_term.old_data.is::<char>(),
                "Utf8StringTermLineCharMutView expected ReplacementTerm edit to have old_data of type UnicodeChar"
            );
            anyhow::ensure!(
                replacement_term.new_data.is::<char>(),
                "Utf8StringTermLineCharMutView expected ReplacementTerm edit to have new_data of type UnicodeChar"
            );
            let old_char = replacement_term.old_data.downcast_into::<char>();
            let new_char = replacement_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.line_index < line_count, "Utf8StringTermLineCharMutView ReplacementTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index] + self.char_index,
                old_char.to_string().as_str(),
                new_char.to_string().as_str(),
            )?;
        } else {
            anyhow::bail!(
                "Utf8StringTermLineCharMutView doesn't support edit: {}",
                edit
            );
        }
        Ok(())
    }
}
