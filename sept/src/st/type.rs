use crate::{dy::{self, DynNPTerm}, st::{self, Inhabits, NonParametricTermTrait, Stringify}};

/// This represents the NonParametricType `Type` itself, not the trait TypeTrait.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
/// The abstract type of Type is Type itself.  This may or may not cause problems,
/// but let's go with it for now!  Really, it should be FormalTypeOf(Type).
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Type;

impl Inhabits<Type> for Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for Type {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Type
    }
}

impl Stringify for Type {
    fn stringify(&self) -> String {
        "Type".into()
    }
}
