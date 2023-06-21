use crate::AddressedEdit;

#[derive(Clone, Debug)]
pub enum Command {
    CursorEdit(AddressedEdit),
    RootValueEdit(AddressedEdit),
}
