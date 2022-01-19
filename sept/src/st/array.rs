use crate::{dy::{self, DynNPTerm}, Result, st::{self, ArrayType, Inhabits, NonParametricTermTrait, Stringify}};
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

impl dy::Deconstruct for Array {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
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

impl NonParametricTermTrait for Array {
    fn identifier() -> &'static str {
        "Array"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Array
    }
}

impl Stringify for Array {
    fn stringify(&self) -> String {
        "Array".into()
    }
}
