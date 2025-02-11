use crate::{
    dy, qv,
    st::{self, InhabitsT, StringifiableT, TermT},
    Error, Result,
};

pub type UnicodeCharTerm = char;

impl qv::ApplyEditT for UnicodeCharTerm {
    fn apply_edit(&mut self, edit: dy::Value) -> Result<()> {
        qv::generic_apply_edit(self, edit)
    }
}

impl dy::DeconstructT for UnicodeCharTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        // Deconstruct only the constructor, otherwise infinite recursion!
        dy::ParametricDeconstruction::new(
            st::UnicodeChar.deconstruct(),
            vec![dy::TerminalDeconstruction::new_unchecked(dy::Value::from(self)).into()],
        )
        .into()
    }
}

impl InhabitsT<st::UnicodeChar> for UnicodeCharTerm {
    fn inhabits(&self, _rhs: &st::UnicodeChar) -> bool {
        true
    }
}

impl dy::IntoValueT for UnicodeCharTerm {}

impl st::DeserializableT for UnicodeCharTerm {
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

impl qv::QueryableDynT for UnicodeCharTerm {
    fn make_query<'a>(&'a self) -> Box<dyn qv::QueryT + 'a> {
        Box::new(qv::GenericView::new(self))
    }
}

impl st::SerializableT for UnicodeCharTerm {
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

impl qv::SingleQueryT<dy::Value> for UnicodeCharTerm {
    type ReturnType<'a> = qv::UnicodeCharTermQuery<'a>;
    type Error = Error;
    fn run_single_query<'a>(
        &'a self,
        address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        if let Some(c) = address_token.downcast_ref::<UnicodeCharTerm>() {
            match *c {
                // Plain char view.
                'p' => Ok(qv::UnicodeCharTermPlainView::new(self)?.into()),
                // Single-character escape code view (C for "character").
                'c' => Ok(qv::UnicodeCharTermEscCView::new(self)?.into()),
                // // Hexadecimal escape code view.
                // 'x' => Ok(qv::UnicodeCharTermEscXView::new(self)?.into()),
                // // Unicode escape code view.
                // 'u' => Ok(qv::UnicodeCharTermEscUView::new(self)?.into()),
                _ => {
                    anyhow::bail!(
                        "UnicodeCharTerm::run_single_query; unrecognized address_token {}",
                        address_token.stringify()
                    );
                }
            }
        } else {
            anyhow::bail!(
                "UnicodeCharTerm::run_single_query; unrecognized address_token {}",
                address_token.stringify()
            );
        }
    }
}

impl qv::SingleQueryMutT<dy::Value> for UnicodeCharTerm {
    type ReturnType<'a> = qv::EmptyQuery;
    type Error = Error;
    fn run_single_query_mut<'a>(
        &'a mut self,
        _address_token: &dy::Value,
    ) -> std::result::Result<Self::ReturnType<'a>, Self::Error> {
        anyhow::bail!("UnicodeCharTerm does not support queries at this time");
    }
}

impl StringifiableT for UnicodeCharTerm {
    fn stringify(&self) -> String {
        self.to_string()
    }
}

impl TermT for UnicodeCharTerm {
    type AbstractTypeType = st::UnicodeChar;

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

impl st::TestValuesT for UnicodeCharTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec!['a', ' ', '\n', '\x03', '日']
    }
}
