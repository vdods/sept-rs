use crate::{
    dy, qv,
    st::{self, Bool, False, FalseType, InhabitsT, StringifiableT, TermT, True, TrueType},
    Error, Result,
};

pub type BoolTerm = bool;

impl qv::ApplyEditT for bool {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl dy::DeconstructT for bool {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Bool.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl From<True> for bool {
    fn from(_: True) -> Self {
        true
    }
}

impl From<False> for bool {
    fn from(_: False) -> Self {
        true
    }
}

impl InhabitsT<Bool> for bool {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl InhabitsT<FalseType> for bool {
    fn inhabits(&self, _rhs: &FalseType) -> bool {
        *self == false
    }
}

impl InhabitsT<TrueType> for bool {
    fn inhabits(&self, _rhs: &TrueType) -> bool {
        *self == true
    }
}

impl dy::IntoValueT for bool {}

impl PartialEq<True> for bool {
    fn eq(&self, _other: &True) -> bool {
        *self == true
    }
}

impl PartialEq<False> for bool {
    fn eq(&self, _other: &False) -> bool {
        *self == false
    }
}

impl st::DeserializableT for bool {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        Ok(u8::deserialize(reader)? != 0u8)
    }
}

impl qv::QueryableDynT for bool {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

impl st::SerializableT for bool {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Bool.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // Represent as u8.
        let n = if *self { 1u8 } else { 0u8 };
        Ok(n.serialize(writer)?)
    }
}

impl qv::SingleQueryMutT<dy::Value> for bool {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("bool does not support queries at this time");
    }
}

impl StringifiableT for bool {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermT for bool {
    type AbstractTypeType = Bool;

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

impl st::TestValuesT for bool {
    fn fixed_test_values() -> Vec<Self> {
        vec![true, false]
    }
}
