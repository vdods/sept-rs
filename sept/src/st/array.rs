use crate::{dy, Result, st::{self, ArrayType, Inhabits, Stringify}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "ArrayType", is_parametric = "false", is_type = "true")]
pub struct Array;

impl dy::Constructor for Array {
    type ConstructedType = dy::ArrayTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        // Take the parameter elements directly.
        let parameter_v: Vec<dy::Value> = parameter_t.into();
        Ok(dy::ArrayTerm::from(parameter_v))
    }
}

impl Inhabits<ArrayType> for Array {
    fn inhabits(&self, _: &ArrayType) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for Array {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::NonParametricTermTrait for Array {
    fn identifier() -> &'static str {
        "Array"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_non_parametric_term_code() -> st::NonParametricTermCode {
        st::NonParametricTermCode::Array
    }
}

impl Stringify for Array {
    fn stringify(&self) -> String {
        "Array".into()
    }
}
