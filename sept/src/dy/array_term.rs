use crate::{dy, Result, st::{self, Array, Inhabits, Stringifiable}};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
#[derive(Clone, Debug, derive_more::Deref, derive_more::DerefMut, derive_more::From, derive_more::Into, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Array", is_parametric = "true", is_type = "true")]
pub struct ArrayTerm(Vec<dy::Value>);

impl dy::Deconstruct for ArrayTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new_recursive(Array.into(), self.0.into()).into()
    }
}

impl std::fmt::Display for ArrayTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl Inhabits<Array> for ArrayTerm {
    fn inhabits(&self, _: &Array) -> bool {
        true
    }
}

impl st::Deserializable for ArrayTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let len = st::read_len(reader)?;
        let mut element_v = Vec::with_capacity(len);
        for _ in 0..len {
            element_v.push(dy::Value::deserialize(reader)?);
        }
        Ok(ArrayTerm(element_v))
    }
}

impl st::Serializable for ArrayTerm {
//     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
//     }
//     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
//         Ok(st::Array.serialize(writer)?)
//     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where an ArrayTerm smaller than 8 bytes is encoded in exactly 8 bytes.
        let mut bytes_written = st::write_len(self.len(), writer)?;
        for element in self.iter() {
            bytes_written += element.serialize(writer)?;
        }
        Ok(bytes_written)
    }
}

impl Stringifiable for ArrayTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Array(");
        for (i, element) in self.0.iter().enumerate() {
            s.push_str(&element.stringify());
            if i+1 < self.0.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl st::TestValues for ArrayTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec![
            ArrayTerm::from(vec![]),
            ArrayTerm::from(vec![4u32.into()]),
            ArrayTerm::from(vec![4u32.into(), 5.0f64.into()]),
            ArrayTerm::from(vec![4u32.into(), 5.0f64.into(), true.into()]),
            // Some nested ones for good measure
            ArrayTerm::from(vec![
                ArrayTerm::from(vec![4u32.into(), 5.0f64.into(), true.into()]).into(),
                ArrayTerm::from(vec![6u32.into(), 7.5f64.into(), false.into()]).into(),
                st::Void.into(),
            ]),
        ]
    }
}
