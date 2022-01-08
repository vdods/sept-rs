use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for TrueType {}

pub const TRUE_TYPE: TrueType = TrueType{};
