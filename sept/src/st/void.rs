use crate::{dy, st::{self, Inhabits, NonParametricTermTrait, Stringify, VoidType}};

/// This represents the Void term itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "VoidType", is_parametric = "false", is_type = "false")]
pub struct Void;

impl Inhabits<VoidType> for Void {
    fn inhabits(&self, _: &VoidType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Void {
    fn identifier() -> &'static str {
        "Void"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Void
    }
}

impl Stringify for Void {
    fn stringify(&self) -> String {
        "Void".into()
    }
}
