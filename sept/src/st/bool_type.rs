use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoolType;

impl st::Inhabits<Type> for BoolType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl dy::IntoValue for BoolType {}

impl Stringify for BoolType {
    fn stringify(&self) -> String {
        "BoolType".into()
    }
}

impl TermTrait for BoolType {
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        "BoolType"
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

impl TypeTrait for BoolType {}

pub const BOOL_TYPE: BoolType = BoolType{};
