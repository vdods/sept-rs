use crate::{dy::{self, DynNPTerm}, st::{BoolType, NonParametricTermTrait, Inhabits, Stringify, TermTrait, TypeTrait}};

/// This represents the Bool type itself, not a boolean value such as true or false.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Bool;

impl dy::IntoValue for Bool {}

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

impl TypeTrait for Bool {}

pub const BOOL: Bool = Bool{};
