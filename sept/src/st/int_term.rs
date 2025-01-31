use crate::{
    dy, qv,
    st::{self, InhabitsT, StringifiableT, TermT},
    Error, Result,
};

pub type Sint8Term = i8;
pub type Sint16Term = i16;
pub type Sint32Term = i32;
pub type Sint64Term = i64;
pub type Uint8Term = u8;
pub type Uint16Term = u16;
pub type Uint32Term = u32;
pub type Uint64Term = u64;

impl dy::IntoValueT for i8 {}
impl dy::IntoValueT for i16 {}
impl dy::IntoValueT for i32 {}
impl dy::IntoValueT for i64 {}
impl dy::IntoValueT for u8 {}
impl dy::IntoValueT for u16 {}
impl dy::IntoValueT for u32 {}
impl dy::IntoValueT for u64 {}

impl qv::ApplyEditT for i8 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for i16 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for i32 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for i64 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for u8 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for u16 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for u32 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl qv::ApplyEditT for u64 {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl InhabitsT<st::Sint8> for i8 {
    fn inhabits(&self, _: &st::Sint8) -> bool {
        true
    }
}

impl InhabitsT<st::Sint16> for i16 {
    fn inhabits(&self, _: &st::Sint16) -> bool {
        true
    }
}

impl InhabitsT<st::Sint32> for i32 {
    fn inhabits(&self, _: &st::Sint32) -> bool {
        true
    }
}

impl InhabitsT<st::Sint64> for i64 {
    fn inhabits(&self, _: &st::Sint64) -> bool {
        true
    }
}

impl InhabitsT<st::Uint8> for u8 {
    fn inhabits(&self, _: &st::Uint8) -> bool {
        true
    }
}

impl InhabitsT<st::Uint16> for u16 {
    fn inhabits(&self, _: &st::Uint16) -> bool {
        true
    }
}

impl InhabitsT<st::Uint32> for u32 {
    fn inhabits(&self, _: &st::Uint32) -> bool {
        true
    }
}

impl InhabitsT<st::Uint64> for u64 {
    fn inhabits(&self, _: &st::Uint64) -> bool {
        true
    }
}

impl dy::DeconstructT for i8 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint8.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for i16 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint16.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for i32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint32.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for i64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Sint64.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for u8 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint8.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for u16 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint16.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for u32 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint32.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl dy::DeconstructT for u64 {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Uint64.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl st::DeserializableT for i8 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for i16 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for i32 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for i64 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for u8 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for u16 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for u32 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

impl st::DeserializableT for u64 {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<Self>()];
        reader.read_exact(&mut buffer)?;
        Ok(Self::from_le_bytes(buffer))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for i8 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for i16 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for i32 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for i64 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for u8 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for u16 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for u32 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

// TODO: Replace this with the full set of queries
impl qv::QueryableDynT for u64 {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

impl st::SerializableT for i8 {
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

impl st::SerializableT for i16 {
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

impl st::SerializableT for i32 {
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

impl st::SerializableT for i64 {
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

impl st::SerializableT for u8 {
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

impl st::SerializableT for u16 {
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

impl st::SerializableT for u32 {
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

impl st::SerializableT for u64 {
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

impl qv::SingleQueryMutT<dy::Value> for i8 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("i8 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for i16 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("i16 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for i32 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("i32 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for i64 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("i64 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for u8 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("u8 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for u16 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("u16 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for u32 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("u32 does not support queries at this time");
    }
}

impl qv::SingleQueryMutT<dy::Value> for u64 {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("u64 does not support queries at this time");
    }
}

impl StringifiableT for i8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for i16 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for i32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for i64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for u8 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for u16 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for u32 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl StringifiableT for u64 {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermT for i8 {
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

impl TermT for i16 {
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

impl TermT for i32 {
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

impl TermT for i64 {
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

impl TermT for u8 {
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

impl TermT for u16 {
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

impl TermT for u32 {
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

impl TermT for u64 {
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

impl st::TestValuesT for i8 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for i16 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for i32 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for i64 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, -1, 2, -2, 3, -3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for u8 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for u16 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for u32 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}

impl st::TestValuesT for u64 {
    fn fixed_test_values() -> Vec<Self> {
        // Just do some common values as well as MAX and MIN.
        vec![0, 1, 2, 3, Self::MIN, Self::MAX]
    }
}
