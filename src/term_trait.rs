use crate::TypeTrait;
use std::{any::Any, fmt::Debug};

pub trait TermTrait: Any + Debug + Sized {
    /// Defines if this term (which means an instance of the Rust type implementing this trait) has
    /// any parameters (i.e. "state variables").  If not, then this term is, by definition, a singleton.
    fn is_parametric_term(&self) -> bool;
    /// Defines if this term is a type, i.e. [potentially] has inhabitants.  An example of
    /// a type that has no inhabitants is EmptyType.
    fn is_type_term(&self) -> bool;
    /// Convenience method for checking type inhabitation.
    fn inhabits(&self, t: &impl TypeTrait) -> bool {
        t.has_inhabitant(self)
    }
}

// TODO: Implement derive macro for deriving TermTrait
