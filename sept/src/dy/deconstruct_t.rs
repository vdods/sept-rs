use crate::{dy, st, Result};

/// This trait defines deconstruction of a term.
// TODO: Rename to Deconstructible
pub trait DeconstructT: st::TermT + Clone {
    // TODO: Later specify what the types are (this would actually be st::DeconstructT)
    // TODO: Potentially a term has multiple different constructors

    /// Produce a Deconstruction, which is either a NonParametricDeconstruction (i.e. a non-parametric
    /// term, which is a singleton having a value in NonParametricTermCode), TerminalDeconstruction
    /// (i.e. a value having a type from a fixed collection of "primitive" types) or a
    /// ParametricDeconstruction (which has a constructor and a vector of parameters, each of which
    /// are deconstructed; see ParametricDeconstruction::new_recursive).
    fn deconstruct(self) -> dy::Deconstruction;
    /// Fallback impl of deconstructed which clones self instead of leveraging references.
    fn deconstructed(&self) -> dy::Deconstruction {
        self.clone().deconstruct()
    }
    /// Canonical textification of a DeconstructT term.  This can be delegated to inside
    /// of an impl of std::fmt::Display.
    // TODO: Should this actually take std::io::Write and then use `write!`?
    // TODO: Maybe this should just be a generic function accepting a DeconstructT
    fn textify(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        // Ideally this would use a kind of visitor pattern instead of actually creating a new data structure.
        Ok(textify_impl(&self.deconstructed(), f)?)
    }
    /// Convenience method for producing a String via textification.
    // TODO: Maybe this should just be a generic function accepting a DeconstructT
    fn textified(&self) -> String {
        Textifier(self).to_string()
    }
    //     /// Canonical serialization of a DeconstructT term, where all type information is encoded
    //     /// in the serialization -- i.e. it's a fully-typed, dynamic representation.
    //     // TODO: Maybe this should just be a generic function accepting a DeconstructT
    //     fn serialize_parameters(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(serialize_impl(&self.deconstructed(), writer)?)
    //     }
    //     /// Convenience method for producing a Vec<u8> via serialization.
    //     // TODO: Maybe this should just be a generic function accepting a DeconstructT
    //     fn serialized(&self, starting_capacity_o: Option<usize>) -> Result<Vec<u8>> {
    //         let mut buffer = if let Some(starting_capacity) = starting_capacity_o {
    //             Vec::with_capacity(starting_capacity)
    //         } else {
    //             Vec::new()
    //         };
    //         self.serialize(&mut buffer)?;
    //         Ok(buffer)
    //     }
}

/// This is used as a semantic marker in order to print a value using "full textification", i.e.
/// canonical text rendering of a sept term.  E.g. `format!("{}", Textifier::from(&term))`
#[derive(derive_more::From)]
pub struct Textifier<'a, T: DeconstructT>(&'a T);

impl<'a, T: DeconstructT> std::fmt::Display for Textifier<'a, T> {
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
            use crate::st::StringifiableT;
            write!(
                f,
                "{}",
                non_parametric_deconstruction
                    .reconstructed()
                    .unwrap()
                    .stringify()
            )?
        }
        dy::Deconstruction::Terminal(terminal_deconstruction) => {
            // TODO: More-efficient implementation; the runtime should have a formatter-style Display
            // method for non-parametric terms.
            use crate::st::StringifiableT;
            // TODO: Should this be a more-formal kind of stringification?
            write!(
                f,
                "{}",
                terminal_deconstruction.reconstructed().unwrap().stringify()
            )?
        }
        dy::Deconstruction::Parametric(parametric_deconstruction) => {
            textify_impl(&parametric_deconstruction.constructor_d, f)?;
            write!(f, "(")?;
            for (i, parameter_d) in parametric_deconstruction.parameter_dv.iter().enumerate() {
                textify_impl(&parameter_d, f)?;
                if i + 1 < parametric_deconstruction.parameter_dv.len() {
                    write!(f, ", ")?;
                }
            }
            write!(f, ")")?;
        }
    }
    Ok(())
}

// fn serialize_impl(
//     deconstruction: &dy::Deconstruction,
//     writer: &mut dyn std::io::Write,
// ) -> Result<usize> {
//     match deconstruction {
//         dy::Deconstruction::NonParametric(non_parametric_deconstruction) => {
//             let mut bytes_written = 0usize;
//
// //             // Retrieve the code for this non parametric term and serialize that.
// //             // The `.as_ref().as_ref()` is correct; the first one gets &Value, the second one
// //             // gets &ValueGuts.
// //             let non_parametric_term_code =
// //                 dy::RUNTIME_LA
// //                     .read()
// //                     .unwrap()
// //                     .non_parametric_term_code(non_parametric_deconstruction.as_ref().as_ref())?;
//             let non_parametric_term_code = non_parametric_deconstruction.into_inner();
//             assert!((non_parametric_term_code as u32) < 0x100u32, "NonParametricTermCode exceeds 1-byte storage capacity");
//             writer.write_all(&[non_parametric_term_code as u8])?;
//             bytes_written += 1;
//
//             Ok(bytes_written)
//         }
//         dy::Deconstruction::Terminal(terminal_deconstruction) => {
//             // Each terminal has its own custom serialization behavior, registered in the runtime.
//             // The `.as_ref().as_ref()` is correct; the first one gets &Value, the second one
//             // gets &ValueGuts.
//             Ok(dy::RUNTIME_LA
//                 .read()
//                 .unwrap()
//                 .serialize(terminal_deconstruction.as_ref().as_ref(), writer)?
//             )
//         }
//         dy::Deconstruction::Parametric(parametric_deconstruction) => {
//             let mut bytes_written = 0usize;
//             bytes_written += serialize_impl(&parametric_deconstruction.constructor_d, writer)?;
//             for parameter_d in parametric_deconstruction.parameter_dv.iter() {
//                 bytes_written += serialize_impl(parameter_d, writer)?;
//             }
//             Ok(bytes_written)
//         }
//     }
// }
