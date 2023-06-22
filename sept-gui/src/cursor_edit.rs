use crate::AddressedEdit;

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into)]
pub struct CursorEdit(AddressedEdit);
