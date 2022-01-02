use crate::{dy, st::{Stringify, TermTrait, Type, TypeTrait}};

/// EmptyType is a Type that by definition has no inhabitants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EmptyType;

impl dy::IntoValue for EmptyType {}

impl Stringify for EmptyType {
    fn stringify(&self) -> String {
        "EmptyType".into()
    }
}

impl TermTrait for EmptyType {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "EmptyType"
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

impl TypeTrait for EmptyType {}

pub const EMPTY_TYPE: EmptyType = EmptyType{};
