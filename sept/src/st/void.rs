use crate::{dy::{self, DynNPTerm}, st::{self, Inhabits, NonParametricTermTrait, Stringify, VoidType}};

/// This represents the Void term itself.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
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

impl NonParametricTermTrait for Void {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Void
    }
}

pub const VOID: Void = Void{};
