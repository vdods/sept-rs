use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for FalseType {}

pub const FALSE_TYPE: FalseType = FalseType{};
