use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct Utf8StringTermCharElemMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalTrait<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalTrait that has a specific type.
    pub string: &'a mut String,
    pub char_index: usize,

    // Cached values
    pub char_count: usize,
}

impl<'a> Utf8StringTermCharElemMutView<'a> {
    pub fn new(string: &'a mut String, char_index: usize) -> Result<Self> {
        let char_count = string.chars().count();
        anyhow::ensure!(
            char_index <= char_count,
            "Utf8StringTermCharMutView char_index out of bounds"
        );
        Ok(Self {
            string,
            char_index,
            char_count,
        })
    }
    pub fn new_with_cached_values(
        string: &'a mut String,
        char_index: usize,
        char_count: usize,
    ) -> Result<Self> {
        assert_eq!(
            char_count,
            string.chars().count(),
            "programmer error: given char_count did not match actual char_count"
        );
        anyhow::ensure!(
            char_index <= char_count,
            "Utf8StringTermCharMutView char_index out of bounds"
        );
        Ok(Self {
            string,
            char_index,
            char_count,
        })
    }
}

impl<'a> qv::ApplyEditTrait for Utf8StringTermCharElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch more efficiently (look up table as in Runtime?)
        if edit.type_id() == std::any::TypeId::of::<qv::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<qv::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                insertion_term.new_data.is::<char>(),
                "Utf8StringTermCharMutView expected InsertionTerm edit to have new_data of type char"
            );
            let new_char = insertion_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.char_index <= self.string.len(), "Utf8StringTermCharMutView InsertionTerm edit had out-of-bounds char_index (char_index: {}, string len: {})", self.char_index, self.string.len());
            st::replace_substr_in_string(
                &mut *self.string,
                self.char_index,
                "",
                &format!("{}", new_char),
            )?;
        } else if edit.type_id() == std::any::TypeId::of::<qv::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<qv::DeletionTerm>();
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
        } else if edit.type_id() == std::any::TypeId::of::<qv::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
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

impl<'b> qv::SingleQueryMut<dy::Value> for Utf8StringTermCharElemMutView<'b> {
    type ReturnType<'a> = qv::EmptyQuery where Self: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("Utf8StringTermCharElemMutView doesn't support further queries at this time");
    }
}
