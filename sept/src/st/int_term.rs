use crate::{
    dy,
    st::{self, Inhabits, Stringifiable, TermTrait},
    Result,
};

pub type Sint8Term = i8;
pub type Sint16Term = i16;
pub type Sint32Term = i32;
pub type Sint64Term = i64;
pub type Uint8Term = u8;
pub type Uint16Term = u16;
pub type Uint32Term = u32;
pub type Uint64Term = u64;

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
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for i16 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint16.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for i32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint32.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for i64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint64.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for u8 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint8.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for u16 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint16.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for u32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint32.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::Deconstruct for u64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint64.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl st::Deserializable for i8 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for i16 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for i32 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for i64 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for u8 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for u16 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for u32 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for u64 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Serializable for i8 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Sint8.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for i16 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Sint16.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for i32 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Sint32.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for i64 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Sint64.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for u8 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Uint8.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for u16 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Uint16.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for u32 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Uint32.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for u64 {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Uint64.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl Stringifiable for i8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for i16 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for i64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for u8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for u16 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for u32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl Stringifiable for u64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermTrait for i8 {
    type AbstractTypeType = st::Sint8;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for i16 {
    type AbstractTypeType = st::Sint16;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for i32 {
    type AbstractTypeType = st::Sint32;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for i64 {
    type AbstractTypeType = st::Sint64;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for u8 {
    type AbstractTypeType = st::Uint8;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for u16 {
    type AbstractTypeType = st::Uint16;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for u32 {
    type AbstractTypeType = st::Uint32;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl TermTrait for u64 {
    type AbstractTypeType = st::Uint64;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType {}
    }
}

impl st::TestValues for i8 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for i16 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for i32 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for i64 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for u8 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for u16 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for u32 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}

impl st::TestValues for u64 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}
