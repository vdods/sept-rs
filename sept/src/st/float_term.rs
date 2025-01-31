use crate::{
    dy, qv,
    st::{self, Float32, Float64, InhabitsT, StringifiableT, TermT},
    Error, Result,
};

pub type Float32Term = f32;
pub type Float64Term = f64;

impl qv::ApplyEditT for f32 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for f64 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl dy::DeconstructT for f32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Float32.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for f64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Float64.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl InhabitsT<Float32> for f32 {
    fn inhabits(&self, _: &Float32) -> bool {
        true
    }
}

impl InhabitsT<Float64> for f64 {
    fn inhabits(&self, _: &Float64) -> bool {
        true
    }
}

impl dy::IntoValueT for f32 {}
impl dy::IntoValueT for f64 {}

impl st::DeserializableT for f32 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for f64 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for f32 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for f64 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

impl st::SerializableT for f32 {
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

impl st::SerializableT for f64 {
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

impl qv::SingleQueryMutT<dy::Value> for f32 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("f32 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for f64 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("f64 does not support queries at this time");
    }
}

impl StringifiableT for f32 {
    fn stringify(&self) -> String {
        // Apparently Rust, by default, formats floats with enough digits to make them unique.
        self.to_string()
    }
}

impl StringifiableT for f64 {
    fn stringify(&self) -> String {
        // Apparently Rust, by default, formats floats with enough digits to make them unique.
        self.to_string()
    }
}

impl TermT for f32 {
    type AbstractTypeType = Float32;

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

impl TermT for f64 {
    type AbstractTypeType = Float64;

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

impl st::TestValuesT for f32 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as the constants.  Note that Self::NAN is not included
        // here because it has different comparison semantics than every other value.
        vec![
            0.0,
            1.0,
            2.0,
            3.0,
            0.5,
            0.25,
            0.1,
            1.0e10,
            Self::EPSILON,
            Self::INFINITY,
            Self::MAX,
            Self::MAX_10_EXP as Self,
            Self::MAX_EXP as Self,
            Self::MIN,
            Self::MIN_10_EXP as Self,
            Self::MIN_EXP as Self,
            Self::MIN_POSITIVE,
            Self::NEG_INFINITY,
        ]
    }
}

impl st::TestValuesT for f64 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as the constants.  Note that Self::NAN is not included
        // here because it has different comparison semantics than every other value.
        vec![
            0.0,
            1.0,
            2.0,
            3.0,
            0.5,
            0.25,
            0.1,
            1.0e10,
            Self::EPSILON,
            Self::INFINITY,
            Self::MAX,
            Self::MAX_10_EXP as Self,
            Self::MAX_EXP as Self,
            Self::MIN,
            Self::MIN_10_EXP as Self,
            Self::MIN_EXP as Self,
            Self::MIN_POSITIVE,
            Self::NEG_INFINITY,
        ]
    }
}
