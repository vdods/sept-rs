use crate::{dy, st::{Bool, False, FalseType, Inhabits, Stringify, TermTrait, True, TrueType}};

impl dy::IntoValue for bool {}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        true
    }
}

impl Inhabits<Bool> for bool {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl Inhabits<FalseType> for bool {
    fn inhabits(&self, _rhs: &FalseType) -> bool {
        *self == false
    }
}

impl Inhabits<TrueType> for bool {
    fn inhabits(&self, _rhs: &TrueType) -> bool {
        *self == true
    }
}

impl PartialEq<True> for bool {
    fn eq(&self, _other: &True) -> bool {
        *self == true
    }
}

impl PartialEq<False> for bool {
    fn eq(&self, _other: &False) -> bool {
        *self == false
    }
}

impl Stringify for bool {
    fn stringify(&self) -> String {
        if *self { "True".into() } else { "False".into() }
    }
}

impl TermTrait for bool {
    type AbstractTypeType = Bool;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}
