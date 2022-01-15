use crate::{dy, st};

/// This trait defines deconstruction of a term.
pub trait Deconstruct: st::TermTrait + Clone {
    // TODO: Later specify what the types are (this would actually be st::Deconstruct)
    // TODO: Potentially a term has multiple different constructors

    /// Produce a Deconstruction, which is either a NonParametricDeconstruction (i.e. a non-parametric
    /// term; aka a terminal) or a ParametricDeconstruction (which has a constructor and a vector of
    /// parameters, each of which are deconstructed; see ParametricDeconstruction::new_recursive).
    fn deconstruct(self) -> dy::Deconstruction;
    fn deconstructed(&self) -> dy::Deconstruction {
        self.clone().deconstruct()
    }
}
