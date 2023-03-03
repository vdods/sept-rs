use crate::{
    dy,
    st::{self, Inhabits, Stringifiable, TermTrait},
    Result,
};

pub type Utf8StringTerm = String;

impl dy::Deconstruct for String {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::Utf8String.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl Inhabits<st::Utf8String> for String {
    fn inhabits(&self, _rhs: &st::Utf8String) -> bool {
        true
    }
}

impl dy::IntoValue for String {}

impl st::Deserializable for String {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let len = st::read_len(reader)?;
        let mut string = String::with_capacity(len);
        use std::io::Read;
        let bytes_read = reader.take(len as u64).read_to_string(&mut string)?;
        anyhow::ensure!(
            bytes_read == len,
            "EOF encountered in deserialize before expected end of String"
        );
        Ok(string)
    }
}

impl st::Serializable for String {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Utf8String.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where a string smaller than 8 bytes is encoded in exactly 8 bytes.
        let mut bytes_written = st::write_len(self.len(), writer)?;
        // TODO: Probably make a utility function for writing a byte array.
        writer.write_all(self.as_bytes())?;
        bytes_written += self.len();
        Ok(bytes_written)
    }
}

// impl st::Deserializable for String {
//     fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
//         let len = u64::deserialize(reader)?;
//         anyhow::ensure!(
//             len <= usize::MAX as u64,
//             "attempting to deserialize String with a len (which is {}) that exceeds usize::MAX (which is {})",
//             len,
//             usize::MAX,
//         );
//         let mut string = String::new();
//         use std::io::Read;
//         let bytes_read = reader.take(len).read_to_string(&mut string)?;
//         anyhow::ensure!(bytes_read as u64 == len, "EOF encountered in deserialize before expected end of String");
//         Ok(string)
//     }
// }
//
// impl st::Serializable for String {
//     fn serialize_parameters(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
//         // like where a string smaller than 8 bytes is encoded in exactly 8 bytes.
//         let mut bytes_written = (self.len() as u64).serialize(writer)?;
//         writer.write_all(self.as_bytes())?;
//         bytes_written += self.len();
//         Ok(bytes_written)
//     }
// }
//
impl Stringifiable for String {
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
        Self::AbstractTypeType {}
    }
}

impl st::TestValues for String {
    fn fixed_test_values() -> Vec<Self> {
        vec!["", "a", "abc", "\n", "\t", "日本"]
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}

// TODO: Move these into sept_tests
#[cfg(test)]
mod tests {
    fn test_serialize_deserialize_case(string: String, expected_bytes_written: usize) {
        let mut buffer = Vec::new();
        use crate::st::Serializable;
        let bytes_written = string.serialize(&mut buffer).expect("pass");
        assert_eq!(bytes_written, expected_bytes_written);
        use crate::st::Deserializable;
        // `buffer.as_slice()` is the content, and you have to take a mut ref to it to get a reader.
        // The content the slice is pointing to doesn't change, but the slice start does change.
        let reader: &mut dyn std::io::Read = &mut buffer.as_slice();
        let deserialized_string = String::deserialize(reader).expect("pass");
        assert_eq!(deserialized_string, string);
    }

    #[test]
    fn test_serialize_deserialize() {
        const SIZE_OF_U64: usize = 8;
        test_serialize_deserialize_case("".to_string(), SIZE_OF_U64 + 0);
        test_serialize_deserialize_case("a".to_string(), SIZE_OF_U64 + 1);
        test_serialize_deserialize_case("\n".to_string(), SIZE_OF_U64 + 1);
        test_serialize_deserialize_case(" ".to_string(), SIZE_OF_U64 + 1);
        test_serialize_deserialize_case("ø".to_string(), SIZE_OF_U64 + 2);
        test_serialize_deserialize_case("blah blah blah".to_string(), SIZE_OF_U64 + 14);
    }
}
