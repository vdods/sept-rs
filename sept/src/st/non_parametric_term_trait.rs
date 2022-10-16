use crate::{dy, Result, st};

/// A NonParametricTermTrait (NonParametricTermTrait) is one that has no "state", i.e. each NonParametricTermTrait is a singleton.
// TODO: Create a macro to derive this
pub trait NonParametricTermTrait: st::TermTrait + dy::IntoValue + Clone + Copy {
    /// This should provide the name of this term.
    // TODO: Might need to worry about namespacing later.  For now, this is considered a kind of keyword.
    fn identifier() -> &'static str;
    /// Instantiate this term.  By construction, no parameters are needed.
    fn instantiate() -> Self;
    /// Retrieve the runtime-valued (i.e. dynamically-valued) form of this NonParametricTerm.
    // TODO: Are const functions possible?  Or are const values in a trait possible?
    fn as_non_parametric_term_code() -> st::NonParametricTermCode;
}

impl<N: NonParametricTermTrait> dy::Deconstruct for N {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: Consider making this take self.as_non_parametric_term_code instead.
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl<N: NonParametricTermTrait> st::Serializable for N {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        Ok((N::as_non_parametric_term_code() as u8).serialize(writer)?)
    }
}
