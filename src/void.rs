use crate::{NonParametricTermTrait, DynNPTerm, TermTrait};

/// This represents the Void term itself.
#[derive(Debug)]
pub struct Void;

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
