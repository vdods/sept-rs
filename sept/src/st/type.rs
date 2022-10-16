use crate::{dy, st::{self, Inhabits}};

/// This represents the NonParametricType `Type` itself, not the trait TypeTrait.
#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, st::NonParametricTermTrait, PartialEq, st::TermTrait, st::TypeTrait)]
/// The abstract type of Type is Type itself.  This may or may not cause problems,
/// but let's go with it for now!  Really, it should be FormalTypeOf(Type).
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Type;

impl Inhabits<Type> for Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}
