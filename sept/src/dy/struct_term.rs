use crate::{dy, Result, st::{self, Inhabits, Stringify, Struct, TermTrait}};
use std::collections::HashMap;

// TODO: Theoretically, the key (i.e. name) could be any type, thereby enabling the possibility of structured names.
// But even if this isn't done, then first class sept-enabled strings should be used.
#[derive(Clone, Debug, derive_more::From, derive_more::Into, dy::IntoValue, PartialEq, st::TermTrait)]
#[st_term_trait(AbstractTypeType = "Struct", is_parametric = "self.field_decl_v.len() > 0", is_type = "true")]
pub struct StructTerm {
    /// This stores the field declarations (i.e. `field: Type`) in a particular order.
    // TODO: Check that each is a type.
    // TODO: Maybe separate this out into name_v (and eventually name_tuple_term) and type_tuple_term,
    // which would simplify various checks and projections into TupleTerm.
    // TODO: Probably eventually allow arbitrary terms as the field names.
    // TODO: Define and use SymbolDecl as a formal type.
    pub(crate) field_decl_v: Vec<(String, dy::Value)>,
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
        let name_index_m: HashMap<String, usize> = field_decl_v.iter().enumerate().map(|(i, (name, _))| (name.clone(), i)).collect();
        Ok(Self { field_decl_v, name_index_m })
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
        log::warn!("NOTE: Just copying the StructTerm as the StructTermTerm's type for now. TODO: figure out what the right approach is");
        Ok(dy::StructTermTerm::new_unchecked(self.clone().into(), parameter_t))
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
        ).into()
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

impl st::Serializable for StructTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        // TODO: Figure out if this should be u64 or u32, or if there's some smarter encoding
        // like where a StructTerm smaller than 8 bytes is encoded in exactly 8 bytes.
        let mut bytes_written = (self.field_decl_v.len() as u64).serialize(writer)?;
        for (field_name, field_type) in self.field_decl_v.iter() {
            bytes_written += field_name.serialize(writer)?;
            bytes_written += field_type.serialize(writer)?;
        }
        Ok(bytes_written)
    }
}

impl Stringify for StructTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Struct(");
        for (i, (field_name, field_type)) in self.field_decl_v.iter().enumerate() {
            // TODO: Probably use write! here, because it can write directly to a String apparently?
            s.push_str(&format!("{:?}: {}", field_name, field_type));
            if i+1 < self.field_decl_v.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl st::TypeTrait for StructTerm {}
