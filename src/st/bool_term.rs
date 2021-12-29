use crate::st::{Bool, False, FalseType, Inhabits, Stringify, TermTrait, True, TrueType};

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
    type AbstractTypeFnReturnType = Bool;

    fn label() -> &'static str {
        "bool"
    }
    fn is_parametric_term(&self) -> bool {
        true
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}
