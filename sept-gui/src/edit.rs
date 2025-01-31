use crate::{CursorEdit, RootValueEdit};

#[derive(Clone, Debug, derive_more::From)]
pub enum Edit {
    CursorEdit(CursorEdit),
    RootValueEdit(RootValueEdit),
}
