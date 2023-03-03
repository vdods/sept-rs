use crate::st;

pub trait TestValues: st::TermTrait {
    /// This returns a vector of test values which should include all (within reason) relevant
    /// corner case values.  These comprise the minimal set of test values.
    fn fixed_test_values() -> Vec<Self>;
    // TODO: a random test value generator
}

impl<N: st::NonParametricTermTrait> TestValues for N {
    fn fixed_test_values() -> Vec<Self> {
        vec![N::instantiate()]
    }
}

// TODO: impl for PODTermTrait (requires that trait)
