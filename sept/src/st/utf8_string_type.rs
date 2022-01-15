use crate::{dy, st::{self, Stringify, Type}};

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Utf8StringType;

impl dy::Deconstruct for Utf8StringType {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for Utf8StringType {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for Utf8StringType {
    fn as_dyn_npterm(&self) -> dy::DynNPTerm {
        dy::DynNPTerm::Utf8StringType
    }
}

impl Stringify for Utf8StringType {
    fn stringify(&self) -> String {
        "Utf8StringType".into()
    }
}
