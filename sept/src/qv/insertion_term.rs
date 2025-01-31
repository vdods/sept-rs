use crate::{dy, qv, st, Result};

/// This diff represents insertion of a value into a container or other structure, and depending
/// on the container/structure, may require additional context to be fully meaningful (e.g. the
/// address of the index before which to insert).
#[derive(Clone, Debug, Eq, dy::IntoValueT, PartialEq, st::TermT)]
#[st_term_t(
    AbstractTypeType = "st::Insertion",
    is_parametric = "true",
    is_type = "false"
)]
pub struct InsertionTerm {
    pub new_data: dy::Value,
}

impl InsertionTerm {
    pub fn new(new_data: dy::Value) -> Self {
        Self { new_data }
    }
}

impl dy::DeconstructT for InsertionTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(
            st::Insertion.into(),
            dy::TupleTerm::from(vec![self.new_data]),
        )
        .into()
    }
}

impl st::DeserializableT for InsertionTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let new_data = dy::Value::deserialize(reader)?;
        Ok(Self { new_data })
    }
}

impl st::EditT for InsertionTerm {
    type Inverse = qv::DeletionTerm;
    fn into_inverse(self) -> Self::Inverse {
        qv::DeletionTerm {
            old_data: self.new_data,
        }
    }
}

impl st::InhabitsT<st::Insertion> for InsertionTerm {
    fn inhabits(&self, _rhs: &st::Insertion) -> bool {
        true
    }
}

impl st::SerializableT for InsertionTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        self.new_data.serialize(writer)
    }
}

impl st::StringifiableT for InsertionTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("InsertionTerm(");
        // DUMB
        s.push_str(&self.new_data.stringify());
        s.push_str(")");
        s
    }
}
