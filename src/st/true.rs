use crate::st::{Bool, Inhabits, False, Stringify, TermTrait, TrueType};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct True;

impl Inhabits<Bool> for True {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl PartialEq<bool> for True {
    fn eq(&self, other: &bool) -> bool {
        *other == true
    }
}

impl PartialEq<False> for True {
    fn eq(&self, _other: &False) -> bool {
        false
    }
}

impl Stringify for True {
    fn stringify(&self) -> String {
        "True".into()
    }
}

impl TermTrait for True {
    type AbstractTypeFnReturnType = TrueType;

    fn label() -> &'static str {
        "True"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

pub const TRUE: True = True{};
