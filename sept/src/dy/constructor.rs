use crate::{dy, st, Result};

/// This trait defines a constructor.  In particular, a constructor is a term that transforms
/// a tuple of parameters into a term potentially of another type.  E.g. `Array` is a constructor,
/// taking a tuple of parameters and producing an instance of `ArrayTerm`.
// NOTE: For now, it's assumed that different terms will be used to provide different constructors.
// NOTE: This corresponds to dy::Parameterization, and if there are other variants added, such
// as Tabular, then they would need some sort of analog to Constructor.  Maybe there would
// be ParameterizedConstructor and TabularConstructor, etc, and maybe even NonParametricConstructor,
// which would necessarily return itself.
pub trait Constructor: st::TermTrait {
    type ConstructedType: st::TermTrait + Into<dy::Value>;

    /// Perform the construction using the given tuple of parameters.
    // TODO: Consider adding a "origin_constructor" parameter, in particular to allow for a global/local
    // symref to be passed in, instead of the dereferenced valued implicitly taken to be self.  This only
    // matters in some situations.
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType>;
}
