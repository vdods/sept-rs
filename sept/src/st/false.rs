use crate::{dy, st::{Bool, FalseType, Inhabits, Stringify, TermTrait, True}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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

impl TermTrait for False {
    type AbstractTypeType = FalseType;

    fn label() -> &'static str {
        "False"
    }
    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

pub const FALSE: False = False{};
