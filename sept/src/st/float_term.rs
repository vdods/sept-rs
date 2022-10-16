use crate::{dy, Result, st::{self, Float32, Float64, Inhabits, Stringify, TermTrait}};

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

impl st::Serializable for f32 {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl st::Serializable for f64 {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all(&self.to_le_bytes())?;
        Ok(std::mem::size_of::<Self>())
    }
}

impl Stringify for f32 {
    fn stringify(&self) -> String {
        // Apparently Rust, by default, formats floats with enough digits to make them unique.
        self.to_string()
    }
}

impl Stringify for f64 {
    fn stringify(&self) -> String {
        // Apparently Rust, by default, formats floats with enough digits to make them unique.
        self.to_string()
    }
}

impl TermTrait for f32 {
    type AbstractTypeType = Float32;

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

impl TermTrait for f64 {
    type AbstractTypeType = Float64;

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
