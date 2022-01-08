use crate::{dy, st::{self, Bool, FalseType, Inhabits, Stringify, TermTrait, True}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "FalseType", is_parametric = "false", is_type = "false")]
pub struct False;

impl dy::IntoValue for False {}

impl Inhabits<Bool> for False {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl Inhabits<FalseType> for False {
    fn inhabits(&self, _rhs: &FalseType) -> bool {
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

pub const FALSE: False = False{};
