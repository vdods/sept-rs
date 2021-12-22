use crate::{Stringify, TermTrait, TypeTrait};

/// EmptyType is a Type that by definition has no inhabitants.
#[derive(Debug, Eq, PartialEq)]
pub struct EmptyType;

impl Stringify for EmptyType {
    fn stringify(&self) -> String {
        "EmptyType".into()
    }
}

impl TermTrait for EmptyType {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl TypeTrait for EmptyType {
    fn has_inhabitant(&self, _x: &impl TermTrait) -> bool {
        // No inhabitants by definition
        false
    }
}

pub const EMPTY_TYPE: EmptyType = EmptyType{};
