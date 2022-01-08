use crate::{dy::{self, DynNPTerm}, st::{Inhabits, IntNType, NonParametricTermTrait, Stringify, TermTrait, TypeTrait}};
use std::fmt::Debug;

pub const SIGNED: bool = true;
pub const UNSIGNED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntN<const IS_SIGNED: bool, const N: usize>;

impl<const IS_SIGNED: bool, const N: usize> dy::IntoValue for IntN<IS_SIGNED, N> {}

unsafe impl<const IS_SIGNED: bool, const N: usize> Send for IntN<IS_SIGNED, N> {}
unsafe impl<const IS_SIGNED: bool, const N: usize> Sync for IntN<IS_SIGNED, N> {}

impl<const IS_SIGNED: bool, const N: usize> Inhabits<IntNType<IS_SIGNED, N>> for IntN<IS_SIGNED, N> {
    fn inhabits(&self, _: &IntNType<IS_SIGNED, N>) -> bool {
        true
    }
}

impl<const IS_SIGNED: bool, const N: usize> NonParametricTermTrait for IntN<IS_SIGNED, N> {
    fn as_dyn_npterm(&self) -> DynNPTerm {
        match IS_SIGNED {
            SIGNED => match N {
                8 => DynNPTerm::Sint8,
                16 => DynNPTerm::Sint16,
                32 => DynNPTerm::Sint32,
                64 => DynNPTerm::Sint64,
                n => panic!("unsupported Sint size: {}", n),
            },
            UNSIGNED => match N {
                8 => DynNPTerm::Uint8,
                16 => DynNPTerm::Uint16,
                32 => DynNPTerm::Uint32,
                64 => DynNPTerm::Uint64,
                n => panic!("unsupported Uint size: {}", n),
            },
        }
    }
}

impl<const IS_SIGNED: bool, const N: usize> Stringify for IntN<IS_SIGNED, N> {
    fn stringify(&self) -> String {
        format!("{}int{}", if IS_SIGNED { "S" } else { "U" }, N)
    }
}

impl<const IS_SIGNED: bool, const N: usize> TermTrait for IntN<IS_SIGNED, N> {
    type AbstractTypeType = IntNType<IS_SIGNED, N>;

    fn label() -> &'static str {
        "IntN"
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

impl<const IS_SIGNED: bool, const N: usize> TypeTrait for IntN<IS_SIGNED, N> {}

pub type Sint8 = IntN<SIGNED, 8>;
pub type Sint16 = IntN<SIGNED, 16>;
pub type Sint32 = IntN<SIGNED, 32>;
pub type Sint64 = IntN<SIGNED, 64>;

pub type Uint8 = IntN<UNSIGNED, 8>;
pub type Uint16 = IntN<UNSIGNED, 16>;
pub type Uint32 = IntN<UNSIGNED, 32>;
pub type Uint64 = IntN<UNSIGNED, 64>;

pub const SINT8: Sint8 = Sint8{};
pub const SINT16: Sint16 = Sint16{};
pub const SINT32: Sint32 = Sint32{};
pub const SINT64: Sint64 = Sint64{};

pub const UINT8: Uint8 = Uint8{};
pub const UINT16: Uint16 = Uint16{};
pub const UINT32: Uint32 = Uint32{};
pub const UINT64: Uint64 = Uint64{};
