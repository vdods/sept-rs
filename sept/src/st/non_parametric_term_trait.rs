use crate::{dy, Result, st};

/// A NonParametricTermTrait (NonParametricTermTrait) is one that has no "state", i.e. each
/// NonParametricTermTrait is a singleton.  It's recommended to derive this trait using
/// derive(st::NonParametricTermTrait).
pub trait NonParametricTermTrait: st::TermTrait + dy::IntoValue + Clone + Copy {
    /// The name of this term.
    // TODO: Might need to worry about namespacing later.  For now, this is considered a kind of keyword.
    const IDENTIFIER: &'static str;
    /// The serialization code for this NonParametricTerm.
    const NON_PARAMETRIC_TERM_CODE: st::NonParametricTermCode;
    /// Instantiate this term.  By construction, no parameters are needed.
    fn instantiate() -> Self;
}

impl<N: NonParametricTermTrait> dy::Deconstruct for N {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: Consider making this take self.as_non_parametric_term_code instead.
        dy::NonParametricDeconstruction::new_unchecked(dy::Value::from(self)).into()
    }
}

impl<N: NonParametricTermTrait> st::Serializable for N {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        Ok((N::NON_PARAMETRIC_TERM_CODE as u8).serialize(writer)?)
    }
}
