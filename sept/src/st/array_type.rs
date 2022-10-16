use crate::{dy, st::{self, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
// TODO: AbstractTypeType could/should actually be "FormalTypeOf(ArrayType)"
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct ArrayType;

impl st::Inhabits<st::Type> for ArrayType {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl Stringify for ArrayType {
    fn stringify(&self) -> String {
        "ArrayType".into()
    }
}
