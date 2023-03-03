use std::{any::Any, fmt::Debug};

// TODO: Figure out if it's possible to require trait Inhabits<Self::AbstractTypeType>
pub trait TermTrait: Any + Clone + Send + Sync + Debug + Sized {
    /// This defines the return type of `fn abstract_type(&self)`.  Pardon the awkward naming.
    // TODO: This could be renamed to ConstructorRepr or something.
    // TODO: Maybe the bound on this could be `where Self: Inhabits<Self::AbstractTypeType>`
    type AbstractTypeType: TermTrait;

    /// Non-parametric label for this kind of term.  If this is a parametric term, then the parameters
    /// should be represented by `...` or something.  For example, a term of type Array should have a
    /// label of "Array(...)".  This is meant for debugging purposes.
    // NOTE: &'static str may change later.
    // TODO: Maybe call this concrete_type_name
    fn label() -> &'static str {
        std::any::type_name::<Self>()
    }
    /// Defines if this term (which means an instance of the Rust type implementing this trait) has
    /// any parameters (i.e. "state variables").  If not, then this term is, by definition, a singleton.
    // TODO: Does this really need &self?  And if not, can this just be a const?  This may require
    // &self because e.g. Tuple() is a non-parametric type but Tuple(Float32) isn't.
    // TODO: Make dy::ParametricityTrait with this method, and then have st::NonParametricTermTrait
    // and st::ParametricTermTrait which both implement that.
    // TODO: Consider renaming this to is_parametric_term.
    fn is_parametric(&self) -> bool;
    /// Defines if this term is a type, i.e. [potentially] has inhabitants.  An example of
    /// a type that has no inhabitants is EmptyType.
    // TODO: Does this really need &self?  And if not, can this just be a const?
    // TODO: Make dy::TypefulnessTrait with this method, and then have st::NonTypeTrait
    // and st::TypeTrait which both implement that.
    fn is_type(&self) -> bool;
    /// Defines "the" abstract type of this term.  In particular, the abstract type is not necessarily
    /// the representational type (for example, the abstract type of an instance of the Rust object
    /// sept::ArrayTerm is NOT "ArrayTerm", but rather Array.
    // TODO: This could be renamed to constructor, and it could be split out into a Constructible trait.
    // TODO: Maybe also make a `fn serialize_constructor`
    fn abstract_type(&self) -> Self::AbstractTypeType;
}

// TODO: Implement derive macro for deriving TermTrait
