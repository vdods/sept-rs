use crate::dy;

#[derive(Debug)]
pub struct GlobalSymRefTermReadGuard<'a> {
    pub(crate) global_sym_ref_term: &'a dy::GlobalSymRefTerm,
    pub(crate) global_symbol_table_g: std::sync::RwLockReadGuard<'a, dy::SymbolTable>,
}

impl<'a> AsRef<dy::Value> for GlobalSymRefTermReadGuard<'a> {
    fn as_ref(&self) -> &dy::Value {
        self.global_symbol_table_g.resolve_symbol(&self.global_sym_ref_term.symbol_id).unwrap()
    }
}

impl<'a> std::ops::Deref for GlobalSymRefTermReadGuard<'a> {
    type Target = dy::Value;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a> std::fmt::Display for GlobalSymRefTermReadGuard<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(self.as_ref().fmt(f)?)
    }
}
