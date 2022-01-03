use crate::dy;

#[derive(Debug)]
pub struct GlobalSymRefTermWriteGuard<'a> {
    pub(crate) global_sym_ref_term: &'a dy::GlobalSymRefTerm,
    pub(crate) global_symbol_table_g: std::sync::RwLockWriteGuard<'a, dy::SymbolTable>,
}

impl<'a> AsMut<dy::Value> for GlobalSymRefTermWriteGuard<'a> {
    fn as_mut(&mut self) -> &mut dy::Value {
        self.global_symbol_table_g.resolve_symbol_mut(&self.global_sym_ref_term.symbol_id).unwrap()
    }
}

impl<'a> AsRef<dy::Value> for GlobalSymRefTermWriteGuard<'a> {
    fn as_ref(&self) -> &dy::Value {
        self.global_symbol_table_g.resolve_symbol(&self.global_sym_ref_term.symbol_id).unwrap()
    }
}

impl<'a> std::ops::Deref for GlobalSymRefTermWriteGuard<'a> {
    type Target = dy::Value;
    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<'a> std::ops::DerefMut for GlobalSymRefTermWriteGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<'a> std::fmt::Display for GlobalSymRefTermWriteGuard<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(self.as_ref().fmt(f)?)
    }
}

