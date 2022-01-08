use crate::{dy::{self, DynNPTerm}, st::{Inhabits, FloatNType, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FloatN<const N: usize> {}

impl<const N: usize> dy::IntoValue for FloatN<N> {}

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
    type AbstractTypeType = FloatNType<N>;

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

impl<const N: usize> TypeTrait for FloatN<N> {}

pub type Float32 = FloatN<32>;
pub type Float64 = FloatN<64>;
