use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct UTF8StringTermLineElemCharElemMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    string: &'a mut String,
    line_index: usize,
    char_index: usize,

    // Cached values
    #[allow(unused)]
    line_count: usize,
    #[allow(unused)]
    line_char_count: usize,
}

impl<'a> UTF8StringTermLineElemCharElemMutView<'a> {
    pub fn new(string: &'a mut String, line_index: usize, char_index: usize) -> Result<Self> {
        let line_count = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n').count();
        anyhow::ensure!(
            line_index < line_count,
            "UTF8StringTermLineElemCharElemMutView line_index out of bounds"
        );
        let line_char_count = {
            let line = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n')
                .nth(line_index)
                .unwrap();
            line.chars().count()
        };
        anyhow::ensure!(char_index <= line_char_count, "UTF8StringTermLineElemCharElemMutView char_index ({}) out of bounds, max allowable char_index value is {}", char_index, line_char_count);
        Ok(Self {
            string,
            line_index,
            char_index,
            line_count,
            line_char_count,
        })
    }
    pub fn new_with_cached_values(
        string: &'a mut String,
        line_index: usize,
        char_index: usize,
        line_count: usize,
        line_char_count: usize,
    ) -> Result<Self> {
        assert_eq!(
            line_count,
            st::split_inclusive_allow_trailing_empty(string.as_str(), '\n').count(),
            "programmer error: given line_count did not match actual line_count"
        );
        anyhow::ensure!(
            line_index < line_count,
            "UTF8StringTermLineElemCharElemMutView line_index out of bounds"
        );
        {
            let line = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n')
                .nth(line_index)
                .unwrap();
            assert_eq!(
                line_char_count,
                line.chars().count(),
                "programmer error: given line_char_count did not match actual line_char_count"
            );
        }
        anyhow::ensure!(char_index <= line_char_count, "UTF8StringTermLineElemCharElemMutView char_index ({}) out of bounds, max allowable char_index value is {}", char_index, line_char_count);
        Ok(Self {
            string,
            line_index,
            char_index,
            line_count,
            line_char_count,
        })
    }
    /// Simultaneously compute the line count and the char indices at the beginning of each line (and the very end).
    fn string_stats(&self) -> (usize, Vec<usize>) {
        let mut line_count = 0usize;
        let mut char_index = 0usize;
        let mut line_char_index_v = Vec::new();
        for line in st::split_inclusive_allow_trailing_empty(self.string.as_str(), '\n') {
            line_count += 1;
            line_char_index_v.push(char_index);
            char_index += line.chars().count();
        }
        line_char_index_v.push(char_index);
        (line_count, line_char_index_v)
    }
}

impl<'a> qv::ApplyEditT for UTF8StringTermLineElemCharElemMutView<'a> {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        let (line_count, line_char_index_v) = self.string_stats();

        // TODO: Figure out how to extend.
        // TODO: Figure out how to dispatch efficiently.
        if edit.is::<st::NoOp>() {
            // Nothing to do.
        } else if edit.is::<qv::InsertionTerm>() {
            let insertion_term = edit.downcast_into::<qv::InsertionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                insertion_term.new_data.is::<char>(),
                "UTF8StringTermLineCharMutView expected InsertionTerm edit to have new_data of type UnicodeChar"
            );
            let new_char = insertion_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.line_index <= line_count, "UTF8StringTermLineCharMutView InsertionTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index] + self.char_index,
                "",
                new_char.to_string().as_str(),
            )?;
        } else if edit.is::<qv::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<qv::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                deletion_term.old_data.is::<char>(),
                "UTF8StringTermLineCharMutView expected DeletionTerm edit to have old_data of type UnicodeChar"
            );
            let old_char = deletion_term.old_data.downcast_into::<char>();
            anyhow::ensure!(self.line_index < line_count, "UTF8StringTermLineCharMutView DeletionTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index] + self.char_index,
                old_char.to_string().as_str(),
                "",
            )?;
        } else if edit.is::<qv::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                replacement_term.old_data.is::<char>(),
                "UTF8StringTermLineCharMutView expected ReplacementTerm edit to have old_data of type UnicodeChar"
            );
            anyhow::ensure!(
                replacement_term.new_data.is::<char>(),
                "UTF8StringTermLineCharMutView expected ReplacementTerm edit to have new_data of type UnicodeChar"
            );
            let old_char = replacement_term.old_data.downcast_into::<char>();
            let new_char = replacement_term.new_data.downcast_into::<char>();
            anyhow::ensure!(self.line_index < line_count, "UTF8StringTermLineCharMutView ReplacementTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index] + self.char_index,
                old_char.to_string().as_str(),
                new_char.to_string().as_str(),
            )?;
        } else {
            anyhow::bail!(
                "UTF8StringTermLineCharMutView doesn't support edit: {}",
                edit
            );
        }
        Ok(())
    }
}

impl<'b> qv::SingleQueryMutT<dy::Value> for UTF8StringTermLineElemCharElemMutView<'b> {
    type ReturnType<'a> = qv::EmptyQuery where Self: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!(
            "UTF8StringTermLineElemCharElemMutView doesn't support further queries at this time"
        );
    }
}
