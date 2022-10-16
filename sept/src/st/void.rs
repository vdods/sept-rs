use crate::{dy, st::{self, Inhabits, Stringify, VoidType}};

/// This represents the Void term itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "VoidType", is_parametric = "false", is_type = "false")]
pub struct Void;

impl Inhabits<VoidType> for Void {
    fn inhabits(&self, _: &VoidType) -> bool {
        true
    }
}

impl Stringify for Void {
    fn stringify(&self) -> String {
        "Void".into()
    }
}
