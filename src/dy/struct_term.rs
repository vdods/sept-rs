use crate::{dy, st::{self, Inhabits, Stringify, Struct, TermTrait}};
use std::collections::HashMap;

// TODO: Theoretically, the key (i.e. name) could be any type, thereby enabling the possibility of structured names.
// But even if this isn't done, then first class sept-enabled strings should be used.
#[derive(Debug, derive_more::From, derive_more::Into, PartialEq)]
pub struct StructTerm {
    // NOTE: This is probably temporary, since constructing a term of this type won't necessarily
    // use GlobalSymRefTerm.
    symbol_id: String,
    /// This stores the actual `elem: Type` pairs in a particular order.  TODO: Check that each is a type.
    /// Theoretically this could store non-types, but that is feature creep for structs.
    // TODO: Maybe separate this out into name_v (and eventually name_tuple_term) and type_tuple_term,
    // which would simplify various checks and projections into TupleTerm.
    pub(crate) ordered_type_v: Vec<(String, dy::Value)>,
    /// This is a cache for the quick lookup of the element index based on a name.
    name_index_m: HashMap<String, usize>,
}

impl dy::IntoValue for StructTerm {}

/// TODO: Implement projection to TupleTerm of types.
impl StructTerm {
    pub fn new(symbol_id: String, ordered_type_v: Vec<(String, dy::Value)>) -> Self {
        // Generate name_index_m.
        let name_index_m: HashMap<String, usize> = ordered_type_v.iter().enumerate().map(|(i, (name, _))| (name.clone(), i)).collect();
        Self { symbol_id, ordered_type_v, name_index_m }
    }
    // TEMP HACK NAME -- this type-checks the contents.
    pub fn construct(&self, element_tuple_term: dy::TupleTerm) -> anyhow::Result<dy::StructTermTerm> {
        self.verify_inhabitation_by(&element_tuple_term)?;
        Ok(dy::StructTermTerm::new(dy::GlobalSymRefTerm::from(self.symbol_id.clone()), element_tuple_term)?)
    }
    pub(crate) fn verify_inhabitation_by(&self, element_tuple_term: &dy::TupleTerm) -> anyhow::Result<()> {
        anyhow::ensure!(element_tuple_term.len() == self.ordered_type_v.len(), "mismatch in number of type elements in StructTerm (which was {}) and in element_tuple_term (which was {})", self.ordered_type_v.len(), element_tuple_term.len());
        for (i, datum) in element_tuple_term.iter().enumerate() {
            anyhow::ensure!(datum.inhabits(&self.ordered_type_v[i].1), "expected {}th element_tuple_term element (which is named {:?}) to inhabit type {} but it did not; element_tuple_term element abstract_type was {}", i, self.ordered_type_v[i].0, self.ordered_type_v[i].1, datum.abstract_type());
        }
        Ok(())
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

impl Stringify for StructTerm {
    fn stringify(&self) -> String {
        let mut s = String::new();
        s.push_str("Struct(");
        for (i, (key, value)) in self.ordered_type_v.iter().enumerate() {
            // TODO: Probably use write! here, because it can write directly to a String apparently?
            s.push_str(&format!("{:?}: {}", key, value));
            if i+1 < self.ordered_type_v.len() {
                s.push_str(", ");
            }
        }
        s.push_str(")");
        s
    }
}

impl TermTrait for StructTerm {
    type AbstractTypeFnReturnType = Struct;

    fn label() -> &'static str {
        "StructTerm"
    }
    /// An Struct term is parametric if there is at least one parameter.
    fn is_parametric_term(&self) -> bool {
        self.ordered_type_v.len() > 0
    }
    fn is_type_term(&self) -> bool {
        // A StructTerm is always a type.
        true
    }
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        Self::AbstractTypeFnReturnType{}
    }
}

impl st::TypeTrait for StructTerm {}
