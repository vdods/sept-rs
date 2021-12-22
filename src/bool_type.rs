use crate::{Bool, FalseType, TermTrait, TrueType, TypeTrait};
use std::any::Any;

#[derive(Debug)]
pub struct BoolType;

impl TermTrait for BoolType {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl TypeTrait for BoolType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Bool>() || x_.is::<TrueType>() || x_.is::<FalseType>()
    }
}

pub const BOOL_TYPE: BoolType = BoolType{};
