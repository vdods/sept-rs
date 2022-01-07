use crate::{dy::DynNPTerm, st::TermTrait};

/// A NonParametricTermTrait (NonParametricTermTrait) is one that has no "state", i.e. each NonParametricTermTrait is a singleton.
// TODO: Create a macro to derive this
pub trait NonParametricTermTrait: TermTrait + Clone + Copy {
    /// Retrieve the runtime-valued (i.e. dynamically-valued) form of this NonParametricTerm.
    fn as_dyn_npterm(&self) -> DynNPTerm;
}
