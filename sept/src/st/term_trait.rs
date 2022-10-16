use std::{any::Any, fmt::Debug};

// TODO: Figure out if it's possible to require trait Inhabits<Self::AbstractTypeType>
pub trait TermTrait: Any + Clone + Send + Sync + Debug + Sized {
    /// This defines the return type of `fn abstract_type(&self)`.  Pardon the awkward naming.
    type AbstractTypeType: TermTrait;

    /// Non-parametric label for this kind of term.  If this is a parametric term, then the parameters
    /// should be represented by `...` or something.  For example, a term of type Array should have a
    /// label of "Array(...)"
    // NOTE: &'static str may change later.
    // TODO: Maybe call this concrete_type_name
    fn label() -> &'static str {
        std::any::type_name::<Self>()
    }
    /// Defines if this term (which means an instance of the Rust type implementing this trait) has
    /// any parameters (i.e. "state variables").  If not, then this term is, by definition, a singleton.
    // TODO: Does this really need &self?  And if not, can this just be a const?  This may require
    // &self because e.g. Tuple() is a non-parametric type but Tuple(Float32) isn't.
    fn is_parametric(&self) -> bool;
    /// Defines if this term is a type, i.e. [potentially] has inhabitants.  An example of
    /// a type that has no inhabitants is EmptyType.
    // TODO: Does this really need &self?  And if not, can this just be a const?
    fn is_type(&self) -> bool;
    /// Defines "the" abstract type of this term.  In particular, the abstract type is not necessarily
    /// the representational type (for example, the abstract type of an instance of the Rust object
    /// sept::ArrayTerm is NOT "ArrayTerm", but rather Array.
    fn abstract_type(&self) -> Self::AbstractTypeType;
}

// TODO: Implement derive macro for deriving TermTrait
