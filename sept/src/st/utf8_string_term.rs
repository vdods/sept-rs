use crate::{dy, st::{Inhabits, Stringify, TermTrait, Utf8String}};

impl dy::Deconstruct for String {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: Utf8String.into(),
            parameters: (self,).into(),
        }.into()
    }
}

impl Inhabits<Utf8String> for String {
    fn inhabits(&self, _rhs: &Utf8String) -> bool {
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
    type AbstractTypeType = Utf8String;

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
