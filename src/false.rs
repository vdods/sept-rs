use crate::TermTrait;

#[derive(Debug)]
pub struct False;

impl TermTrait for False {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
}

pub const FALSE: False = False{};
