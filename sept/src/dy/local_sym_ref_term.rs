use crate::{dy::{self, TransparentRefTrait}, Result, st::{self, Stringifiable, TermTrait}};
use std::sync::{Arc, RwLock};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
// TODO: Make a `mod st` version of this that also specifies the type of the resolved value.
#[derive(Clone, Debug, dy::IntoValue)]
pub struct LocalSymRefTerm {
    /// This is the symbol table to which this sym ref refers.
    local_symbol_table_la: Arc<RwLock<dy::SymbolTable>>,
    /// This is the symbol name for the reference.
    pub symbol_id: String,
}

// TODO: Implement Constructor

/// LocalSymRefTerm's impl for dy::Deconstruct does not use referential transparency, because
/// the goal is to represent the thing exactly as it is.
impl dy::Deconstruct for LocalSymRefTerm {
    fn deconstruct(self) -> dy::Deconstruction {
        unimplemented!("not sure how to represent the local symbol table unless it's somehow named and has a deconstruction");
//         dy::ParametricDeconstruction::new(st::LocalSymRef.deconstructed(), vec![/* local symbol table deconstruction would go here*/ self.symbol_id.deconstructed()]).into()
    }
    fn deconstructed(&self) -> dy::Deconstruction {
        unimplemented!("not sure how to represent the local symbol table unless it's somehow named and has a deconstruction");
//         dy::ParametricDeconstruction::new(st::LocalSymRef.deconstructed(), vec![/* local symbol table deconstruction would go here*/ self.symbol_id.deconstructed()]).into()
    }
}


impl std::fmt::Display for LocalSymRefTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl st::Inhabits<st::Type> for LocalSymRefTerm {
    fn inhabits(&self, _: &st::Type) -> bool {
        self.resolved().expect("LocalSymRefTerm failed to resolve").read().unwrap().is_type()
    }
}

impl st::Inhabits<dy::Value> for LocalSymRefTerm {
    fn inhabits(&self, rhs: &dy::Value) -> bool {
        self.resolved().expect("LocalSymRefTerm failed to resolve").read().unwrap().inhabits(rhs)
    }
}

impl PartialEq<LocalSymRefTerm> for LocalSymRefTerm {
    fn eq(&self, rhs: &LocalSymRefTerm) -> bool {
        // Special case shortcut where the symbol table pointers and symbol_id values are equal
        // (this may not be a worthwhile shortcut).  Otherwise delegate to the runtime.
        (Arc::ptr_eq(&self.local_symbol_table_la, &rhs.local_symbol_table_la) && self.symbol_id == rhs.symbol_id)
        ||
        dy::RUNTIME_LA.read().unwrap().eq(self, rhs)
    }
}

impl Stringifiable for LocalSymRefTerm {
    fn stringify(&self) -> String {
        format!("LocalSymRefTerm({:?}, {:?})", Arc::as_ptr(&self.local_symbol_table_la), self.symbol_id)
    }
}

impl TermTrait for LocalSymRefTerm {
    type AbstractTypeType = dy::Value;

    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_parametric(&self) -> bool {
        self.resolved().expect("LocalSymRefTerm failed to resolve").read().unwrap().is_parametric()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_type(&self) -> bool {
        self.resolved().expect("LocalSymRefTerm failed to resolve").read().unwrap().is_type()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn abstract_type(&self) -> Self::AbstractTypeType {
        self.resolved().expect("LocalSymRefTerm failed to resolve").read().unwrap().abstract_type()
    }
}

impl st::TypeTrait for LocalSymRefTerm {}

impl dy::TransparentRefTrait for LocalSymRefTerm {
    fn dereferenced_once(&self) -> Result<Arc<RwLock<dy::Value>>> {
        Ok(self.local_symbol_table_la.read().unwrap().resolved_symbol(&self.symbol_id)?)
    }
}

impl LocalSymRefTerm {
    /// This constructor ensures the symbolic reference resolves before returning.
    // TODO: Maybe add new_checked_typed which also checks the type of the referred value.
    pub fn new_checked(
        local_symbol_table_la: Arc<RwLock<dy::SymbolTable>>,
        symbol_id: String,
    ) -> Result<Self> {
        local_symbol_table_la.read().unwrap().resolved_symbol(&symbol_id)?;
        Ok(Self { local_symbol_table_la, symbol_id })
    }
    /// This constructor doesn't check that the symbolic reference resolves before returning.
    /// This would be useful e.g. if the referred symbol has yet to be defined.
    pub fn new_unchecked(local_symbol_table_la: Arc<RwLock<dy::SymbolTable>>, symbol_id: String) -> Self {
        Self { local_symbol_table_la, symbol_id }
    }

    /// Explicitly resolves (dereferences) this ref.
    pub fn resolved(&self) -> Result<Arc<RwLock<dy::Value>>> {
        Ok(dy::RUNTIME_LA.read().unwrap().dereferenced_inner(self.dereferenced_once()?)?)
    }
}
