use crate::{dy::{self, DynNPTerm}, st::{self, BoolType, NonParametricTermTrait, Inhabits, Stringify, TermTrait, TypeTrait}};

/// This represents the Bool type itself, not a boolean value such as true or false.
#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "BoolType", is_parametric = "false", is_type = "true")]
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

pub const BOOL: Bool = Bool{};
