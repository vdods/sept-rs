use crate::{DynNPTerm, Inhabits, IntNType, NonParametricTermTrait, Stringify, TermTrait, TypeTrait};
use std::{any::Any, fmt::Debug};

pub const SIGNED: bool = true;
pub const UNSIGNED: bool = false;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntN<const IS_SIGNED: bool, const N: usize> {}

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
    type AbstractTypeFnReturnType = IntNType<IS_SIGNED, N>;

    fn label() -> &'static str {
        "IntN"
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

impl<const IS_SIGNED: bool, const N: usize> TypeTrait for IntN<IS_SIGNED, N> {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        match IS_SIGNED {
            SIGNED => match N {
                8 => x_.is::<i8>(),
                16 => x_.is::<i16>(),
                32 => x_.is::<i32>(),
                64 => x_.is::<i64>(),
                n => panic!("unsupported Sint size: {}", n),
            },
            UNSIGNED => match N {
                8 => x_.is::<u8>(),
                16 => x_.is::<u16>(),
                32 => x_.is::<u32>(),
                64 => x_.is::<u64>(),
                n => panic!("unsupported Uint size: {}", n),
            },
        }
    }
}

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
