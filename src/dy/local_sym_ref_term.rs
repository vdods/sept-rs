use crate::{dy, st::{self, /*LocalSymRef, Inhabits, */Stringify, TermTrait}};
use std::sync::{Arc, RwLock};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
// TODO: Make a `mod st` version of this that also specifies the type of the resolved value.
#[derive(Clone, Debug)]
pub struct LocalSymRefTerm {
    /// This is the symbol table to which this sym ref refers.
    local_symbol_table_la: Arc<RwLock<dy::SymbolTable>>,
    /// This is the symbol name for the reference.
    pub symbol_id: String,
}

impl dy::IntoValue for LocalSymRefTerm {}

// impl std::ops::Deref for LocalSymRefTerm {
//     type Target = dy::Value;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl std::fmt::Display for LocalSymRefTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

// impl From<&str> for LocalSymRefTerm {
//     fn from(s: &str) -> Self {
//         Self { symbol_id: s.into() }
//     }
// }
//
// impl From<String> for LocalSymRefTerm {
//     fn from(s: String) -> Self {
//         Self { symbol_id: s }
//     }
// }

// NOTE: This one is not there because LocalSymRef is defined to have referential transparency, so this would violate that.
// impl Inhabits<LocalSymRef> for LocalSymRefTerm {
//     fn inhabits(&self, _: &LocalSymRef) -> bool {
//         true
//     }
// }

impl Stringify for LocalSymRefTerm {
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn stringify(&self) -> String {
        self.local_symbol_table_la.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().stringify()
    }
}

impl TermTrait for LocalSymRefTerm {
    type AbstractTypeFnReturnType = dy::Value;

    fn label() -> &'static str {
        "LocalSymRefTerm"
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_parametric_term(&self) -> bool {
        self.local_symbol_table_la.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().is_parametric_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_type_term(&self) -> bool {
        self.local_symbol_table_la.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().is_type_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        self.local_symbol_table_la.read().unwrap().resolve_symbol(&self.symbol_id).unwrap().abstract_type()
    }
}

impl st::TypeTrait for LocalSymRefTerm {}

// // TODO: Probably move this into its own file
// #[derive(derive_more::From, derive_more::Into)]
// pub struct LocalSymRefTermReadGuard<'a> {
//     global_sym_ref_term: &'a LocalSymRefTerm,
//     guard: std::sync::RwLockReadGuard<'a, dy::Runtime>,
// }
//
// impl<'a> AsRef<dy::Value> for LocalSymRefTermReadGuard<'a> {
//     fn as_ref(&self) -> &dy::Value {
//         self.guard.global_symbol_table.resolve_symbol(&self.global_sym_ref_term.symbol_id).unwrap()
//     }
// }

impl LocalSymRefTerm {
    /// This constructor ensures the symbolic reference resolves before returning.
    // TODO: Maybe add new_checked_typed which also checks the type of the referred value.
    pub fn new_checked(
        local_symbol_table_la: Arc<RwLock<dy::SymbolTable>>,
        symbol_id: String,
    ) -> anyhow::Result<Self> {
        local_symbol_table_la.read().unwrap().resolve_symbol(&symbol_id)?;
        Ok(Self { local_symbol_table_la, symbol_id })
    }
    /// This constructor doesn't check that the symbolic reference resolves before returning.
    /// This would be useful e.g. if the referred symbol has yet to be defined.
    pub fn new_unchecked(local_symbol_table_la: Arc<RwLock<dy::SymbolTable>>, symbol_id: String) -> Self {
        Self { local_symbol_table_la, symbol_id }
    }
}
