use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TrueType;

impl st::Inhabits<Type> for TrueType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for TrueType {}

impl st::Inhabits<st::BoolType> for TrueType {
    fn inhabits(&self, _rhs: &st::BoolType) -> bool {
        true
    }
}

impl Stringify for TrueType {
    fn stringify(&self) -> String {
        "TrueType".into()
    }
}

impl TermTrait for TrueType {
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "TrueType"
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

impl TypeTrait for TrueType {}

pub const TRUE_TYPE: TrueType = TrueType{};
