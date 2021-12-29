use crate::{Stringify, TermTrait, Type, TypeTrait, Void};
use std::any::Any;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct VoidType;

impl Stringify for VoidType {
    fn stringify(&self) -> String {
        "VoidType".into()
    }
}

impl TermTrait for VoidType {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "VoidType"
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

impl TypeTrait for VoidType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Void>()
    }
}

pub const VOID_TYPE: VoidType = VoidType{};
