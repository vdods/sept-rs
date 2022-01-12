use crate::{dy, st};

/// A Parameterization is a constructor term and a tuple of parameters (aka state variables).
/// There may be constraints on the parameters for the construction to be valid.  E.g. if
/// there were a term of type UnitVector3 (i.e. a 3D vector whose norm is 1), then it would have
/// constructor UnitVector3 and a 3-tuple of coordinates, but those coordinates would be constrained
/// by the unit norm condition.
#[derive(Clone, Debug)]
pub struct Parameterization {
    pub constructor: dy::Value,
    pub parameters: dy::TupleTerm,
}

// This macro_attr! and EnumFromInner! are used to derive From<T> for each inner variant type.
macro_attr! {
    // TODO: Other schemas for deconstruction probably make sense, such as Tabular (a StructTerm
    // would fit this, because its data consists of two columns: "field name" and "field type"), and
    // linked data structures that have nodes on a heap and pointers to the heap (e.g. Tree, DAG)
    #[derive(Clone, Debug, EnumFromInner!)]
    pub enum Deconstruction {
        NonParametric(dy::Value),
        Parametric(Parameterization),
    }
}

impl std::fmt::Display for Deconstruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        use crate::st::Stringify;
        match self {
            Deconstruction::NonParametric(value) => write!(f, "{}", value.stringify())?,
            Deconstruction::Parametric(Parameterization { constructor, parameters }) => {
                write!(f, "{}(", constructor)?;
                for (i, element) in parameters.iter().enumerate() {
                    write!(f, "{}", element)?;
                    if i+1 < parameters.len() {
                        write!(f, ", ")?;
                    }
                }
                write!(f, ")")?;
            }
        }
        Ok(())
    }
}

/// This trait defines deconstruction of a term.
pub trait Deconstruct: st::TermTrait + Clone {
    // TODO: Later specify what the types are (this would actually be st::Deconstruct)
    // TODO: Potentially a term has multiple different constructors

    /// Produce Deconstruction { constructor, parameters }, where constructor is the term that constructed
    /// this term, and parameters is the tuple of parameters canonically representing this term
    /// with respect to that constructor.
    fn deconstruct_into(self) -> Deconstruction;
    fn deconstruct(&self) -> Deconstruction {
        self.clone().deconstruct_into()
    }
}
