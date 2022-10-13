use crate::{dy, Result};

/// This represents a deconstructed term, but is currently not a first class term within the sept data
/// model, since it's only meant to facilitate textify/parse and de/serialize.
// TODO: Other schemas for deconstruction probably also make sense, such as Tabular (a StructTerm
// would fit this, because its data consists of two columns: "field name" and "field type"), and
// linked data structures that have nodes on a heap and pointers to the heap (e.g. Tree, DAG)
#[derive(Clone, Debug, enum_kinds::EnumKind, PartialEq)]
#[enum_kind(DeconstructionKind)]
pub enum Deconstruction {
    NonParametric(dy::NonParametricDeconstruction),
    /// Box is necessary because Deconstruction and ParametricDeconstruction are mutually recursive data structures.
    Parametric(Box<dy::ParametricDeconstruction>),
}

impl From<dy::NonParametricDeconstruction> for Deconstruction {
    fn from(non_parametric_deconstruction: dy::NonParametricDeconstruction) -> Self {
        Self::NonParametric(non_parametric_deconstruction)
    }
}

impl From<dy::ParametricDeconstruction> for Deconstruction {
    fn from(parametric_deconstruction: dy::ParametricDeconstruction) -> Self {
        Self::Parametric(Box::new(parametric_deconstruction))
    }
}

impl Deconstruction {
    pub fn kind(&self) -> DeconstructionKind {
        self.into()
    }
    pub fn is_non_parametric(&self) -> bool {
        match self {
            Deconstruction::NonParametric(_) => true,
            _ => false
        }
    }
    pub fn is_parametric(&self) -> bool {
        match self {
            Deconstruction::Parametric(_) => true,
            _ => false
        }
    }
    pub fn into_non_parametric(self) -> Option<dy::NonParametricDeconstruction> {
        match self {
            Deconstruction::NonParametric(non_parametric_deconstruction) => Some(non_parametric_deconstruction),
            _ => None,
        }
    }
    pub fn into_parametric(self) -> Option<dy::ParametricDeconstruction> {
        match self {
            Deconstruction::Parametric(parametric_deconstruction_b) => Some(*parametric_deconstruction_b),
            _ => None,
        }
    }
    // TODO: This should be derivable via trait macro (also requires a Reconstruct trait).
    // TODO: Also make a reconstructed method which operates by &self
    pub fn reconstruct(self) -> Result<dy::Value> {
        match self {
            dy::Deconstruction::NonParametric(non_parametric_deconstruction) => Ok(non_parametric_deconstruction.reconstruct()?),
            dy::Deconstruction::Parametric(parametric_deconstruction) => Ok(parametric_deconstruction.reconstruct()?),
        }
    }
}
