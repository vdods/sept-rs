use crate::{Bool, Inhabits, Stringify, TermTrait, True};

#[derive(Debug, Eq, PartialEq)]
pub struct False;

impl Inhabits<Bool> for False {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl PartialEq<bool> for False {
    fn eq(&self, other: &bool) -> bool {
        *other == false
    }
}

impl PartialEq<True> for False {
    fn eq(&self, _other: &True) -> bool {
        false
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
