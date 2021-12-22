use crate::{Stringify, TermTrait, TypeTrait, Void};
use std::any::Any;

#[derive(Debug, Eq, PartialEq)]
pub struct VoidType;

impl Stringify for VoidType {
    fn stringify(&self) -> String {
        "VoidType".into()
    }
}

impl TermTrait for VoidType {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl TypeTrait for VoidType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Void>()
    }
}

pub const VOID_TYPE: VoidType = VoidType{};
