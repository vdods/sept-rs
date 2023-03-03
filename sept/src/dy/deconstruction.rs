use crate::{dy, Result};

/// This represents a deconstructed term, but is currently not a first class term within the sept data
/// model, since it's only meant to facilitate textify/parse and de/serialize.
// TODO: Other schemas for deconstruction probably also make sense, such as Tabular (a StructTerm
// would fit this, because its data consists of two columns: "field name" and "field type"), and
// linked data structures that have nodes on a heap and pointers to the heap (e.g. Tree, DAG)
// TODO: Consider combining NonParametric and Terminal into Terminal, since it seems both require
// a kind of type code in order to be able to deserialize.  A Terminal is basically a one-level
// ParametricDeconstruction, where the constructor is a NonParametricTerm.
#[derive(Clone, Debug, enum_kinds::EnumKind, PartialEq)]
#[enum_kind(DeconstructionKind, repr(u8))]
pub enum Deconstruction {
    /// First base case -- non-parametric terms
    NonParametric(dy::NonParametricDeconstruction),
    /// Second base case -- parametric terms from a finite set of types defined to be terminals.
    Terminal(dy::TerminalDeconstruction),
    /// Box is necessary because Deconstruction and ParametricDeconstruction are mutually recursive data structures.
    Parametric(Box<dy::ParametricDeconstruction>),
}

// // TODO: Figure out if this can be derived through enum_kinds somehow.
// impl TryFrom<u8> for DeconstructionKind {
//     type Error = Error;
//     fn try_from(n: u8) -> std::result::Result<Self, Self::Error> {
//         let deconstruction_kind = match n {
// //             DeconstructionKind::NonParametric as u8 => DeconstructionKind::NonParametric,
// //             DeconstructionKind::Terminal as u8 => DeconstructionKind::Terminal,
// //             DeconstructionKind::Parametric as u8 => DeconstructionKind::Parametric,
//             // TODO: This is fragile, figure out how to make it robust.
//             0u8 => DeconstructionKind::NonParametric,
//             1u8 => DeconstructionKind::Terminal,
//             2u8 => DeconstructionKind::Parametric,
//             _ => { anyhow::bail!("invalid DeconstructionKind code {}", n); }
//         };
//         Ok(deconstruction_kind)
//     }
// }

impl From<dy::NonParametricDeconstruction> for Deconstruction {
    fn from(non_parametric_deconstruction: dy::NonParametricDeconstruction) -> Self {
        Self::NonParametric(non_parametric_deconstruction)
    }
}

impl From<dy::TerminalDeconstruction> for Deconstruction {
    fn from(terminal_deconstruction: dy::TerminalDeconstruction) -> Self {
        Self::Terminal(terminal_deconstruction)
    }
}

/// Note that this applies Box::new to the argument, so it's not canonically derivable.
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
            _ => false,
        }
    }
    pub fn is_terminal(&self) -> bool {
        match self {
            Deconstruction::Terminal(_) => true,
            _ => false,
        }
    }
    pub fn is_parametric(&self) -> bool {
        match self {
            Deconstruction::Parametric(_) => true,
            _ => false,
        }
    }
    pub fn into_non_parametric(self) -> Option<dy::NonParametricDeconstruction> {
        match self {
            Deconstruction::NonParametric(non_parametric_deconstruction) => Some(non_parametric_deconstruction),
            _ => None,
        }
    }
    pub fn into_terminal(self) -> Option<dy::TerminalDeconstruction> {
        match self {
            Deconstruction::Terminal(terminal_deconstruction) => Some(terminal_deconstruction),
            _ => None,
        }
    }
    pub fn into_parametric(self) -> Option<dy::ParametricDeconstruction> {
        match self {
            Deconstruction::Parametric(parametric_deconstruction_b) => Some(*parametric_deconstruction_b),
            _ => None,
        }
    }
    // TODO: This should be derivable via trait macro (also requires a Reconstructable trait).
    pub fn reconstruct(self) -> Result<dy::Value> {
        match self {
            dy::Deconstruction::NonParametric(non_parametric_deconstruction) => Ok(non_parametric_deconstruction.reconstruct()?),
            dy::Deconstruction::Terminal(terminal_deconstruction) => Ok(terminal_deconstruction.reconstruct()?),
            dy::Deconstruction::Parametric(parametric_deconstruction) => Ok(parametric_deconstruction.reconstruct()?),
        }
    }
    // TODO: This should be derivable via trait macro (also requires a Reconstructable trait).
    pub fn reconstructed(&self) -> Result<dy::Value> {
        match self {
            dy::Deconstruction::NonParametric(non_parametric_deconstruction) => Ok(non_parametric_deconstruction.reconstructed()?),
            dy::Deconstruction::Terminal(terminal_deconstruction) => Ok(terminal_deconstruction.reconstructed()?),
            dy::Deconstruction::Parametric(parametric_deconstruction) => Ok(parametric_deconstruction.reconstructed()?),
        }
    }
}

// impl st::Deserializable for Deconstruction {
//     fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
//         // Deserialize the DecostructionKind so that we know which one to deserialize.
//         let deconstruction_kind_code = u8::deserialize(reader)?;
//         let deconstruction_kind = DeconstructionKind::try_from(deconstruction_kind_code)?;
//         let deconstruction = match deconstruction_kind {
//             DeconstructionKind::NonParametric => {
//                 Deconstruction::NonParametric(dy::NonParametricDeconstruction::deserialize(reader)?)
//             }
//             DeconstructionKind::Terminal => {
//                 Deconstruction::Terminal(dy::TerminalDeconstruction::deserialize(reader)?)
//             }
//             DeconstructionKind::Parametric => {
//                 Deconstruction::Parametric(dy::ParametricDeconstruction::deserialize(reader)?)
//             }
//         };
//         Ok(deconstruction)
//     }
// }
//
// impl st::Serializable for Deconstruction {
//     fn serialize_parameters(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         // Serialize the DecostructionKind so that deserialization knows which one to deserialize.
//         let mut bytes_written = (self.kind() as u8).serialize(writer)?;
//         bytes_written += match self {
//             dy::Deconstruction::NonParametric(non_parametric_deconstruction) => non_parametric_deconstruction.serialize()?,
//             dy::Deconstruction::Terminal(terminal_deconstruction) => terminal_deconstruction.serialize()?,
//             dy::Deconstruction::Parametric(parametric_deconstruction) => parametric_deconstruction.serialize()?,
//         };
//         Ok(bytes_written)
//     }
// }
