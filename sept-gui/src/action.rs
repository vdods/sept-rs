use crate::{AddressedEdit, Edit};

/// An action is comprised of a sequence of edits, and is the unit of undo/redo.
#[derive(Clone, Debug)]
pub struct Action {
    edit_v: Vec<Edit>,
    cursor_edit_count: usize,
    root_value_edit_count: usize,
}

impl Action {
    /// Create a new Action from a sequence of edits.
    pub fn new(edit_v: Vec<Edit>) -> Self {
        let cursor_edit_count = edit_v
            .iter()
            .filter(|edit| matches!(edit, Edit::CursorEdit(_)))
            .count();
        let root_value_edit_count = edit_v
            .iter()
            .filter(|edit| matches!(edit, Edit::RootValueEdit(_)))
            .count();
        Self {
            edit_v,
            cursor_edit_count,
            root_value_edit_count,
        }
    }
    pub fn edit_v(&self) -> &[Edit] {
        &self.edit_v
    }
    pub fn cursor_edit_count(&self) -> usize {
        self.cursor_edit_count
    }
    pub fn root_value_edit_count(&self) -> usize {
        self.root_value_edit_count
    }
    /// Apply the edits in this Action.
    pub fn apply(
        &self,
        root_value: &mut sept::dy::Value,
        cursor_address: &mut sept::dy::TupleTerm,
    ) -> anyhow::Result<()> {
        Self::apply_edits(&self.edit_v, root_value, cursor_address)
    }
    /// Revert the edits in this Action.
    pub fn revert(
        &self,
        root_value: &mut sept::dy::Value,
        cursor_address: &mut sept::dy::TupleTerm,
    ) {
        Self::revert_edits(&self.edit_v, root_value, cursor_address)
    }
    /// Apply the sequence of edits as a transaction.  If any of them fail, roll the applied edits back.
    fn apply_edits(
        edit_v: &[Edit],
        root_value: &mut sept::dy::Value,
        cursor_address: &mut sept::dy::TupleTerm,
    ) -> anyhow::Result<()> {
        let mut successful_edit_count = 0usize;
        for edit in edit_v {
            tracing::trace!("Action::apply; executing edit {:?}", edit);
            use sept::qv::QueryMutAndApplyEditT;
            match edit {
                Edit::CursorEdit(cursor_edit) => {
                    cursor_address
                        .query_mut_and_apply_edit(
                            &mut cursor_edit.address.iter(),
                            cursor_edit.edit.clone(),
                        )
                        .expect("programmer error: error in CursorEdit");
                }
                Edit::RootValueEdit(root_value_edit) => {
                    if let Err(e) = root_value.query_mut_and_apply_edit(
                        &mut root_value_edit.address.iter(),
                        root_value_edit.edit.clone(),
                    ) {
                        tracing::error!(
                            "Action::apply; Edit error: {} -- rolling back applied edits",
                            e
                        );
                        Self::revert_edits(
                            &edit_v[0..successful_edit_count],
                            root_value,
                            cursor_address,
                        );
                        return Err(e);
                    }
                }
            }
            successful_edit_count += 1;
        }
        Ok(())
    }
    /// Revert the given edits in reverse order.  This will panic if any of the edits produce an error.
    fn revert_edits(
        edit_v: &[Edit],
        root_value: &mut sept::dy::Value,
        cursor_address: &mut sept::dy::TupleTerm,
    ) {
        use sept::qv::QueryMutAndApplyEditT;
        use sept::st::EditT;
        for edit in edit_v.iter().rev() {
            match edit {
                Edit::CursorEdit(cursor_edit) => {
                    // TODO: Figure out how to not clone cursor_edit.
                    let AddressedEdit {
                        address,
                        edit: edit_inv,
                    } = cursor_edit.clone().into_inverse().into();
                    cursor_address
                        .query_mut_and_apply_edit(
                            &mut address.iter(),
                            edit_inv,
                        )
                        .expect("programmer error: there is some problem with the definition of some EditT inverse.");
                }
                Edit::RootValueEdit(root_value_edit) => {
                    // TODO: Figure out how to not clone root_value_edit.
                    let AddressedEdit {
                        address,
                        edit: edit_inv,
                    } = root_value_edit.clone().into_inverse().into();
                    root_value.query_mut_and_apply_edit(
                        &mut address.iter(),
                        edit_inv,
                    )
                    .expect("programmer error: there is some problem with the definition of some EditT inverse.");
                }
            }
        }
    }
}
