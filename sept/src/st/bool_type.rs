use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
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

impl TypeTrait for BoolType {}

pub const BOOL_TYPE: BoolType = BoolType{};
