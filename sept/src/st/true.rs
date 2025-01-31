use crate::{
    dy,
    st::{self, Bool, False, InhabitsT, TrueType},
};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValueT, st::NonParametricTermT, PartialEq, st::TermT)]
#[st_term_t(
    AbstractTypeType = "TrueType",
    is_parametric = "false",
    is_type = "false"
)]
pub struct True;

impl InhabitsT<Bool> for True {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl InhabitsT<TrueType> for True {
    fn inhabits(&self, _rhs: &TrueType) -> bool {
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
