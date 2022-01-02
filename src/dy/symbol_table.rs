use crate::dy;
use std::{collections::HashMap/*, sync::RwLock*/};

/// Default is to construct an empty SymbolTable.
#[derive(Debug, Default)]
pub struct SymbolTable {
    symbol_map: HashMap<String, dy::Value>,
//     parent_symbol_table_lo: Option<&'static RwLock<SymbolTable>>,
}

impl SymbolTable {
//     /// Constructs a new, empty SymbolTable with no parent.
//     pub fn new() -> Self {
//         Self { symbol_map: HashMap::new()/*, parent_symbol_table_lo: None*/ }
//     }

    pub fn resolve_symbol(&self, symbol_id: &str) -> anyhow::Result<&dy::Value> {
        Ok(self.symbol_map.get(symbol_id).ok_or_else(|| anyhow::anyhow!("unresolved symbol {:?}", symbol_id))?)
    }
//     pub fn resolve_symbol(&self, symbol_id: &str) -> anyhow::Result<&dy::Value> {
//         match self.symbol_map.get(symbol_id) {
//             Some(value) => Ok(value),
//             None => match self.parent_symbol_table_lo {
//                 Some(parent_symbol_table_l) => Ok(parent_symbol_table_l.read()?.resolve_symbol(symbol_id)?),
//                 None => { anyhow::bail!("unresolved symbol {:?}", symbol_id); }
//             }
//         }
//     }
    pub fn resolve_symbol_mut(&mut self, symbol_id: &str) -> anyhow::Result<&mut dy::Value> {
        Ok(self.symbol_map.get_mut(symbol_id).ok_or_else(|| anyhow::anyhow!("unresolved symbol {:?}", symbol_id))?)
    }

    pub fn symbol_is_defined(&self, symbol_id: &str) -> bool {
        self.symbol_map.contains_key(symbol_id)
    }
    pub fn define_symbol(&mut self, symbol_id: impl Into<String>, value: dy::Value) -> anyhow::Result<()> {
        let symbol_id_string: String = symbol_id.into();
        // TODO: Use HashMap::try_insert whenever that becomes a stabilized feature.
        anyhow::ensure!(!self.symbol_map.contains_key(&symbol_id_string), "symbol {:?} is already defined; can't redefine", symbol_id_string);
        self.symbol_map.insert(symbol_id_string, value);
        Ok(())
    }
//     pub fn has_parent_symbol_table(&self) -> bool {
//         false
//     }
//     pub fn parent_symbol_table(&self) -> ? {
//     }

    // TODO: Figure out what this should do with parent
    pub fn clear(&mut self) {
        self.symbol_map.clear()
    }
}
