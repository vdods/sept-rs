use crate::{CursorEdit, RootValueEdit};

#[derive(Clone, Debug, derive_more::From)]
pub enum Command {
    CursorEdit(CursorEdit),
    RootValueEdit(RootValueEdit),
}
