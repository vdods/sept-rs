use crate::{dy, st::{False, Stringify, TermTrait, Type, TypeTrait}};
use std::any::Any;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FalseType;

impl dy::IntoValue for FalseType {}

impl Stringify for FalseType {
    fn stringify(&self) -> String {
        "FalseType".into()
    }
}

impl TermTrait for FalseType {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "FalseType"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TypeTrait for FalseType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<False>() ||
        match x_.downcast_ref::<bool>() {
            Some(b) => !*b,
            None => false
        }

    }
}

pub const FALSE_TYPE: FalseType = FalseType{};
