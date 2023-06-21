use crate::{dy, qv, st, Result};

/// This diff represents deletion of a value from a container or other structure, and depending
/// on the container/structure, may require additional context to be fully meaningful (e.g. the
/// address of the index to delete).
#[derive(Clone, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
    AbstractTypeType = "st::Deletion",
    is_parametric = "true",
    is_type = "false"
)]
pub struct DeletionTerm {
    pub old_data: dy::Value,
}

impl DeletionTerm {
    pub fn new(old_data: dy::Value) -> Self {
        Self { old_data }
    }
}

impl dy::Deconstruct for DeletionTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(
            st::Insertion.into(),
            dy::TupleTerm::from(vec![self.old_data]),
        )
        .into()
    }
}

impl st::Deserializable for DeletionTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let old_data = dy::Value::deserialize(reader)?;
        Ok(Self { old_data })
    }
}

impl st::EditTrait for DeletionTerm {
    type Inverse = qv::InsertionTerm;
    fn into_inverse(self) -> Self::Inverse {
        qv::InsertionTerm {
            new_data: self.old_data,
        }
    }
}

impl st::Inhabits<st::Deletion> for DeletionTerm {
    fn inhabits(&self, _rhs: &st::Deletion) -> bool {
        true
    }
}

impl st::Serializable for DeletionTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        self.old_data.serialize(writer)
    }
}

impl st::Stringifiable for DeletionTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("DeletionTerm(");
        // DUMB
        s.push_str(&self.old_data.stringify());
        s.push_str(")");
        s
    }
}
