use crate::{False, TermTrait, TypeTrait};
use std::any::Any;

#[derive(Debug)]
pub struct FalseType;

impl TermTrait for FalseType {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
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
