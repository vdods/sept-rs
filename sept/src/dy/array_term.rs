use crate::{dy, st::{self, Array, Inhabits, Stringify}};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(Clone, Debug, derive_more::From, derive_more::Into, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Array", is_parametric = "self.0.len() > 0", is_type = "true")]
pub struct ArrayTerm(Vec<dy::Value>);

impl dy::Deconstruct for ArrayTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(Array.into(), self.0.into()).into()
    }
}

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
