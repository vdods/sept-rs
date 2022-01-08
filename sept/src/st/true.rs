use crate::{dy, st::{self, Bool, Inhabits, False, Stringify, TermTrait, TrueType}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "TrueType", is_parametric = "false", is_type = "false")]
pub struct True;

impl dy::IntoValue for True {}

impl Inhabits<Bool> for True {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl Inhabits<TrueType> for True {
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

impl Stringify for True {
    fn stringify(&self) -> String {
        "True".into()
    }
}

pub const TRUE: True = True{};
