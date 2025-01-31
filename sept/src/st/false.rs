use crate::{
    dy,
    st::{self, Bool, FalseType, InhabitsT, True},
};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT)]
#[st_term_t(
    AbstractTypeType = "FalseType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct False;

impl InhabitsT<Bool> for False {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl InhabitsT<FalseType> for False {
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
