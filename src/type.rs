use crate::{Inhabits, DynNPTerm, NonParametricTermTrait, Stringify, TermTrait, TypeTrait};

/// This represents the NonParametricType `Type` itself, not the trait TypeTrait.
#[derive(Debug, Eq, PartialEq)]
pub struct Type;

impl Inhabits<Type> for Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for Type {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Type
    }
}

impl Stringify for Type {
    fn stringify(&self) -> String {
        "Type".into()
    }
}

impl TermTrait for Type {
    /// The abstract type of Type is Type itself.  This may or may not cause problems,
    /// but let's go with it for now!  Really, it should be FormalTypeOf(Type).
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "Type"
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

impl TypeTrait for Type {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        x.is_type_term()
    }
}

pub const TYPE: Type = Type{};
