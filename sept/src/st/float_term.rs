use crate::{dy, Result, st::{self, Float32, Float64, Inhabits, Stringifiable, TermTrait}};

pub type Float32Term = f32;
pub type Float64Term = f64;

impl dy::Deconstruct for f32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Float32.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl dy::Deconstruct for f64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Float64.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl Inhabits<Float32> for f32 {
    fn inhabits(&self, _: &Float32) -> bool {
        true
    }
}

impl Inhabits<Float64> for f64 {
    fn inhabits(&self, _: &Float64) -> bool {
        true
    }
}

impl dy::IntoValue for f32 {}
impl dy::IntoValue for f64 {}

impl st::Deserializable for f32 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Deserializable for f64 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::Serializable for f32 {
//     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
//     }
//     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(st::Float32.serialize(writer)?)
//     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for f64 {
//     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
//     }
//     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(st::Float64.serialize(writer)?)
//     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl Stringifiable for f32 {
    fn stringify(&self) -> String {
        // Apparently Rust, by default, formats floats with enough digits to make them unique.
        self.to_string()
    }
}

impl Stringifiable for f64 {
    fn stringify(&self) -> String {
        // Apparently Rust, by default, formats floats with enough digits to make them unique.
        self.to_string()
    }
}

impl TermTrait for f32 {
    type AbstractTypeType = Float32;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl TermTrait for f64 {
    type AbstractTypeType = Float64;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}

impl st::TestValues for f32 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as the constants.  Note that Self::NAN is not included
        // here because it has different comparison semantics than every other value.
        vec![
            0.0, 1.0, 2.0, 3.0, 0.5, 0.25, 0.1, 1.0e10, Self::EPSILON, Self::INFINITY, Self::MAX,
            Self::MAX_10_EXP as Self, Self::MAX_EXP as Self, Self::MIN, Self::MIN_10_EXP as Self,
            Self::MIN_EXP as Self, Self::MIN_POSITIVE, Self::NEG_INFINITY,
        ]
    }
}

impl st::TestValues for f64 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as the constants.  Note that Self::NAN is not included
        // here because it has different comparison semantics than every other value.
        vec![
            0.0, 1.0, 2.0, 3.0, 0.5, 0.25, 0.1, 1.0e10, Self::EPSILON, Self::INFINITY, Self::MAX,
            Self::MAX_10_EXP as Self, Self::MAX_EXP as Self, Self::MIN, Self::MIN_10_EXP as Self,
            Self::MIN_EXP as Self, Self::MIN_POSITIVE, Self::NEG_INFINITY,
        ]
    }
}
