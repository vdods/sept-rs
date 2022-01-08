use crate::{dy::{self, DynNPTerm}, st::{self, NonParametricTermTrait, Stringify, TermTrait, Type, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatNType<const N: usize> {}

impl<const N: usize> st::Inhabits<Type> for FloatNType<N> {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl<const N: usize> dy::IntoValue for FloatNType<N> {}

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
    type AbstractTypeType = Type;

    fn label() -> &'static str {
        std::any::type_name::<Self>()
    }
    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl<const N: usize> TypeTrait for FloatNType<N> {}

pub type Float32Type = FloatNType<32>;
pub type Float64Type = FloatNType<64>;

pub const FLOAT32_TYPE: Float32Type = Float32Type{};
pub const FLOAT64_TYPE: Float64Type = Float64Type{};
