use crate::{
    dy,
    st::{self, Bool, False, FalseType, Inhabits, Stringifiable, TermTrait, True, TrueType},
    Result,
};

pub type BoolTerm = bool;

impl dy::Deconstruct for bool {
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

impl Inhabits<Bool> for bool {
    fn inhabits(&self, _rhs: &Bool) -> bool {
        true
    }
}

impl Inhabits<FalseType> for bool {
    fn inhabits(&self, _rhs: &FalseType) -> bool {
        *self == false
    }
}

impl Inhabits<TrueType> for bool {
    fn inhabits(&self, _rhs: &TrueType) -> bool {
        *self == true
    }
}

impl dy::IntoValue for bool {}

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

impl st::Deserializable for bool {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        Ok(u8::deserialize(reader)? != 0u8)
    }
}

impl st::Serializable for bool {
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

impl Stringifiable for bool {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermTrait for bool {
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

impl st::TestValues for bool {
    fn fixed_test_values() -> Vec<Self> {
        vec![true, false]
    }
}
