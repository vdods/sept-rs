use crate::{dy, Result, st::{self, Inhabits, Stringify, TermTrait}};

// TODO: Consider making a type alias for Utf8StringTerm.

impl dy::Deconstruct for String {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Utf8String.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        ).into()
    }
}

impl Inhabits<st::Utf8String> for String {
    fn inhabits(&self, _rhs: &st::Utf8String) -> bool {
        true
    }
}

impl dy::IntoValue for String {}

impl st::Serializable for String {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where a string smaller than 8 bytes is encoded in exactly 8 bytes.
        let mut bytes_written = (self.len() as u64).serialize(writer)?;
        writer.write_all(self.as_bytes())?;
        bytes_written += self.len();
        Ok(bytes_written)
    }
}

impl Stringify for String {
    fn stringify(&self) -> String {
        // Create a quoted string literal.
        format!("{:?}", self)
    }
}

impl TermTrait for String {
    type AbstractTypeType = st::Utf8String;

    fn is_parametric(&self) -> bool {
        true
    }
    fn is_type(&self) -> bool {
        false
    }
    fn abstract_type(&self) -> Self::AbstractTypeType {
        Self::AbstractTypeType{}
    }
}
