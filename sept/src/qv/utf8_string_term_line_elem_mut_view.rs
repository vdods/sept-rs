use crate::{dy, qv, st, Error, Result};

#[derive(Debug)]
pub struct Utf8StringTermLineElemMutView<'a> {
    // TODO: This needs to eventually be generic somehow, i.e. a String view object, or Box<dyn Borrow<str>>.
    // Or actually it should be EvalT<'b> where 'a: 'b (i.e. 'b outlives 'a).
    // Eventually there could be st-module EvalT that has a specific type.
    pub string: &'a mut String,
    pub line_index: usize,

    // Cached values -- not sure how to cache the indexed line; maybe just cache the byte offsets?
    pub line_count: usize,
    pub line_char_count: usize,
}

impl<'a> Utf8StringTermLineElemMutView<'a> {
    pub fn new(string: &'a mut String, line_index: usize) -> Result<Self> {
        let line_count = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n').count();
        anyhow::ensure!(
            line_index < line_count,
            "Utf8StringTermLineElemMutView line_index out of bounds"
        );
        let line = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n')
            .nth(line_index)
            .unwrap();
        let line_char_count = line.chars().count();
        Ok(Self {
            string,
            line_index,
            line_count,
            line_char_count,
        })
    }
    pub fn new_with_cached_values(
        string: &'a mut String,
        line_index: usize,
        line_count: usize,
        // line_char_count: usize,
    ) -> Result<Self> {
        assert_eq!(
            line_count,
            st::split_inclusive_allow_trailing_empty(string.as_str(), '\n').count(),
            "programmer error: given line_count did not match actual line_count"
        );
        anyhow::ensure!(
            line_index < line_count,
            "Utf8StringTermLineElemMutView line_index out of bounds"
        );
        let line = st::split_inclusive_allow_trailing_empty(string.as_str(), '\n')
            .nth(line_index)
            .unwrap();
        let line_char_count = line.chars().count();
        Ok(Self {
            string,
            line_index,
            line_count,
            line_char_count,
        })
    }
    /// Simultaneously compute the line count and the char indices at the beginning of each line (and the very end).
    // TODO: Don't need to return line_count, since it's part of self.
    fn string_stats(&self) -> (usize, Vec<usize>) {
        let mut char_index = 0usize;
        let mut line_char_index_v = Vec::new();
        for line in st::split_inclusive_allow_trailing_empty(self.string.as_str(), '\n') {
            line_char_index_v.push(char_index);
            char_index += line.chars().count();
        }
        line_char_index_v.push(char_index);
        (self.line_count, line_char_index_v)
    }
}

impl<'a> qv::ApplyEditT for Utf8StringTermLineElemMutView<'a> {
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
                insertion_term.new_data.is::<String>(),
                "Utf8StringTermLineMutView expected InsertionTerm edit to have new_data of type String"
            );
            let new_line = insertion_term.new_data.downcast_into::<String>();
            anyhow::ensure!(self.line_index <= line_count, "Utf8StringTermLineMutView InsertionTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index],
                "",
                new_line.as_str(),
            )?;
        } else if edit.is::<qv::DeletionTerm>() {
            let deletion_term = edit.downcast_into::<qv::DeletionTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                deletion_term.old_data.is::<String>(),
                "Utf8StringTermLineMutView expected DeletionTerm edit to have old_data of type String"
            );
            let old_line = deletion_term.old_data.downcast_into::<String>();
            anyhow::ensure!(self.line_index < line_count, "Utf8StringTermLineMutView DeletionTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index],
                old_line.as_str(),
                "",
            )?;
        } else if edit.is::<qv::ReplacementTerm>() {
            let replacement_term = edit.downcast_into::<qv::ReplacementTerm>();
            // Ideally this would invoke an st-module version of apply_edit, where all the types are known.
            anyhow::ensure!(
                replacement_term.old_data.is::<String>(),
                "Utf8StringTermLineMutView expected ReplacementTerm edit to have old_data of type String"
            );
            anyhow::ensure!(
                replacement_term.new_data.is::<String>(),
                "Utf8StringTermLineMutView expected ReplacementTerm edit to have new_data of type String"
            );
            let old_line = replacement_term.old_data.downcast_into::<String>();
            let new_line = replacement_term.new_data.downcast_into::<String>();
            anyhow::ensure!(self.line_index < line_count, "Utf8StringTermLineMutView ReplacementTerm edit had out-of-bounds line_index (line_index: {}, line_count: {})", self.line_index, line_count);
            st::replace_substr_in_string(
                &mut *self.string,
                line_char_index_v[self.line_index],
                old_line.as_str(),
                new_line.as_str(),
            )?;
        } else {
            anyhow::bail!("Utf8StringTermLineMutView doesn't support edit: {}", edit);
        }
        Ok(())
    }
}

impl<'b> qv::SingleQueryMutT<dy::Value> for Utf8StringTermLineElemMutView<'b> {
    type ReturnType<'a> = qv::Utf8StringTermLineElemCharMutView<'a> where Self: 'a;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        use st::StringifiableT;
        match address_token.downcast_ref::<String>().map(String::as_str) {
            Some("char") => qv::Utf8StringTermLineElemCharMutView::new_with_cached_values(
                self.string,
                self.line_index,
                self.line_count,
                self.line_char_count,
            ),
            _ => anyhow::bail!(
                "Utf8StringTermLineElemMutView query doesn't support address: {}",
                address_token.stringify()
            ),
        }
    }
}
