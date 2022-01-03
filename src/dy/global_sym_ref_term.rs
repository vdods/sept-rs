use crate::{dy::{self, GLOBAL_SYMBOL_TABLE_LA}, st::{self, Stringify, TermTrait}};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
// TODO: Make a `mod st` version of this that also specifies the type of the resolved value.
#[derive(Clone, Debug, PartialEq)]
pub struct GlobalSymRefTerm {
    pub symbol_id: String,
}

impl dy::IntoValue for GlobalSymRefTerm {}

impl std::fmt::Display for GlobalSymRefTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

impl st::Inhabits<dy::Value> for GlobalSymRefTerm {
    fn inhabits(&self, rhs: &dy::Value) -> bool {
        dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().inhabits(rhs)
    }
}

impl Stringify for GlobalSymRefTerm {
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn stringify(&self) -> String {
        dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().stringify()
    }
}

impl TermTrait for GlobalSymRefTerm {
    type AbstractTypeFnReturnType = dy::Value;

    fn label() -> &'static str {
        "GlobalSymRefTerm"
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_parametric_term(&self) -> bool {
        GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().is_parametric_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_type_term(&self) -> bool {
        GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().is_type_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().abstract_type()
    }
}

impl st::TypeTrait for GlobalSymRefTerm {}

impl GlobalSymRefTerm {
    /// This constructor ensures the symbolic reference resolves before returning.
    // TODO: Maybe add new_checked_typed which also checks the type of the referred value.
    pub fn new_checked(symbol_id: String) -> anyhow::Result<Self> {
        GLOBAL_SYMBOL_TABLE_LA.read().unwrap().resolve_symbol(&symbol_id)?;
        Ok(Self { symbol_id })
    }
    /// This constructor doesn't check that the symbolic reference resolves before returning.
    /// This would be useful e.g. if the referred symbol has yet to be defined.
    pub fn new_unchecked(symbol_id: String) -> Self {
        Self { symbol_id }
    }

    /// Explicitly dereferences this ref.
    // TODO: Should this somehow be implemented via Deref?
    pub fn dereferenced<'a>(&'a self) -> dy::GlobalSymRefTermReadGuard<'a> {
        dy::GlobalSymRefTermReadGuard {
            global_sym_ref_term: self,
            global_symbol_table_g: dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap(),
        }
    }
    /// Explicitly dereferences this ref.
    // TODO: Should this somehow be implemented via DerefMut?
    pub fn dereferenced_mut<'a>(&'a self) -> dy::GlobalSymRefTermWriteGuard<'a> {
        dy::GlobalSymRefTermWriteGuard {
            global_sym_ref_term: self,
            global_symbol_table_g: dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap(),
        }
    }
}
