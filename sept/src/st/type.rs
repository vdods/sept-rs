use crate::{dy::{self, DynNPTerm}, st::{Inhabits, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};

/// This represents the NonParametricType `Type` itself, not the trait TypeTrait.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Type;

impl dy::IntoValue for Type {}

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
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "Type"
    }
    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TypeTrait for Type {}

pub const TYPE: Type = Type{};