use crate::{
    dy,
    st::{self, Inhabits, Stringifiable, Struct, TermTrait},
    Result,
};
use std::collections::HashMap;

// TODO: Theoretically, the key (i.e. name) could be any type, thereby enabling the possibility of structured names.
// But even if this isn't done, then first class sept-enabled strings should be used.
#[derive(Clone, Debug, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Struct", is_parametric = "true", is_type = "true")]
pub struct StructTerm {
    /// This stores the field declarations (i.e. `field: Type`) in a particular order.
    // TODO: Check that each is a type.
    // TODO: Maybe separate this out into name_v (and eventually name_tuple_term) and type_tuple_term,
    // which would simplify various checks and projections into TupleTerm.
    // TODO: Probably eventually allow arbitrary terms as the field names.
    // TODO: Define and use SymbolDecl as a formal type.
    pub field_decl_v: Vec<(String, dy::Value)>,
    /// This is a cache for the quick lookup of the element index based on a field name.
    name_index_m: HashMap<String, usize>,
}

/// TODO: Implement projection to TupleTerm of types.
impl StructTerm {
    pub fn new(field_decl_v: Vec<(String, dy::Value)>) -> Result<Self> {
        // Check that all elements in field_decl_v are actually types.
        for (i, field_decl) in field_decl_v.iter().enumerate() {
            anyhow::ensure!(field_decl.1.inhabits(&st::Type), "expected {}th StructTerm field type (which was {:?}) to inhabit Type, but it did not", i, field_decl.1);
        }
        // Generate name_index_m.
        let name_index_m: HashMap<String, usize> = field_decl_v
            .iter()
            .enumerate()
            .map(|(i, (name, _))| (name.clone(), i))
            .collect();
        Ok(Self {
            field_decl_v,
            name_index_m,
        })
    }
    /// Verifies inhabitation by field_t (which is a kind of untyped StructTermTerm), otherwise
    /// returns an error describing the failure.
    pub(crate) fn verify_inhabitation_by(&self, field_t: &dy::TupleTerm) -> Result<()> {
        anyhow::ensure!(field_t.len() == self.field_decl_v.len(), "mismatch in number of type elements in StructTerm (which was {}) and in field_t (which was {})", self.field_decl_v.len(), field_t.len());
        for (i, datum) in field_t.iter().enumerate() {
            anyhow::ensure!(datum.inhabits(&self.field_decl_v[i].1), "expected {}th field_t element (which is named {:?}) to inhabit type {} but it did not; field_t element abstract_type was {}", i, self.field_decl_v[i].0, self.field_decl_v[i].1, datum.abstract_type());
        }
        Ok(())
    }
    /// Simpler version of verify_inhabitation_by which only returns a bool.
    pub(crate) fn is_inhabited_by(&self, field_t: &dy::TupleTerm) -> bool {
        if field_t.len() != self.field_decl_v.len() {
            return false;
        }
        for (i, datum) in field_t.iter().enumerate() {
            if !datum.inhabits(&self.field_decl_v[i].1) {
                return false;
            }
        }
        true
    }
}

impl dy::Constructor for StructTerm {
    type ConstructedType = dy::StructTermTerm;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        self.verify_inhabitation_by(&parameter_t)?;
        // log::warn!("NOTE: Just copying the StructTerm as the StructTermTerm's type for now. TODO: figure out what the right approach is");
        Ok(dy::StructTermTerm::new_unchecked(
            self.clone().into(),
            parameter_t,
        ))
    }
    fn deserialize_parameters_and_construct(
        &self,
        reader: &mut dyn std::io::Read,
    ) -> Result<Self::ConstructedType> {
        use st::Deserializable;
        let struct_term_term = Self::ConstructedType::deserialize(reader)?;
        use dy::Deconstruct;
        // NOTE: this inhabitation check could cause a sym ref dereference, which is not allowed.  any type checking
        // could be done as a separate pass, though that would present problems for static types.
        anyhow::ensure!(
            struct_term_term.inhabits(self),
            "type mismatch in StructTerm::deserialize_parameters_and_construct; expected type_ {} but got {}",
            self.textified(),
            struct_term_term.direct_type().textified(),
        );
        Ok(struct_term_term)
    }
}

impl dy::Deconstruct for StructTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        // TODO: how to incorporate symbol_id?
        dy::ParametricDeconstruction::new_recursive(
            Struct.into(),
            // TODO: This will simplify later once symbol_decl is a thing.
            self.field_decl_v
                .into_iter()
                .map(|(field_name, field_type)| {
                    dy::Value::from(dy::TupleTerm::from((field_name, field_type)))
                })
                .collect::<Vec<dy::Value>>()
                .into(),
        )
        .into()
    }
}

impl std::fmt::Display for StructTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl Inhabits<Struct> for StructTerm {
    fn inhabits(&self, _: &Struct) -> bool {
        true
    }
}

impl st::Inhabits<st::Type> for StructTerm {
    fn inhabits(&self, _: &st::Type) -> bool {
        true
    }
}

impl st::Deserializable for StructTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let len = st::read_len(reader)?;
        let mut field_decl_v = Vec::with_capacity(len);
        for _ in 0..len {
            let field_name = String::deserialize(reader)?;
            let field_type = dy::Value::deserialize(reader)?;
            field_decl_v.push((field_name, field_type));
        }
        Ok(Self::new(field_decl_v)?)
    }
}

impl st::Serializable for StructTerm {
    //     fn serialize_top_level_code(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::SerializedTopLevelCode::Construction.write(writer)?)
    //     }
    //     fn serialize_constructor(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
    //         Ok(st::Struct.serialize(writer)?)
    //     }
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where a StructTerm smaller than 8 bytes is encoded in exactly 8 bytes.
        // TODO: Once Vec<T> is supported as a term of ArrayE(T), then this could change
        // simply into self.field_decl_v.serialize(writer)
        let mut bytes_written = st::write_len(self.field_decl_v.len(), writer)?;
        for (field_name, field_type) in self.field_decl_v.iter() {
            bytes_written += field_name.serialize(writer)?;
            bytes_written += field_type.serialize(writer)?;
        }
        Ok(bytes_written)
    }
}

impl Stringifiable for StructTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Struct(");
        for (i, (field_name, field_type)) in self.field_decl_v.iter().enumerate() {
            // TODO: Probably use write! here, because it can write directly to a String apparently?
            s.push_str(&format!("{:?}: {}", field_name, field_type));
            if i + 1 < self.field_decl_v.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl st::TestValues for StructTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec![
            // Empty struct
            Self::new(vec![]).unwrap(),
            Self::new(vec![("x".to_string(), st::Float64.into())]).unwrap(),
            Self::new(vec![
                ("x".to_string(), st::Float64.into()),
                ("Y".to_string(), st::Array.into()),
            ])
            .unwrap(),
            // Nested struct
            {
                let inner_struct_term = Self::new(vec![
                    ("x".to_string(), st::Float64.into()),
                    ("Y".to_string(), st::Array.into()),
                ])
                .unwrap();
                Self::new(vec![
                    ("b".to_string(), st::Bool.into()),
                    ("t".to_string(), st::TrueType.into()),
                    ("g".to_string(), inner_struct_term.into()),
                ])
                .unwrap()
            },
        ]
    }
}

impl st::TypeTrait for StructTerm {}
