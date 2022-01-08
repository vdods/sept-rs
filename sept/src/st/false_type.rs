use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FalseType;

impl st::Inhabits<Type> for FalseType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for FalseType {}

impl st::Inhabits<st::BoolType> for FalseType {
    fn inhabits(&self, _rhs: &st::BoolType) -> bool {
        true
    }
}

impl Stringify for FalseType {
    fn stringify(&self) -> String {
        "FalseType".into()
    }
}

impl TermTrait for FalseType {
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "FalseType"
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

impl TypeTrait for FalseType {}

pub const FALSE_TYPE: FalseType = FalseType{};
