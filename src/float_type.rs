use crate::{DynNPTerm, Float32, Float64, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait};
use std::{any::Any, fmt::Debug};

#[derive(Debug, Eq, PartialEq)]
pub struct FloatNType<const N: usize> {}

impl<const N: usize> NonParametricTermTrait for FloatNType<N> {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        match N {
            32 => DynNPTerm::Float32Type,
            64 => DynNPTerm::Float64Type,
            n => panic!("unsupported Float size: {}", n),
        }
    }
}

impl<const N: usize> Stringify for FloatNType<N> {
    fn stringify(&self) -> String {
        format!("Float{}Type", N)
    }
}

impl<const N: usize> TermTrait for FloatNType<N> {
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "FloatNType"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl<const N: usize> TypeTrait for FloatNType<N> {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        match N {
            32 => x_.is::<Float32>(),
            64 => x_.is::<Float64>(),
            n => panic!("unsupported Float size: {}", n),
        }
    }
}

pub type Float32Type = FloatNType<32>;
pub type Float64Type = FloatNType<64>;

pub const FLOAT32_TYPE: Float32Type = Float32Type{};
pub const FLOAT64_TYPE: Float64Type = Float64Type{};
