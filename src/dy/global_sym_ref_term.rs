use crate::{dy::{self, RUNTIME_LA}, st::{self, /*GlobalSymRef, Inhabits, */Stringify, TermTrait}};

// TODO: Figure out the naming scheme, squaring against the conventions of the c++ sept implementation
// TODO: Make a `mod st` version of this that also specifies the type of the resolved value.
#[derive(Clone, Debug, derive_more::Into, PartialEq)]
pub struct GlobalSymRefTerm {
    pub symbol_id: String,
}

impl dy::IntoValue for GlobalSymRefTerm {}

// impl std::ops::Deref for GlobalSymRefTerm {
//     type Target = dy::Value;
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

impl std::fmt::Display for GlobalSymRefTerm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", &self.stringify())
    }
}

// TODO: Maybe replace this with new_checked and new_unchecked.
impl From<&str> for GlobalSymRefTerm {
    fn from(s: &str) -> Self {
        Self { symbol_id: s.into() }
    }
}

impl From<String> for GlobalSymRefTerm {
    fn from(s: String) -> Self {
        Self { symbol_id: s }
    }
}

// NOTE: This one is not there because GlobalSymRef is defined to have referential transparency, so this would violate that.
// impl Inhabits<GlobalSymRef> for GlobalSymRefTerm {
//     fn inhabits(&self, _: &GlobalSymRef) -> bool {
//         true
//     }
// }

impl Stringify for GlobalSymRefTerm {
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn stringify(&self) -> String {
        RUNTIME_LA.read().unwrap().global_symbol_table.resolve_symbol(&self.symbol_id).unwrap().stringify()
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
        RUNTIME_LA.read().unwrap().global_symbol_table.resolve_symbol(&self.symbol_id).unwrap().is_parametric_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn is_type_term(&self) -> bool {
        RUNTIME_LA.read().unwrap().global_symbol_table.resolve_symbol(&self.symbol_id).unwrap().is_type_term()
    }
    /// Forwards via referential transparency.
    /// NOTE: This panics if the symbol isn't defined, which is probably not great.
    fn abstract_type(&self) -> Self::AbstractTypeFnReturnType {
        RUNTIME_LA.read().unwrap().global_symbol_table.resolve_symbol(&self.symbol_id).unwrap().abstract_type()
    }
}

impl st::TypeTrait for GlobalSymRefTerm {}

// // TODO: Probably move this into its own file
// #[derive(derive_more::From, derive_more::Into)]
// pub struct GlobalSymRefTermReadGuard<'a> {
//     global_sym_ref_term: &'a GlobalSymRefTerm,
//     guard: std::sync::RwLockReadGuard<'a, dy::Runtime>,
// }
//
// impl<'a> AsRef<dy::Value> for GlobalSymRefTermReadGuard<'a> {
//     fn as_ref(&self) -> &dy::Value {
//         self.guard.global_symbol_table.resolve_symbol(&self.global_sym_ref_term.symbol_id).unwrap()
//     }
// }
