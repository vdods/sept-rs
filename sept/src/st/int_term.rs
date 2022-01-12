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

impl dy::Deconstruct for i8 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Sint8{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for i16 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Sint16{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for i32 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Sint32{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for i64 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Sint64{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for u8 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Uint8{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for u16 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Uint16{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for u32 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Uint32{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl dy::Deconstruct for u64 {
    fn deconstruct_into(self) -> dy::Deconstruction {
        dy::Parameterization {
            constructor: dy::Value::from(Uint64{}),
            parameters: dy::TupleTerm::from((self,)),
        }.into()
    }
}

impl Stringify for i8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for i16 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for i64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for u8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for u16 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for u32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringify for u64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermTrait for i8 {
    type AbstractTypeType = Sint8;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for i16 {
    type AbstractTypeType = Sint16;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for i32 {
    type AbstractTypeType = Sint32;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for i64 {
    type AbstractTypeType = Sint64;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for u8 {
    type AbstractTypeType = Uint8;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for u16 {
    type AbstractTypeType = Uint16;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for u32 {
    type AbstractTypeType = Uint32;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for u64 {
    type AbstractTypeType = Uint64;

    fn is_parametric(&self) -> bool {
        false
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}
