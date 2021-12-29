use crate::{
    DynNPTerm, NonParametricTermTrait, SIGNED, Sint8, Sint16, Sint32, Sint64, Stringify,
    TermTrait, Type, TypeTrait, UNSIGNED, Uint8, Uint16, Uint32, Uint64,
};
use std::{any::Any, fmt::Debug};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct IntNType<const IS_SIGNED: bool, const N: usize> {}

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
    type AbstractTypeFnReturnType = Type;

    fn label() -> &'static str {
        "IntNType"
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

impl<const IS_SIGNED: bool, const N: usize> TypeTrait for IntNType<IS_SIGNED, N> {
    fn has_inhabitant(&self, x: &impl TermTrait) -> bool {
        let x_: &dyn Any = x;
        match IS_SIGNED {
            SIGNED => match N {
                8 => x_.is::<Sint8>(),
                16 => x_.is::<Sint16>(),
                32 => x_.is::<Sint32>(),
                64 => x_.is::<Sint64>(),
                n => panic!("unsupported Sint size: {}", n),
            },
            UNSIGNED => match N {
                8 => x_.is::<Uint8>(),
                16 => x_.is::<Uint16>(),
                32 => x_.is::<Uint32>(),
                64 => x_.is::<Uint64>(),
                n => panic!("unsupported Uint size: {}", n),
            },
        }
    }
}

pub type Sint8Type = IntNType<SIGNED, 8>;
pub type Sint16Type = IntNType<SIGNED, 16>;
pub type Sint32Type = IntNType<SIGNED, 32>;
pub type Sint64Type = IntNType<SIGNED, 64>;

pub type Uint8Type = IntNType<UNSIGNED, 8>;
pub type Uint16Type = IntNType<UNSIGNED, 16>;
pub type Uint32Type = IntNType<UNSIGNED, 32>;
pub type Uint64Type = IntNType<UNSIGNED, 64>;

pub const SINT8_TYPE: Sint8Type = Sint8Type{};
pub const SINT16_TYPE: Sint16Type = Sint16Type{};
pub const SINT32_TYPE: Sint32Type = Sint32Type{};
pub const SINT64_TYPE: Sint64Type = Sint64Type{};

pub const UINT8_TYPE: Uint8Type = Uint8Type{};
pub const UINT16_TYPE: Uint16Type = Uint16Type{};
pub const UINT32_TYPE: Uint32Type = Uint32Type{};
pub const UINT64_TYPE: Uint64Type = Uint64Type{};
