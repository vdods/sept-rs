use crate::{NonParametricTermTrait, DynNPTerm, Stringify, TermTrait, TypeTrait};

/// This represents the NonParametricType `Type` itself, not the trait TypeTrait.
#[derive(Debug, Eq, PartialEq)]
pub struct Type;

impl Stringify for Type {
    fn stringify(&self) -> String {
        "Type".into()
    }
}

impl TermTrait for Type {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl NonParametricTermTrait for Type {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Type
    }
}

impl TypeTrait for Type {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        x.is_type_term()
    }
}

pub const TYPE: Type = Type{};
