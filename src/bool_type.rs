use crate::{Bool, FalseType, Stringify, TermTrait, TrueType, Type, TypeTrait};
use std::any::Any;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoolType;

impl Stringify for BoolType {
    fn stringify(&self) -> String {
        "BoolType".into()
    }
}

impl TermTrait for BoolType {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "BoolType"
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

impl TypeTrait for BoolType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Bool>() || x_.is::<TrueType>() || x_.is::<FalseType>()
    }
}

pub const BOOL_TYPE: BoolType = BoolType{};
