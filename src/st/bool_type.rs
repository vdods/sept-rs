use crate::{dy, st::{Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoolType;

impl dy::IntoValue for BoolType {}

impl Stringify for BoolType {
    fn stringify(&self) -> String {
        "BoolType".into()
    }
}

impl TermTrait for BoolType {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "BoolType"
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

impl TypeTrait for BoolType {}

pub const BOOL_TYPE: BoolType = BoolType{};
