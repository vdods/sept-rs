use crate::{Stringify, TermTrait};

#[derive(Debug, Eq, PartialEq)]
pub struct False;

impl PartialEq<bool> for False {
    fn eq(&self, other: &bool) -> bool {
        *other == false
    }
}
impl Stringify for False {
    fn stringify(&self) -> String {
        "False".into()
    }
}

impl TermTrait for False {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
}

pub const FALSE: False = False{};
