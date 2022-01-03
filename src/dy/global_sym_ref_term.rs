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
        self.dereferenced().inhabits(rhs)
    }
}

impl st::Inhabits<GlobalSymRefTerm> for GlobalSymRefTerm {
    fn inhabits(&self, rhs: &GlobalSymRefTerm) -> bool {
        self.symbol_id == rhs.symbol_id || self.dereferenced().inhabits(rhs.dereferenced().as_ref())
    }
}

impl Stringify for GlobalSymRefTerm {
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn stringify(&self) -> String {
        self.dereferenced().stringify()
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
        self.dereferenced().is_parametric_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_type_term(&self) -> bool {
        self.dereferenced().is_type_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        self.dereferenced().abstract_type()
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
    // TODO: Should this somehow be implemented via Deref? -- maybe not, otherwise there might be an infinite recursion.
    pub fn dereferenced<'a>(&'a self) -> dy::GlobalSymRefTermReadGuard<'a> {
        dy::GlobalSymRefTermReadGuard {
            global_sym_ref_term: self,
            global_symbol_table_g: dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap(),
        }
    }
    /// Explicitly dereferences this ref.
    // TODO: Should this somehow be implemented via DerefMut? -- maybe not, otherwise there might be an infinite recursion.
    pub fn dereferenced_mut<'a>(&'a self) -> dy::GlobalSymRefTermWriteGuard<'a> {
        dy::GlobalSymRefTermWriteGuard {
            global_sym_ref_term: self,
            global_symbol_table_g: dy::GLOBAL_SYMBOL_TABLE_LA.write().unwrap(),
        }
    }
}
