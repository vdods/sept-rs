use crate::{dy, st::{self, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct TrueType;

impl st::Inhabits<Type> for TrueType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<st::BoolType> for TrueType {
    fn inhabits(&self, _rhs: &st::BoolType) -> bool {
        true
    }
}
