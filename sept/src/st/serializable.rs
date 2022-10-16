use crate::Result;

pub trait Serializable {
    // TODO: Consider adding serialization parameters such as endianness.  Could also manage
    // type context projection this way.
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize>;
}
