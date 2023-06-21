use crate::{
    dy, qv,
    st::{self, Inhabits, Stringifiable, TermTrait},
    Error, Result,
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

// TODO: Maybe move this elsewhere so as not to clog up this file
pub fn replace_single_char_in_string(
    s: &mut String,
    char_index: usize,
    expected_existing_char_o: Option<char>,
    replacement: &str,
) -> Result<()> {
    // Find the slice index range that this char occupies
    let mut char_indices = s.char_indices().skip(char_index);
    let char_index_start = match char_indices.next() {
        Some((char_byte_index, existing_char)) => {
            if let Some(expected_existing_char) = expected_existing_char_o {
                anyhow::ensure!(
                    existing_char == expected_existing_char,
                    "replacement encountered different existing char ({:?}) than expected ({:?})",
                    existing_char,
                    expected_existing_char
                );
            }
            char_byte_index
        }
        None => s.len(),
    };
    let char_index_end = match char_indices.next() {
        Some((char_byte_index, _)) => char_byte_index,
        None => s.len(),
    };
    // Now replace that range with the given char.
    s.replace_range(char_index_start..char_index_end, replacement);
    Ok(())
}

// TODO: Maybe move this elsewhere so as not to clog up this file
#[allow(unused)]
pub fn replace_substr_in_string(
    s: &mut String,
    substr_char_index_start: usize,
    existing_substr: &str,
    replacement: &str,
) -> Result<()> {
    // Check that the existing_substr actually matches, and get the begin and end byte index values within s.
    let byte_index_start = s
        .char_indices()
        .skip(substr_char_index_start)
        .next()
        // The use of unwrap_or makes substr_char_index_start a bit more forgiving, but also doesn't produce
        // precise errors regarding invalid index values.  'x' is a dummy value that is discarded.
        .unwrap_or((s.len(), 'x'))
        .0;
    let actual_existing_substr_o =
        s.get(byte_index_start..byte_index_start + existing_substr.len());
    anyhow::ensure!(
        actual_existing_substr_o == Some(existing_substr),
        "replacement encountered different existing substr ({:?}) than expected ({:?})",
        actual_existing_substr_o,
        Some(existing_substr)
    );
    let byte_index_end = byte_index_start + existing_substr.len();
    // Do the replacement.
    s.replace_range(byte_index_start..byte_index_end, replacement);
    Ok(())
}

impl qv::ApplyEditTrait for String {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        // TODO: "clear" edit
        if edit.is::<qv::ReplacementTerm>() {
            let edit = edit.downcast_into::<qv::ReplacementTerm>();
            anyhow::ensure!(
                edit.old_data.is::<String>(),
                "Utf8StringTerm ReplacementTerm edit expected old_data to be String"
            );
            anyhow::ensure!(
                edit.new_data.is::<String>(),
                "Utf8StringTerm ReplacementTerm edit expected new_data to be String"
            );
            let old_string = edit.old_data.downcast_into::<String>();
            let new_string = edit.new_data.downcast_into::<String>();
            anyhow::ensure!(*self == old_string, "Utf8StringTerm ReplacementTerm edit expected current value ({:?}) to match old_data ({:?})", self, old_string);
            *self = new_string;
        } else {
            anyhow::bail!("Utf8StringTerm does not support edit: {}", edit);
        }
        Ok(())
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

impl qv::QueryableDynTrait for String {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryTrait + 'a> {
        Box::new(qv::Utf8StringTermView::new(self))
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

impl qv::SingleQuery<dy::Value> for String {
    type ReturnType<'a> = qv::Utf8StringTermQuery<'a>;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(address_string) = address_token.downcast_ref::<String>() {
            match address_string.as_str() {
                "char" => Ok(qv::Utf8StringTermCharView::new(self).into()),
                "line" => Ok(qv::Utf8StringTermLineView::new(self).into()),
                // TODO: "len" perhaps
                _ => {
                    anyhow::bail!(
                        "Utf8StringTerm::run_single_query; unrecognized address_token {:?}",
                        address_string.as_str()
                    );
                }
            }
        } else {
            anyhow::bail!(
                "Utf8StringTerm::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

impl qv::SingleQueryMut<dy::Value> for String {
    type ReturnType<'a> = qv::Utf8StringTermQueryMut<'a>;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(address_string) = address_token.downcast_ref::<String>() {
            match address_string.as_str() {
                "char" => Ok(qv::Utf8StringTermCharMutView::new(self).into()),
                "line" => Ok(qv::Utf8StringTermLineMutView::new(self).into()),
                // TODO: "len" perhaps
                _ => {
                    anyhow::bail!(
                        "Utf8StringTerm::run_single_query_mut; unrecognized address_token {:?}",
                        address_string.as_str()
                    );
                }
            }
        } else {
            anyhow::bail!(
                "Utf8StringTerm::run_single_query_mut; unrecognized address_token {}",
                address_token.stringify()
            );
        }
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

impl TermTrait for &'static str {
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

/// The use of the Either enum is a kludge to get around the bug https://github.com/rust-lang/rust/issues/111457
pub fn split_inclusive_allow_trailing_empty<'a>(
    s: &'a str,
    sep: char,
) -> either::Either<
    std::iter::Chain<std::str::SplitInclusive<'a, char>, std::iter::Once<&'a str>>,
    std::str::SplitInclusive<'a, char>,
> {
    if s.is_empty() || s.ends_with(sep) {
        either::Either::Left(s.split_inclusive(sep).chain(std::iter::once("")))
    } else {
        either::Either::Right(s.split_inclusive(sep))
    }
}
