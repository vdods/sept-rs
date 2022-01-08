use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

/// EmptyType is a Type that by definition has no inhabitants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmptyType;

impl st::Inhabits<Type> for EmptyType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl<T: TermTrait + dy::IntoValue + 'static> st::Inhabits<EmptyType> for T {
    /// Nothing inhabits EmptyType.
    fn inhabits(&self, _rhs: &EmptyType) -> bool {
        false
    }
}

impl dy::IntoValue for EmptyType {}

impl Stringify for EmptyType {
    fn stringify(&self) -> String {
        "EmptyType".into()
    }
}

impl TermTrait for EmptyType {
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "EmptyType"
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

impl TypeTrait for EmptyType {}

pub const EMPTY_TYPE: EmptyType = EmptyType{};
