use crate::{
    dy::{self, DynNPTerm},
    st::{self, NonParametricTermTrait, SIGNED, Stringify, TermTrait, Type, TypeTrait, UNSIGNED},
};
use std::fmt::Debug;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntNType<const IS_SIGNED: bool, const N: usize> {}

impl<const IS_SIGNED: bool, const N: usize>  dy::Deconstruct for IntNType<IS_SIGNED, N> {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Value::from(self).into()
    }
}

impl<const IS_SIGNED: bool, const N: usize> st::Inhabits<Type> for IntNType<IS_SIGNED, N> {
    fn inhabits(&self, _rhs: &Type) -> bool {
        true
    }
}

impl<const IS_SIGNED: bool, const N: usize> dy::IntoValue for IntNType<IS_SIGNED, N> {}

impl<const IS_SIGNED: bool, const N: usize> NonParametricTermTrait for IntNType<IS_SIGNED, N> {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        match IS_SIGNED {
            SIGNED => match N {
                8 => DynNPTerm::Sint8Type,
                16 => DynNPTerm::Sint16Type,
                32 => DynNPTerm::Sint32Type,
                64 => DynNPTerm::Sint64Type,
                n => panic!("unsupported Sint size: {}", n),
            },
            UNSIGNED => match N {
                8 => DynNPTerm::Uint8Type,
                16 => DynNPTerm::Uint16Type,
                32 => DynNPTerm::Uint32Type,
                64 => DynNPTerm::Uint64Type,
                n => panic!("unsupported Uint size: {}", n),
            },
        }
    }
}

impl<const IS_SIGNED: bool, const N: usize> Stringify for IntNType<IS_SIGNED, N> {
    fn stringify(&self) -> String {
        format!("{}int{}Type", if IS_SIGNED { "S" } else { "U" }, N)
    }
}

impl<const IS_SIGNED: bool, const N: usize> TermTrait for IntNType<IS_SIGNED, N> {
    type AbstractTypeType = Type;

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

impl<const IS_SIGNED: bool, const N: usize> TypeTrait for IntNType<IS_SIGNED, N> {}

pub type Sint8Type = IntNType<SIGNED, 8>;
pub type Sint16Type = IntNType<SIGNED, 16>;
pub type Sint32Type = IntNType<SIGNED, 32>;
pub type Sint64Type = IntNType<SIGNED, 64>;

pub type Uint8Type = IntNType<UNSIGNED, 8>;
pub type Uint16Type = IntNType<UNSIGNED, 16>;
pub type Uint32Type = IntNType<UNSIGNED, 32>;
pub type Uint64Type = IntNType<UNSIGNED, 64>;
