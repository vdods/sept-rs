use crate::{dy, st::{self, Inhabits, Stringify, TermTrait}};

// TODO: Consider making a type alias for Utf8StringTerm.

impl dy::Deconstruct for String {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Utf8String.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl Inhabits<st::Utf8String> for String {
    fn inhabits(&self, _rhs: &st::Utf8String) -> bool {
        true
    }
}

impl dy::IntoValue for String {}

impl Stringify for String {
    fn stringify(&self) -> String {
        // Create a quoted string literal.
        format!("{:?}", self)
    }
}

impl TermTrait for String {
    type AbstractTypeType = st::Utf8String;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}
