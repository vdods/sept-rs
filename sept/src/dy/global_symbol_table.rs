use crate::dy;
use std::sync::{Arc, RwLock};

lazy_static::lazy_static! {
    /// This is the static singleton global SymbolTable.
    pub static ref GLOBAL_SYMBOL_TABLE_LA: Arc<RwLock<dy::SymbolTable>> = Arc::new(RwLock::new(dy::SymbolTable::new_with_parent(None)));
}
