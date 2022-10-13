use crate::{dy, Result, st};

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
    // TODO: Should this actually take std::io::Write and then use `write!`?
    fn textify(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // Ideally this would use a kind of visitor pattern instead of actually creating a new data structure.
        Ok(textify_impl(&self.deconstructed(), f)?)
    }
    /// Convenience method for producing a String via textification.
    fn textified(&self) -> String {
        Textifier(self).to_string()
    }
    /// Canonical serialization of a Deconstruct-ible term, where all type information is encoded
    /// in the serialization -- i.e. it's a fully-typed, dynamic representation.
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<()> {
        Ok(serialize_impl(&self.deconstructed(), writer)?)
    }
    /// Convenience method for producing a Vec<u8> via serialization.
    fn serialized(&self, starting_capacity_o: Option<usize>) -> Result<Vec<u8>> {
        let mut buffer = if let Some(starting_capacity) = starting_capacity_o {
            Vec::with_capacity(starting_capacity)
        } else {
            Vec::new()
        };
        self.serialize(&mut buffer)?;
        Ok(buffer)
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

// TODO: Formalize this
enum SynCode {
    /// Analogous to parser::Terminal
    NonParametricTerm = 0,
    /// Analogous to scanner::Token::OpenParen
    ParametersBegin,
    /// Analogous to scanner::Token::CloseParen
    ParametersEnd,
    /// Analogous to scanner::Token::Comma
    ParameterSeparator,
}

fn serialize_impl(
    deconstruction: &dy::Deconstruction,
    writer: &mut dyn std::io::Write,
) -> Result<()> {
    match deconstruction {
        dy::Deconstruction::NonParametric(non_parametric_deconstruction) => {
            // We're about to write a NonParametricTerm, so write the SynCode for that.
            writer.write_all(&[SynCode::NonParametricTerm as u8])?;
            // Retrieve the code for this non parametric term and serialize that.
            log::debug!("serialize_impl; non_parametric_deconstruction: {:?}", non_parametric_deconstruction);
            // The `.as_ref().as_ref()` is correct; the first one gets &Value, the second one
            // gets &ValueGuts.
            let non_parametric_term_code =
                dy::RUNTIME_LA
                    .read()
                    .unwrap()
                    .non_parametric_term_code(non_parametric_deconstruction.as_ref().as_ref())?;
            assert!((non_parametric_term_code as u32) < 0x100u32, "NonParametricTermCode exceeds 1-byte storage capacity");
            writer.write_all(&[non_parametric_term_code as u8])?;
        }
        dy::Deconstruction::Parametric(parametric_deconstruction) => {
            log::debug!("serialize_impl; parametric_deconstruction: {:?}", parametric_deconstruction);
            serialize_impl(&parametric_deconstruction.constructor_d, writer)?;

            // TODO: Need "syntactical marker codes" analogous to the parens and comma in textify.
            // Note though that this won't give a minimal serialization where type information is
            // known in the context of the deserializer.  That will need another approach -- in
            // particular, using the context-projection/embedding method.

            // We're about to write the parameters, so write the SynCode for that.
            writer.write_all(&[SynCode::ParametersBegin as u8])?;
            // Write the params.
            for (i, parameter_d) in parametric_deconstruction.parameter_dv.iter().enumerate() {
                serialize_impl(&parameter_d, writer)?;
                if i+1 < parametric_deconstruction.parameter_dv.len() {
                    // To separate parameters, write the SynCode for that.
                    writer.write_all(&[SynCode::ParameterSeparator as u8])?;
                }
            }
            // We're done writing the parameters, so write the SynCode for that.
            writer.write_all(&[SynCode::ParametersEnd as u8])?;
        }
    }
    Ok(())
}
