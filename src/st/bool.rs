use crate::{dy::DynNPTerm, st::{BoolType, False, NonParametricTermTrait, Inhabits, Stringify, TermTrait, True, TypeTrait}};
use std::any::Any;

/// This represents the Bool type itself, not a boolean value such as true or false.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bool;

impl Inhabits<BoolType> for Bool {
    fn inhabits(&self, _: &BoolType) -> bool {
        true
    }
}

impl NonParametricTermTrait for Bool {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Bool
    }
}

impl Stringify for Bool {
    fn stringify(&self) -> String {
        "Bool".into()
    }
}

impl TermTrait for Bool {
    type AbstractTypeFnReturnType = BoolType;

    fn label() -> &'static str {
        "Bool"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TypeTrait for Bool {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        // TODO: Could potentially implement this via equals
        let x_: &dyn Any = x;
        x_.is::<True>() ||
        x_.is::<False>() ||
        x_.is::<bool>() ||
        match x_.downcast_ref::<DynNPTerm>() {
            Some(DynNPTerm::True) => true,
            Some(DynNPTerm::False) => true,
            _ => false,
        }
    }
}

pub const BOOL: Bool = Bool{};
