use crate::{TermTrait, True, TypeTrait};
use std::any::Any;

#[derive(Debug)]
pub struct TrueType;

impl TermTrait for TrueType {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl TypeTrait for TrueType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<True>() ||
        match x_.downcast_ref::<bool>() {
            Some(b) => *b,
            None => false
        }
    }
}

pub const TRUE_TYPE: TrueType = TrueType{};
