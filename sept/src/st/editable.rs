use crate::{dy, st, Result};

// TODO: This really does belong in dy module.

// Terminal in the sense of the terminal element in the sequence of address tokens.
pub trait TerminalEditable<E: st::DiffTrait>: st::TermTrait {
    /// Apply an edit to this term.
    fn apply_terminal_edit(&mut self, edit: &E) -> Result<()>;
}

// Nonterminal in the sense of not the terminal element in the sequence of address tokens.
pub trait NonterminalEditable<A: st::TermTrait, E: st::DiffTrait>: st::TermTrait {
    /// Apply an edit to this term.
    fn apply_nonterminal_edit<'a>(
        &mut self,
        address_head: &A,
        address_tail_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        edit: &E,
    ) -> Result<()>;
}

// TODO: Move these to an appropriate place.

/// Canonical implementation of TerminalEditable<NoOp> for any type.
impl<T: st::TermTrait> TerminalEditable<st::NoOp> for T {
    fn apply_terminal_edit(&mut self, _no_op: &st::NoOp) -> Result<()> {
        // Nothing to do.
        Ok(())
    }
}

/// Canonical implementation of NonterminalEditable<A, NoOp> for any type.
impl<A: st::TermTrait, T: st::TermTrait> NonterminalEditable<A, st::NoOp> for T {
    fn apply_nonterminal_edit<'a>(
        &mut self,
        _address_head: &A,
        _address_tail_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        _no_op: &st::NoOp,
    ) -> Result<()> {
        // Nothing to do.
        Ok(())
    }
}

/// Canonical implementation of TerminalEditable<ReplacementTerm> for any type.
impl<T: st::TermTrait> TerminalEditable<dy::ReplacementTerm> for T {
    fn apply_terminal_edit(&mut self, replacement_term: &dy::ReplacementTerm) -> Result<()> {
        use st::Stringifiable;
        let new_term = replacement_term
            .new_data
            .downcast_ref::<T>()
            .ok_or_else(|| anyhow::anyhow!("TerminalEditable<ReplacementTerm> expected new_data to be of type {} but found type {}", std::any::type_name::<T>(), replacement_term.new_data.stringify()))?;
        *self = new_term.clone();
        Ok(())
    }
}

// TODO: Make generic on integer types maybe.
// TEMP HACK: Assume indexes are u32.
impl NonterminalEditable<u32, dy::InsertionTerm> for st::Utf8StringTerm {
    fn apply_nonterminal_edit<'a>(
        &mut self,
        char_index: &u32,
        mut address_tail_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        insertion_term: &dy::InsertionTerm,
    ) -> Result<()> {
        // TODO: Implement line, char, line-char, and byte views.
        anyhow::ensure!(address_tail_i.next().is_none(), "Utf8StringTerm::apply_nonterminal_edit with InsertionTerm expected address_tail_i to be empty");
        // NOTE: This allows inserting at the end of the String using any char index at the end or after.
        // 'x' is just a dummy char.
        let (char_byte_index, _) = self
            .char_indices()
            .nth(*char_index as usize)
            .unwrap_or_else(|| (self.len(), 'x'));
        let char = *insertion_term
            .new_data
            .downcast_ref::<char>()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Utf8StringTerm::apply_nonterminal_edit with InsertionTerm expected UnicodeCharTerm for data"
                )
            })?;
        self.insert(char_byte_index, char);
        Ok(())
    }
}

// TODO: Make generic on integer types maybe.
// TEMP HACK: Assume indexes are u32.
impl NonterminalEditable<u32, dy::DeletionTerm> for st::Utf8StringTerm {
    fn apply_nonterminal_edit<'a>(
        &mut self,
        char_index: &u32,
        mut address_tail_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        deletion_term: &dy::DeletionTerm,
    ) -> Result<()> {
        // TODO: Implement line, char, line-char, and byte views.
        anyhow::ensure!(address_tail_i.next().is_none(), "Utf8StringTerm::apply_nonterminal_edit with DeletionTerm expected address_tail_i to be empty");
        let (char_byte_index, char_to_delete) = self
            .char_indices()
            .nth(*char_index as usize)
            .ok_or_else(|| anyhow::anyhow!("char_index out of range"))?;
        let old_char = *deletion_term
            .old_data
            .downcast_ref::<char>()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Utf8StringTerm::apply_nonterminal_edit with DeletionTerm expected UnicodeCharTerm for data"
                )
            })?;
        anyhow::ensure!(char_to_delete == old_char, "Utf8StringTerm::apply_nonterminal_edit with DeletionTerm expected char at index {} to be {} but found {:?}", char_index, old_char, char_to_delete);
        self.remove(char_byte_index);
        Ok(())
    }
}

impl NonterminalEditable<u32, dy::ReplacementTerm> for st::Utf8StringTerm {
    fn apply_nonterminal_edit<'a>(
        &mut self,
        char_index: &u32,
        mut address_tail_i: impl std::iter::Iterator<Item = &'a dy::Value>,
        replacement_term: &dy::ReplacementTerm,
    ) -> Result<()> {
        // TODO: Implement line, char, line-char, and byte views.
        anyhow::ensure!(address_tail_i.next().is_none(), "Utf8StringTerm::apply_nonterminal_edit with ReplacementTerm expected address_tail_i to be empty");
        let old_char = *replacement_term
            .old_data
            .downcast_ref::<char>()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Utf8StringTerm::apply_nonterminal_edit with ReplacementTerm expected UnicodeCharTerm for data"
                )
            })?;
        let new_char = *replacement_term
            .new_data
            .downcast_ref::<char>()
            .ok_or_else(|| {
                anyhow::anyhow!(
                    "Utf8StringTerm::apply_nonterminal_edit with ReplacementTerm expected UnicodeCharTerm for data"
                )
            })?;
        let actual_char_o = self.chars().nth(*char_index as usize);
        anyhow::ensure!(actual_char_o == Some(old_char), "Utf8StringTerm::apply_nonterminal_edit with ReplacementTerm expected char at index {} to be {} but found {:?}", char_index, old_char, actual_char_o);
        // Ideally this wouldn't allocate, but it's not clear if that's possible using std.
        st::replace_single_char_in_string(
            self,
            *char_index as usize,
            None,
            new_char.to_string().as_str(),
        )?;
        Ok(())
    }
}
