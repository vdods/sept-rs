use crate::{dy::{self, DynNPTerm}, st::{Inhabits, NonParametricTermTrait, Stringify, TermTrait, VoidType}};

/// This represents the Void term itself.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Void;

impl dy::IntoValue for Void {}

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
    type AbstractTypeType = VoidType;

    fn label() -> &'static str {
        "Void"
    }
    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl NonParametricTermTrait for Void {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Void
    }
}

pub const VOID: Void = Void{};
