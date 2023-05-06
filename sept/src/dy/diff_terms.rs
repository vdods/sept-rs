// TODO: This should probably be broken up into multiple files and placed in smart locations.

use crate::{dy, st, Result};

/// This diff represents insertion of a value into a container or other structure, and depending
/// on the container/structure, may require additional context to be fully meaningful (e.g. the
/// address of the index before which to insert).
#[derive(Clone, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
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

impl dy::Deconstruct for InsertionTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(
            st::Insertion.into(),
            dy::TupleTerm::from(vec![self.new_data]),
        )
        .into()
    }
}

impl st::Deserializable for InsertionTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let new_data = dy::Value::deserialize(reader)?;
        Ok(Self { new_data })
    }
}

impl st::DiffTrait for InsertionTerm {
    type Inverse = DeletionTerm;
    fn into_inverse(self) -> Self::Inverse {
        DeletionTerm {
            old_data: self.new_data,
        }
    }
}

impl st::Inhabits<st::Insertion> for InsertionTerm {
    fn inhabits(&self, _rhs: &st::Insertion) -> bool {
        true
    }
}

impl st::Serializable for InsertionTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        self.new_data.serialize(writer)
    }
}

impl st::Stringifiable for InsertionTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("InsertionTerm(");
        // DUMB
        s.push_str(&self.new_data.stringify());
        s.push_str(")");
        s
    }
}

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

impl st::DiffTrait for DeletionTerm {
    type Inverse = InsertionTerm;
    fn into_inverse(self) -> Self::Inverse {
        InsertionTerm {
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

/// This diff represents replacement of a value within a container or other structure, and depending
/// on the container/structure, may require additional context to be fully meaningful (e.g. the
/// address of the index to replace).
#[derive(Clone, Debug, Eq, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(
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

impl dy::Deconstruct for ReplacementTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(
            st::Insertion.into(),
            dy::TupleTerm::from(vec![self.old_data, self.new_data]),
        )
        .into()
    }
}

impl st::Deserializable for ReplacementTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let old_data = dy::Value::deserialize(reader)?;
        let new_data = dy::Value::deserialize(reader)?;
        Ok(Self { old_data, new_data })
    }
}

impl st::DiffTrait for ReplacementTerm {
    type Inverse = ReplacementTerm;
    fn into_inverse(self) -> Self::Inverse {
        ReplacementTerm {
            old_data: self.new_data,
            new_data: self.old_data,
        }
    }
}

impl st::Inhabits<st::Replacement> for ReplacementTerm {
    fn inhabits(&self, _rhs: &st::Replacement) -> bool {
        true
    }
}

impl st::Serializable for ReplacementTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        let mut bytes_written = 0usize;
        bytes_written += self.old_data.serialize(writer)?;
        bytes_written += self.new_data.serialize(writer)?;
        Ok(bytes_written)
    }
}

impl st::Stringifiable for ReplacementTerm {
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
