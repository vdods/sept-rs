#[derive(Clone, Debug)]
pub struct AddressedEdit {
    pub address: sept::dy::TupleTerm,
    pub edit: sept::dy::Value,
}
