use crate::{Inhabits, NonParametricTermTrait, DynNPTerm, Stringify, TermTrait, VoidType};

/// This represents the Void term itself.
#[derive(Debug, Eq, PartialEq)]
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

impl TermTrait for Void {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
}

impl NonParametricTermTrait for Void {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Void
    }
}

pub const VOID: Void = Void{};
