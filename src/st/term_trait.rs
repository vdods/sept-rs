use std::{any::Any, fmt::Debug};

// TODO: Figure out if it's possible to require trait Inhabits<Self::AbstractTypeFnReturnType>
pub trait TermTrait: Any + Send + Sync + Debug + Sized {
    type AbstractTypeFnReturnType: TermTrait;

    /// Non-parametric label for this kind of term.  If this is a parametric term, then the parameters
    /// should be represented by `...` or something.  For example, a term of type Array should have a
    /// label of "Array(...)"
    // NOTE: &'static str may change later.
    fn label() -> &'static str;
    /// Defines if this term (which means an instance of the Rust type implementing this trait) has
    /// any parameters (i.e. "state variables").  If not, then this term is, by definition, a singleton.
    fn is_parametric_term(&self) -> bool;
    /// Defines if this term is a type, i.e. [potentially] has inhabitants.  An example of
    /// a type that has no inhabitants is EmptyType.
    fn is_type_term(&self) -> bool;
    /// Defines "the" abstract type of this term.  In particular, the abstract type is not necessarily
    /// the representational type (for example, the abstract type of an instance of the Rust object
    /// sept::ArrayTerm is NOT "ArrayTerm", but rather Array.  TODO: Not sure if Box<dyn Any> is really
    /// the correct return type here.
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType;
}

// TODO: Implement derive macro for deriving TermTrait
