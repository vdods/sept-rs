use crate::{dy, st};

/// This trait defines deconstruction of a term.
pub trait Deconstruct: st::TermTrait + Clone {
    // TODO: Later specify what the types are (this would actually be st::Deconstruct)
    // TODO: Potentially a term has multiple different constructors

    /// Produce a Deconstruction, which is either a NonParametricDeconstruction (i.e. a non-parametric
    /// term; aka a terminal) or a ParametricDeconstruction (which has a constructor and a vector of
    /// parameters, each of which are deconstructed; see ParametricDeconstruction::new_recursive).
    fn deconstruct(self) -> dy::Deconstruction;
    /// Fallback impl of deconstructed which clones self instead of leveraging references.
    fn deconstructed(&self) -> dy::Deconstruction {
        self.clone().deconstruct()
    }
    /// Canonical textification of a Deconstruct-ible term.  This can be delegated to inside
    /// of an impl of std::fmt::Display.
    fn textify(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // Ideally this would use a kind of visitor pattern instead of actually creating a new data structure.
        Ok(textify_impl(&self.deconstructed(), f)?)
    }
    /// Convenience method for producing a string via textification.
    fn textified(&self) -> String {
        Textifier(self).to_string()
    }
}

/// This is used as a semantic marker in order to print a value using "full textification", i.e.
/// canonical text rendering of a sept term.  E.g. `format!("{}", Textifier::from(&term))`
#[derive(derive_more::From)]
pub struct Textifier<'a, T: Deconstruct>(&'a T);

impl<'a, T: Deconstruct> std::fmt::Display for Textifier<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(self.0.textify(f)?)
    }
}

fn textify_impl(
    deconstruction: &dy::Deconstruction,
    f: &mut std::fmt::Formatter<'_>,
) -> std::result::Result<(), std::fmt::Error> {
    match deconstruction {
        dy::Deconstruction::NonParametric(non_parametric_deconstruction) => {
            // TODO: More-efficient implementation; the runtime should have a formatter-style Display
            // method for non-parametric terms.
            use crate::st::Stringify;
            write!(f, "{}", non_parametric_deconstruction.as_ref().stringify())?
        }
        dy::Deconstruction::Parametric(parametric_deconstruction) => {
            textify_impl(&parametric_deconstruction.constructor_d, f)?;
            write!(f, "(")?;
            for (i, parameter_d) in parametric_deconstruction.parameter_dv.iter().enumerate() {
                textify_impl(&parameter_d, f)?;
                if i+1 < parametric_deconstruction.parameter_dv.len() {
                    write!(f, ", ")?;
                }
            }
            write!(f, ")")?;
        }
    }
    Ok(())
}
