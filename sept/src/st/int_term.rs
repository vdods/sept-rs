use crate::{dy, st::{self, Inhabits, Stringify, TermTrait}};

impl dy::IntoValue for i8 {}
impl dy::IntoValue for i16 {}
impl dy::IntoValue for i32 {}
impl dy::IntoValue for i64 {}
impl dy::IntoValue for u8 {}
impl dy::IntoValue for u16 {}
impl dy::IntoValue for u32 {}
impl dy::IntoValue for u64 {}

impl Inhabits<st::Sint8> for i8 {
    fn inhabits(&self, _: &st::Sint8) -> bool {
        true
    }
}

impl Inhabits<st::Sint16> for i16 {
    fn inhabits(&self, _: &st::Sint16) -> bool {
        true
    }
}

impl Inhabits<st::Sint32> for i32 {
    fn inhabits(&self, _: &st::Sint32) -> bool {
        true
    }
}

impl Inhabits<st::Sint64> for i64 {
    fn inhabits(&self, _: &st::Sint64) -> bool {
        true
    }
}

impl Inhabits<st::Uint8> for u8 {
    fn inhabits(&self, _: &st::Uint8) -> bool {
        true
    }
}

impl Inhabits<st::Uint16> for u16 {
    fn inhabits(&self, _: &st::Uint16) -> bool {
        true
    }
}

impl Inhabits<st::Uint32> for u32 {
    fn inhabits(&self, _: &st::Uint32) -> bool {
        true
    }
}

impl Inhabits<st::Uint64> for u64 {
    fn inhabits(&self, _: &st::Uint64) -> bool {
        true
    }
}

impl dy::Deconstruct for i8 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint8.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for i16 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint16.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for i32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint32.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for i64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint64.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for u8 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint8.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for u16 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint16.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for u32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint32.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for u64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint64.deconstruct(),
            vec![dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
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
    type AbstractTypeType = st::Sint8;

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
    type AbstractTypeType = st::Sint16;

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
    type AbstractTypeType = st::Sint32;

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
    type AbstractTypeType = st::Sint64;

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
    type AbstractTypeType = st::Uint8;

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
    type AbstractTypeType = st::Uint16;

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
    type AbstractTypeType = st::Uint32;

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
    type AbstractTypeType = st::Uint64;

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
