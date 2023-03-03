use crate::Result;

pub trait Deserializable {
    // TODO: Is it necessary to pass in the constructor, e.g. for terms whose types are parametric
    // such as StructTermTerm?  No, that would be a weak trait.  Instead, those should be deserialized
    // via Constructor::deserialize_parameters_and_construct.

    /// Produce an instance of Self by deserializing the serialized parameters.  This obviously
    /// requires knowing Self.
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> where Self: Sized;
//     /// Deserializes an instance of the implementing type from the reader.
//     // TODO: Consider adding serialization parameters such as endianness.  Could also manage
//     // type context projection this way.
//     fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> where Self: Sized;
}

/// Helper function for deserializing arrays of things.
pub fn read_len(reader: &mut dyn std::io::Read) -> Result<usize> {
    let len = u64::deserialize(reader)?;
    anyhow::ensure!(
        len <= usize::MAX as u64,
        "attempting to read a len value (which is {}) that exceeds usize::MAX (which is {})",
        len,
        usize::MAX,
    );
    Ok(len as usize)
}
