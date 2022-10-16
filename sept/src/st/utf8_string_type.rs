use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Utf8StringType;

impl st::Inhabits<Type> for Utf8StringType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl Stringify for Utf8StringType {
    fn stringify(&self) -> String {
        "Utf8StringType".into()
    }
}
