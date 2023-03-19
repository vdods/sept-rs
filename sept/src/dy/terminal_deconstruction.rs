use crate::{dy, Error, Result};

/// A TerminalDeconstruction represents one of the two base cases of the inductive structure that
/// Deconstruction has.  In particular, it represents a parametric value from a finite set of types
/// supported as terminals (e.g. f32, f64, i8, u8, etc).  The types that are supported as terminals
/// are registered through dy::Runtime.
#[derive(derive_more::AsRef, Clone, Debug, derive_more::Into, PartialEq)]
pub struct TerminalDeconstruction(dy::Value);

/// It's possible to pass in a value that's not a non-parametric term, which makes for
/// an ill-formed TerminalDeconstruction.
impl TryFrom<dy::Value> for TerminalDeconstruction {
    type Error = Error;
    fn try_from(value: dy::Value) -> std::result::Result<Self, Self::Error> {
        // TODO: Check that this is actually a terminal type.
        //         anyhow::ensure!(dy::RUNTIME_LA.read().unwrap().is_terminal(value.as_ref()), "can't create TerminalDeconstruction from a Value (which was {:?}) which is not a terminal", value);
        Ok(Self(value))
    }
}

impl TerminalDeconstruction {
    /// It's possible to pass in a value that's not a terminal, which makes for an ill-formed
    /// TerminalDeconstruction.  Equivalent to TerminalDeconstruction::try_from(value).
    pub fn new(value: dy::Value) -> Result<Self> {
        Ok(Self::try_from(value)?)
    }
    /// You, human, must guarantee that the value is a terminal.
    pub fn new_unchecked(value: dy::Value) -> Self {
        Self(value)
    }
    pub fn into_inner(self) -> dy::Value {
        self.0
    }
    pub fn reconstruct(self) -> Result<dy::Value> {
        Ok(self.0)
    }
    pub fn reconstructed(&self) -> Result<dy::Value> {
        Ok(self.0.clone())
    }
}

// impl st::Serializable for TerminalDeconstruction {
//     fn serialize_parameters(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(dy::RUNTIME_LA.read().unwrap().serialize(self.as_ref(), writer)?)
//     }
// }
