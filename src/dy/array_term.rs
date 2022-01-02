// use crate::{Array, dy, Inhabits, Stringify, TermTrait};
use crate::{dy, st::{Array, Inhabits, Stringify, TermTrait}};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(Debug, derive_more::From, derive_more::Into, PartialEq)]
pub struct ArrayTerm(Vec<dy::Value>);

impl dy::IntoValue for ArrayTerm {}

impl std::ops::Deref for ArrayTerm {
    type Target = Vec<dy::Value>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::fmt::Display for ArrayTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl Inhabits<Array> for ArrayTerm {
    fn inhabits(&self, _: &Array) -> bool {
        true
    }
}

impl Stringify for ArrayTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Array(");
        for (i, element) in self.0.iter().enumerate() {
            s.push_str(&element.stringify());
            if i+1 < self.0.len() {
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
    /// An Array term is parametric if there is at least one parameter.
    fn is_parametric_term(&self) -> bool {
        self.0.len() > 0
    }
    fn is_type_term(&self) -> bool {
        // TODO: Think about if this should return the AND of is_type_term for all elements.
        // NOTE: Probably not, that particular semantic is meant for Tuple.
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}
