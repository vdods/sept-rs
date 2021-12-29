use crate::{DynNPTerm, Inhabits, FloatNType, NonParametricTermTrait, Stringify, TermTrait, TypeTrait};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatN<const N: usize> {}

impl<const N: usize> Inhabits<FloatNType<N>> for FloatN<N> {
    fn inhabits(&self, _: &FloatNType<N>) -> bool {
        true
    }
}

impl<const N: usize> NonParametricTermTrait for FloatN<N> {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        match N {
            32 => DynNPTerm::Float32,
            64 => DynNPTerm::Float64,
            n => panic!("unsupported Float size: {}", n),
        }
    }
}

impl<const N: usize> Stringify for FloatN<N> {
    fn stringify(&self) -> String {
        format!("Float{}", N)
    }
}

impl<const N: usize> TermTrait for FloatN<N> {
    type AbstractTypeFnReturnType = FloatNType<N>;

    fn label() -> &'static str {
        "FloatN"
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

impl<const N: usize> TypeTrait for FloatN<N> {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        match N {
            32 => x_.is::<f32>(),
            64 => x_.is::<f64>(),
            n => panic!("unsupported Float size: {}", n),
        }
    }
}

pub type Float32 = FloatN<32>;
pub type Float64 = FloatN<64>;

pub const FLOAT32: Float32 = Float32{};
pub const FLOAT64: Float64 = Float64{};
