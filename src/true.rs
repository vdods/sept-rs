use crate::TermTrait;

#[derive(Debug)]
pub struct True;

impl TermTrait for True {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
}

pub const TRUE: True = True{};
