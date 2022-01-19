use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, Type}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Float32Type;

#[derive(Clone, Copy, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait, st::TypeTrait)]
#[st_term_trait(AbstractTypeType = "Type", is_parametric = "false", is_type = "true")]
pub struct Float64Type;

impl dy::Deconstruct for Float32Type {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl dy::Deconstruct for Float64Type {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl st::Inhabits<Type> for Float32Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl st::Inhabits<Type> for Float64Type {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl NonParametricTermTrait for Float32Type {
    fn identifier() -> &'static str {
        "Float32Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Float32Type
    }
}

impl NonParametricTermTrait for Float64Type {
    fn identifier() -> &'static str {
        "Float64Type"
    }
    fn instantiate() -> Self {
        Self{}
    }
    fn as_dyn_npterm(&self) -> DynNPTerm {
        DynNPTerm::Float64Type
    }
}

impl Stringify for Float32Type {
    fn stringify(&self) -> String {
        "Float32Type".into()
    }
}

impl Stringify for Float64Type {
    fn stringify(&self) -> String {
        "Float64Type".into()
    }
}
