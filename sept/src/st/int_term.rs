use crate::{dy, st::{Inhabits, Sint8, Sint16, Sint32, Sint64, Stringify, TermTrait, Uint8, Uint16, Uint32, Uint64}};

impl dy::IntoValue for i8 {}
impl dy::IntoValue for i16 {}
impl dy::IntoValue for i32 {}
impl dy::IntoValue for i64 {}
impl dy::IntoValue for u8 {}
impl dy::IntoValue for u16 {}
impl dy::IntoValue for u32 {}
impl dy::IntoValue for u64 {}

impl Inhabits<Sint8> for i8 {
    fn inhabits(&self, _: &Sint8) -> bool {
        true
    }
}

impl Inhabits<Sint16> for i16 {
    fn inhabits(&self, _: &Sint16) -> bool {
        true
    }
}

impl Inhabits<Sint32> for i32 {
    fn inhabits(&self, _: &Sint32) -> bool {
        true
    }
}

impl Inhabits<Sint64> for i64 {
    fn inhabits(&self, _: &Sint64) -> bool {
        true
    }
}

impl Inhabits<Uint8> for u8 {
    fn inhabits(&self, _: &Uint8) -> bool {
        true
    }
}

impl Inhabits<Uint16> for u16 {
    fn inhabits(&self, _: &Uint16) -> bool {
        true
    }
}

impl Inhabits<Uint32> for u32 {
    fn inhabits(&self, _: &Uint32) -> bool {
        true
    }
}

impl Inhabits<Uint64> for u64 {
    fn inhabits(&self, _: &Uint64) -> bool {
        true
    }
}

impl Stringify for i8 {
    fn stringify(&self) -> String {
        format!("Sint8({})", self)
    }
}

impl Stringify for i16 {
    fn stringify(&self) -> String {
        format!("Sint16({})", self)
    }
}

impl Stringify for i32 {
    fn stringify(&self) -> String {
        format!("Sint32({})", self)
    }
}

impl Stringify for i64 {
    fn stringify(&self) -> String {
        format!("Sint64({})", self)
    }
}

impl Stringify for u8 {
    fn stringify(&self) -> String {
        format!("Uint8({})", self)
    }
}

impl Stringify for u16 {
    fn stringify(&self) -> String {
        format!("Uint16({})", self)
    }
}

impl Stringify for u32 {
    fn stringify(&self) -> String {
        format!("Uint32({})", self)
    }
}

impl Stringify for u64 {
    fn stringify(&self) -> String {
        format!("Uint64({})", self)
    }
}

impl TermTrait for i8 {
    type AbstractTypeFnReturnType = Sint8;

    fn label() -> &'static str {
        "i8"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for i16 {
    type AbstractTypeFnReturnType = Sint16;

    fn label() -> &'static str {
        "i16"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for i32 {
    type AbstractTypeFnReturnType = Sint32;

    fn label() -> &'static str {
        "i32"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for i64 {
    type AbstractTypeFnReturnType = Sint64;

    fn label() -> &'static str {
        "i64"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for u8 {
    type AbstractTypeFnReturnType = Uint8;

    fn label() -> &'static str {
        "u8"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for u16 {
    type AbstractTypeFnReturnType = Uint16;

    fn label() -> &'static str {
        "u16"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for u32 {
    type AbstractTypeFnReturnType = Uint32;

    fn label() -> &'static str {
        "u32"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl TermTrait for u64 {
    type AbstractTypeFnReturnType = Uint64;

    fn label() -> &'static str {
        "u64"
    }
    fn is_parametric_term(&self) -> bool {
        false
    }
    fn is_type_term(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}
