#[derive(Clone, Debug)]
pub struct AddressedEdit {
    pub address: sept::dy::TupleTerm,
    pub edit: sept::dy::Value,
}

impl sept::st::EditT for AddressedEdit {
    type Inverse = Self;
    fn into_inverse(self) -> Self::Inverse {
        Self {
            address: self.address,
            edit: self.edit.into_inverse(),
        }
    }
}
