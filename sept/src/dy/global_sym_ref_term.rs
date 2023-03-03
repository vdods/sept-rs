use crate::{dy::{self, TransparentRefTrait}, Result, st::{self, Stringifiable, TermTrait}};
use std::sync::{Arc, RwLock};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
// TODO: Make a `mod st` version of this that also specifies the type of the resolved value.
#[derive(Clone, Debug, dy::IntoValue)]
pub struct GlobalSymRefTerm {
    pub symbol_id: String,
}

impl dy::Constructor for GlobalSymRefTerm {
    type ConstructedType = dy::Value;
    fn construct(&self, parameter_t: dy::TupleTerm) -> Result<Self::ConstructedType> {
        Ok(self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().construct(parameter_t)?)
    }
    fn deserialize_parameters_and_construct(&self, reader: &mut dyn std::io::Read) -> Result<Self::ConstructedType> {
        Ok(self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().deserialize_parameters_and_construct(reader)?)
    }
}

/// GlobalSymRefTerm's impl for dy::Deconstruct does not use referential transparency, because
/// the goal is to represent the thing exactly as it is.
impl dy::Deconstruct for GlobalSymRefTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new(st::GlobalSymRef.deconstructed(), vec![self.symbol_id.deconstructed()]).into()
    }
    fn deconstructed(&self) -> dy::Deconstruction {
        dy::ParametricDeconstruction::new(st::GlobalSymRef.deconstructed(), vec![self.symbol_id.deconstructed()]).into()
    }
}

impl std::fmt::Display for GlobalSymRefTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl st::Inhabits<st::Type> for GlobalSymRefTerm {
    fn inhabits(&self, _: &st::Type) -> bool {
        self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().is_type()
    }
}

impl st::Inhabits<dy::Value> for GlobalSymRefTerm {
    fn inhabits(&self, rhs: &dy::Value) -> bool {
        self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().inhabits(rhs)
    }
}

impl PartialEq<GlobalSymRefTerm> for GlobalSymRefTerm {
    fn eq(&self, rhs: &GlobalSymRefTerm) -> bool {
        // Special case shortcut where the symbol_id values are equal (this may not be a worthwhile
        // shortcut).  Otherwise delegate to the runtime.
        self.symbol_id == rhs.symbol_id || dy::RUNTIME_LA.read().unwrap().eq(self, rhs)
    }
}

impl st::Deserializable for GlobalSymRefTerm {
    fn deserialize(reader: &mut dyn std::io::Read) -> Result<Self> {
        let symbol_id = String::deserialize(reader)?;
        Ok(Self::new_unchecked(symbol_id))
    }
}

impl st::Serializable for GlobalSymRefTerm {
    fn serialize(&self, writer: &mut dyn std::io::Write) -> Result<usize> {
        Ok(self.symbol_id.serialize(writer)?)
    }
}

impl Stringifiable for GlobalSymRefTerm {
    fn stringify(&self) -> String {
        format!("GlobalSymRefTerm({:?})", self.symbol_id)
    }
}

impl TermTrait for GlobalSymRefTerm {
    type AbstractTypeType = dy::Value;

    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_parametric(&self) -> bool {
        self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().is_parametric()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_type(&self) -> bool {
        self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().is_type()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn abstract_type(&self) -> Self::AbstractTypeType {
        self.resolved().expect("GlobalSymRefTerm failed to resolve").read().unwrap().abstract_type()
    }
}

impl st::TestValues for GlobalSymRefTerm {
    fn fixed_test_values() -> Vec<Self> {
        vec![Self::new_unchecked("blah".to_string())]
    }
}

impl st::TypeTrait for GlobalSymRefTerm {}

impl TransparentRefTrait for GlobalSymRefTerm {
    fn dereferenced_once(&self) -> Result<Arc<RwLock<dy::Value>>> {
        Ok(dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolved_symbol(&self.symbol_id)?)
    }
}

impl GlobalSymRefTerm {
    /// This constructor ensures the symbolic reference resolves before returning.
    // TODO: Maybe add new_checked_typed which also checks the type of the referred value.
    pub fn new_checked(symbol_id: String) -> Result<Self> {
        dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolved_symbol(&symbol_id)?;
        Ok(Self { symbol_id })
    }
    /// This constructor doesn't check that the symbolic reference resolves before returning.
    /// This would be useful e.g. if the referred symbol has yet to be defined.
    pub fn new_unchecked(symbol_id: String) -> Self {
        Self { symbol_id }
    }

    /// Explicitly resolves (dereferences) this ref.
    pub fn resolved(&self) -> Result<Arc<RwLock<dy::Value>>> {
        Ok(dy::RUNTIME_LA.read().unwrap().dereferenced_inner(self.dereferenced_once()?)?)
    }
}
