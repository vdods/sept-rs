use crate::{
    dy,
    st::{self, Inhabits, Stringifiable, TermTrait, UnicodeChar},
    Result,
};

pub type UnicodeCharTerm = char;

impl dy::Deconstruct for UnicodeCharTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::UnicodeChar.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl Inhabits<UnicodeChar> for UnicodeCharTerm {
    fn inhabits(&self, _rhs: &UnicodeChar) -> bool {
        true
    }
}

impl dy::IntoValue for UnicodeCharTerm {}

impl st::Deserializable for UnicodeCharTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let mut buffer = [0u8; std::mem::size_of::<u32>()];
        reader.read_exact(&mut buffer[0..3])?;
        let n = u32::from_le_bytes(buffer);
        Ok(char::from_u32(n).ok_or_else(|| {
            anyhow::anyhow!(
                "UnicodeCharTerm decode error; {} is not a valid code point",
                n
            )
        })?)
    }
}

impl dy::Queryable for char {
    fn query<'a>(&'a self, address_v: &[dy::Value]) -> Result<&'a dy::ValueGuts> {
        if address_v.is_empty() {
            Ok(self)
        } else {
            unimplemented!("TODO: implement views if any.. perhaps 'as int repr'");
        }
    }
    fn query_mut<'a>(&'a mut self, _address_v: &[dy::Value]) -> Result<&'a mut dy::ValueGuts> {
        unimplemented!("blah");
        // TODO: This should basically be the same as query, though maybe non-l-values (e.g. querying
        // `Len`) wouldn't support this.
    }
}

impl dy::QueryableDynTrait for char {
    fn make_query<'a>(&'a self) -> Box<dyn dy::QueryTrait + 'a> {
        dy::GenericView::new(self)
    }
}

impl dy::QueryableMutDynTrait for char {
    fn make_query_mut<'a>(&'a mut self) -> Box<dyn dy::QueryMutTrait + 'a> {
        dy::GenericMutView::new(self)
    }
}

impl st::Serializable for UnicodeCharTerm {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::UnicodeCharTerm.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // By definition (see Rust docs; primitive type `char`):
        // code points [unicode chars] are in the range 0 to 0x10FFFF, inclusive.
        // Thus they fit in 3 bytes.  This serialization will use 3 bytes unconditionally.
        // An alternative would be to use (the implicitly conditional-length) UTF8 encoding, but this
        // would be better done in something like UTF8EncodedChar, which would have the
        // advantage of being exactly overlaid on a UTF8String serialization.
        let n = *self as u32;
        let le_bytes = n.to_le_bytes();
        // Sanity check.
        assert_eq!(le_bytes[3], 0);
        // Write the 3 least-significant bytes in little-endian order.
        writer.write_all(&le_bytes[0..3])?;
        Ok(3)
    }
}

impl Stringifiable for UnicodeCharTerm {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermTrait for UnicodeCharTerm {
    type AbstractTypeType = UnicodeChar;

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

impl st::TestValues for UnicodeCharTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec!['a', ' ', '\n', '日']
    }
}
