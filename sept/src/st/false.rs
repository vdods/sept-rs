use crate::{dy, st::{self, Bool, FalseType, Inhabits, Stringify, True}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "FalseType", is_parametric = "false", is_type = "false")]
pub struct False;

impl dy::Deconstruct for False {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

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

impl st::NonParametricTermTrait for False {
    fn identifier() -> &'static str {
        "False"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> dy::DynNPTerm {
        dy::DynNPTerm::False
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
