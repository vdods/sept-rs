use crate::{dy, st::{self, Stringify, TermTrait, Type, TypeTrait}};

#[derive(Clone, Copy, Debug, Eq, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct VoidType;

impl dy::IntoValue for VoidType {}

impl st::Inhabits<st::Type> for VoidType {
    fn inhabits(&self, _rhs: &st::Type) -> bool {
        true
    }
}

impl Stringify for VoidType {
    fn stringify(&self) -> String {
        "VoidType".into()
    }
}

impl TypeTrait for VoidType {}

pub const VOID_TYPE: VoidType = VoidType{};
