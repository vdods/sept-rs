use crate::{dy::DynNPTerm, st::{Inhabits, NonParametricTermTrait, Stringify, TermTrait, VoidType}};

/// This represents the Void term itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    type AbstractTypeFnReturnType = VoidType;

    fn label() -> &'static str {
        "Void"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl NonParametricTermTrait for Void {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Void
    }
}

pub const VOID: Void = Void{};
