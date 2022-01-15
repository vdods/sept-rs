use crate::{dy, Result};

/// A ParametricDeconstruction is a (deconstructed) constructor term and a vector of (deconstructed)
/// parameters (aka state variables).  There may be constraints on the parameters for the construction
/// to be valid.  E.g. if there were a term of type UnitVector3 (i.e. a 3D vector whose norm is 1),
/// then it would have constructor UnitVector3 and a 3-tuple of coordinates, but those coordinates
/// would be constrained by the unit norm condition.  Note that for now, this type is not a first class
/// term in the sept runtime, because it's only meant to facilitate render/parse and de/serialization.
#[derive(Clone, Debug, PartialEq)]
pub struct ParametricDeconstruction {
    /// This is the deconstruction of the constructor term.
    pub constructor_d: dy::Deconstruction,
    /// This is the sequence of deconstructed parameter terms.
    pub parameter_dv: Vec<dy::Deconstruction>,
}

impl ParametricDeconstruction {
    /// Plain constructor, doesn't call deconstruct on anything.
    pub fn new(constructor_d: dy::Deconstruction, parameter_dv: Vec<dy::Deconstruction>) -> Self {
        Self { constructor_d, parameter_dv }
    }
    /// This will deconstruct constructor and parameters, producing a fully deconstructed value.
    pub fn new_recursive(constructor: dy::Value, parameter_t: dy::TupleTerm) -> Self {
        use crate::dy::Deconstruct;
        Self {
            constructor_d: constructor.deconstruct(),
            parameter_dv: parameter_t.into_inner().into_iter().map(|parameter| parameter.deconstruct()).collect(),
        }
    }
    /// Recurse and call reconstruct on constructor_d and parameter_dv, then perform the construction.
    pub fn reconstruct(self) -> Result<dy::Value> {
        use crate::dy::Constructor;
        let constructor = self.constructor_d.reconstruct()?;
        let mut parameter_v = Vec::with_capacity(self.parameter_dv.len());
        for parameter_d in self.parameter_dv.into_iter() {
            parameter_v.push(parameter_d.reconstruct()?);
        }
        let parameter_t = dy::TupleTerm::from(parameter_v);
        Ok(constructor.construct(parameter_t)?)
    }
}
