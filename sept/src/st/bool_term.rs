use crate::{dy, st::{self, Bool, False, FalseType, Inhabits, Stringify, TermTrait, True, TrueType}};

impl dy::Deconstruct for bool {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Bool.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

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

impl dy::IntoValue for bool {}

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
        self.to_string()
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
