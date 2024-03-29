use crate::{
    dy,
    st::{self, Bool, FalseType, Inhabits, True},
};

#[derive(
    Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait,
)]
#[st_term_trait(
    AbstractTypeType = "FalseType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct False;

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
