use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct BoolType;

impl st::Inhabits<Type> for BoolType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl Stringify for BoolType {
    fn stringify(&self) -> String {
        "BoolType".into()
    }
}
