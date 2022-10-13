use crate::{dy, st};

/// A NonParametricTermTrait (NonParametricTermTrait) is one that has no "state", i.e. each NonParametricTermTrait is a singleton.
// TODO: Create a macro to derive this
pub trait NonParametricTermTrait: st::TermTrait + dy::IntoValue + Clone + Copy {
    /// This should provide the name of this term.
    // TODO: Might need to worry about namespacing later.  For now, this is considered a kind of keyword.
    fn identifier() -> &'static str;
    /// Instantiate this term.  By construction, no parameters are needed.
    fn instantiate() -> Self;
    /// Retrieve the runtime-valued (i.e. dynamically-valued) form of this NonParametricTerm.
    // TODO: This should not have a &self parameter.
    fn as_non_parametric_term_code() -> dy::NonParametricTermCode;
}
