use crate::{Array, Inhabits, RUNTIME, Stringify, TermTrait};
use std::any::Any;

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
pub type ArrayTerm = Vec<Box<dyn Any>>;

impl Inhabits<Array> for ArrayTerm {
    fn inhabits(&self, _: &Array) -> bool {
        true
    }
}

impl Stringify for ArrayTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Array(");
        for (i, element) in self.iter().enumerate() {
            s.push_str(&RUNTIME.stringify(element.as_ref()));
            if i+1 < self.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl TermTrait for ArrayTerm {
    type AbstractTypeFnReturnType = Array;

    fn label() -> &'static str {
        "ArrayTerm"
    }
    fn is_parametric_term(&self) -> bool {
        true
    }
    fn is_type_term(&self) -> bool {
        // TODO: Think about if this should return the AND of is_type_term for all elements.
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}
