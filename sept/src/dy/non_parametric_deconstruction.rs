use crate::{dy, st, Error, Result};

/// A NonParametricDeconstruction represents the base case of the inductive structure that Deconstruction has.
/// In particular, it represents the instantiation of a non-parametric term.
#[derive(derive_more::AsRef, Clone, Debug, derive_more::From, derive_more::Into, PartialEq)]
pub struct NonParametricDeconstruction(st::NonParametricTermCode);

/// It's possible to pass in a value that's not a non-parametric term, which makes for
/// an ill-formed NonParametricDeconstruction.
impl TryFrom<dy::Value> for NonParametricDeconstruction {
    type Error = Error;
    fn try_from(value: dy::Value) -> std::result::Result<Self, Self::Error> {
        Ok(Self(
            dy::RUNTIME_LA
                .read()
                .unwrap()
                .non_parametric_term_code(value.as_ref())?,
        ))
        //         // This check is not really well-defined.  Maybe this NonParametricDeconstruction should
        //         // really be called TerminalDeconstruction
        // //         anyhow::ensure!(dy::RUNTIME_LA.read().unwrap().is_non_parametric_term(value.as_ref()), "can't create NonParametricDeconstruction from a Value (which was {:?}) which is not a NonParametricTerm", value);
        //         Ok(Self(value))
    }
}

impl NonParametricDeconstruction {
    /// It's possible to pass in a value that's not a non-parametric term, which makes for an ill-formed
    /// NonParametricDeconstruction.  Equivalent to NonParametricDeconstruction::try_from(value).
    pub fn new(value: dy::Value) -> Result<Self> {
        Ok(Self::try_from(value)?)
    }
    //     /// You, human, must guarantee that the value is a non-parametric term.
    //     pub fn new_unchecked(value: dy::Value) -> Self {
    //         Self(value)
    //     }
    //     pub fn into_inner(self) -> dy::Value {
    pub fn into_inner(self) -> st::NonParametricTermCode {
        self.0
    }
    pub fn reconstruct(self) -> Result<dy::Value> {
        //         Ok(self.0)
        Ok(dy::RUNTIME_LA
            .read()
            .unwrap()
            .non_parametric_term_from_code(self.0)?)
    }
    pub fn reconstructed(&self) -> Result<dy::Value> {
        //         Ok(self.0)
        Ok(dy::RUNTIME_LA
            .read()
            .unwrap()
            .non_parametric_term_from_code(self.0)?)
    }
}

// impl st::Deserializable for NonParametricDeconstruction {
//     fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
//
//     }
// }
//
// impl st::Serializable for NonParametricDeconstruction {
//     fn serialize_parameters(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(dy::RUNTIME_LA.read().unwrap().serialize(self.as_ref(), writer)?)
//     }
// }
