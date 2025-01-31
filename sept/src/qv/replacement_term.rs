use crate::{dy, st, Result};

/// This diff represents replacement of a value within a container or other structure, and depending
/// on the container/structure, may require additional context to be fully meaningful (e.g. the
/// address of the index to replace).
#[derive(Clone, Debug, Eq, dy::IntoValueT, PartialEq, st::TermT)]
#[st_term_t(
    AbstractTypeType = "st::Replacement",
    is_parametric = "true",
    is_type = "false"
)]
pub struct ReplacementTerm {
    pub old_data: dy::Value,
    pub new_data: dy::Value,
}

impl ReplacementTerm {
    pub fn new(old_data: dy::Value, new_data: dy::Value) -> Self {
        Self { old_data, new_data }
    }
}

impl dy::DeconstructT for ReplacementTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(
            st::Insertion.into(),
            dy::TupleTerm::from(vec![self.old_data, self.new_data]),
        )
        .into()
    }
}

impl st::DeserializableT for ReplacementTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let old_data = dy::Value::deserialize(reader)?;
        let new_data = dy::Value::deserialize(reader)?;
        Ok(Self { old_data, new_data })
    }
}

impl st::EditT for ReplacementTerm {
    type Inverse = ReplacementTerm;
    fn into_inverse(self) -> Self::Inverse {
        ReplacementTerm {
            old_data: self.new_data,
            new_data: self.old_data,
        }
    }
}

impl st::InhabitsT<st::Replacement> for ReplacementTerm {
    fn inhabits(&self, _rhs: &st::Replacement) -> bool {
        true
    }
}

impl st::SerializableT for ReplacementTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        let mut bytes_written = 0usize;
        bytes_written += self.old_data.serialize(writer)?;
        bytes_written += self.new_data.serialize(writer)?;
        Ok(bytes_written)
    }
}

impl st::StringifiableT for ReplacementTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("ReplacementTerm(");
        // DUMB
        s.push_str(&self.old_data.stringify());
        s.push_str(", ");
        s.push_str(&self.new_data.stringify());
        s.push_str(")");
        s
    }
}
