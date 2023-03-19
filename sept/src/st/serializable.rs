use crate::{Error, Result};

// // TODO: Eventually generalize this to specify the context to project out/embed in during serialization.
// #[derive(Clone, Copy, Debug, PartialEq)]
// pub enum TypeInfo {
//     Include,
//     Exclude,
// }
//
// impl TypeInfo {
// //     pub fn read_upon_include(
// //         &self,
// //         reader: &mut dyn std::io::Read,
// //     ) -> Result<Option<st::NonParametricTermCode>> {
// //         match self {
// //             TypeInfo::Include => Ok(Some(NonParametricTermCode::read(reader)?)),
// //             TypeInfo::Exclude => Ok(None),
// //         }
// //     }
//     pub fn write_upon_include(
//         &self,
//         writer: &mut dyn std::io::Write,
//         non_parametric_term_code: st::NonParametricTermCode,
//     ) -> Result<usize> {
//         match self {
//             TypeInfo::Include => Ok(non_parametric_term_code.write(writer)?),
//             TypeInfo::Exclude => Ok(0),
//         }
//     }
// }

// TODO: This belongs in dy/value.rs
#[derive(Clone, Copy, Debug, int_enum::IntEnum)]
#[repr(u8)]
pub enum SerializedTopLevelCode {
    Construction = 0x00,
    NonParametric = 0x01,
}

impl TryFrom<u8> for SerializedTopLevelCode {
    type Error = Error;
    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        use int_enum::IntEnum;
        Ok(Self::from_int(value)?)
    }
}

impl SerializedTopLevelCode {
    pub fn read(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<u8>()];
        reader.read_exact(&mut buffer)?;
        let n = u8::from_le_bytes(buffer);
        Ok(Self::try_from(n)?)
    }
    pub fn write(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        writer.write_all((*self as u8).to_le_bytes().as_slice())?;
        Ok(std::mem::size_of::<u8>())
    }
}

// TODO: Consider refactoring this to operate with deconstruction (and construction on the deserialize side).
pub trait Serializable {
    //     /// This serializes the top level code, the constructor, and the parameters, which is a
    //     /// construction expression, and is basically equivalent to a Deconstruction.
    //     // TODO: This really means to serialize a Value holding an instance of Self.
    //     fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         let mut bytes_written = self.serialize_constructor(writer)?;
    //         bytes_written += self.serialize_parameters(writer)?;
    //         Ok(bytes_written)
    //     }
    //     /// Writes the u8 value of the appropriate SerializedTopLevelCode into writer.  Returns the
    //     /// number of bytes written, which is always 1.
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize>;
    //     /// This serializes the constructor of self to the given writer, which should be sufficient to
    //     /// deserialize the serialized parameters.  Returns the number of bytes written.
    //     // TODO: This could come from something like Constructible, which would specify the constructor
    //     // term, and then Value would handle serializing the constructor.
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize>;
    /// This serializes only the value of self to the given writer, knowing what the Self type is.
    /// In order to understand the serialized output, it's necessary to know the constructor or
    /// the Self type.  Returns the number of bytes written.
    // TODO: Consider adding serialization parameters such as endianness.  Could also manage
    // type context projection this way.
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize>;
}

// TODO: Consider renaming this to write_len_as_u64 to be very explicit.
pub fn write_len(len: usize, writer: &mut dyn std::io::Write) -> Result<usize> {
    writer.write_all((len as u64).to_le_bytes().as_slice())?;
    Ok(std::mem::size_of::<u64>())
}
