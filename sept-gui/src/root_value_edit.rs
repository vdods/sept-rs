use crate::AddressedEdit;

#[derive(Clone, Debug, derive_more::Deref, derive_more::From, derive_more::Into)]
pub struct RootValueEdit(AddressedEdit);

// TODO: Derive this somehow
impl sept::st::EditTrait for RootValueEdit {
    type Inverse = Self;
    fn into_inverse(self) -> Self::Inverse {
        Self(self.0.into_inverse())
    }
}
