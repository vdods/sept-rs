use crate::{dy, st};

/// This trait defines a constructor.  In particular, a constructor is a term that transforms
/// a tuple of parameters into a term potentially of another type.  E.g. `Array` is a constructor,
/// taking a tuple of parameters and producing an instance of `ArrayTerm`.
// NOTE: For now, it's assumed that different terms will be used to provide different constructors.
// NOTE: This corresponds to dy::Parameterization, and if there are other variants added, such
// as Tabular, then they would need some sort of analog to Constructor.  Maybe there would
// be ParameterizedConstructor and TabularConstructor, etc, and maybe even NonParametricConstructor,
// which would necessarily return itself.
pub trait Constructor: st::TermTrait {
    type ConstructedType: st::TermTrait;

    /// Perform the construction using the given tuple of parameters.
    fn construct(&self, parameters: dy::TupleTerm) -> anyhow::Result<Self::ConstructedType>;
}
