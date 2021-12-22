use crate::{DynNPTerm, Array, NonParametricTermTrait, Stringify, TermTrait, TypeTrait};
use std::{any::Any, fmt::Debug};

#[derive(Debug, Eq, PartialEq)]
pub struct ArrayType {}

impl NonParametricTermTrait for ArrayType {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::ArrayType
    }
}

impl Stringify for ArrayType {
    fn stringify(&self) -> String {
        "ArrayType".into()
    }
}

impl TermTrait for ArrayType {
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
}

impl TypeTrait for ArrayType {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        x_.is::<Array>()
    }
}

pub const ARRAY_TYPE: ArrayType = ArrayType{};
