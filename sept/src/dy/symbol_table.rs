use crate::{dy, Result};
use std::{collections::HashMap, sync::{Arc, RwLock}};

pub struct SymbolTable {
    // TODO: Maybe add a name for the symbol table; this would most likely be a kind of fully qualified namespace.
    symbol_map: HashMap<String, Arc<RwLock<dy::Value>>>,
    parent_symbol_table_lao: Option<Arc<RwLock<SymbolTable>>>,
}

impl SymbolTable {
    /// Constructs a new, empty SymbolTable with given parent.  Specify None for no parent.
    pub fn new_with_parent(parent_symbol_table_lao: Option<Arc<RwLock<SymbolTable>>>) -> Self {
        Self { symbol_map: HashMap::new(), parent_symbol_table_lao }
    }

    pub fn resolved_symbol(&self, symbol_id: &str) -> Result<Arc<RwLock<dy::Value>>> {
        match self.symbol_map.get(symbol_id) {
            Some(value) => Ok(value.clone()),
            None => match &self.parent_symbol_table_lao {
                Some(parent_symbol_table_l) => Ok(parent_symbol_table_l.read().unwrap().resolved_symbol(symbol_id)?),
                None => { anyhow::bail!("unresolved symbol {:?}", symbol_id); }
            }
        }
    }
    pub fn resolved_symbol_mut(&mut self, symbol_id: &str) -> Result<Arc<RwLock<dy::Value>>> {
        match self.symbol_map.get_mut(symbol_id) {
            Some(value) => Ok(value.clone()),
            None => match &self.parent_symbol_table_lao {
                Some(parent_symbol_table_l) => Ok(parent_symbol_table_l.write().unwrap().resolved_symbol_mut(symbol_id)?),
                None => { anyhow::bail!("unresolved symbol {:?}", symbol_id); }
            }
        }
    }

    pub fn has_parent_symbol_table(&self) -> bool {
        self.parent_symbol_table_lao.is_some()
    }
    pub fn symbol_is_defined(&self, symbol_id: &str) -> bool {
        self.symbol_map.contains_key(symbol_id) ||
        match &self.parent_symbol_table_lao {
            Some(parent_symbol_table_l) => parent_symbol_table_l.read().unwrap().symbol_is_defined(symbol_id),
            None => false,
        }
    }
    pub fn define_symbol(&mut self, symbol_id: impl Into<String>, value: dy::Value) -> Result<()> {
        let symbol_id_string: String = symbol_id.into();
        // TODO: Use HashMap::try_insert whenever that becomes a stabilized feature.
        anyhow::ensure!(!self.symbol_map.contains_key(&symbol_id_string), "symbol {:?} is already defined; can't redefine", symbol_id_string);
        self.symbol_map.insert(symbol_id_string, Arc::new(RwLock::new(value)));
        Ok(())
    }

    // TODO: Figure out what this should do with parent
    pub fn clear(&mut self) {
        log::warn!("Clearing symbol table but NOT clearing its parent symbol table, if any; this is a somewhat arbitrary decision, though this clear method is probably only for testing purposes");
        self.symbol_map.clear()
    }
}

impl std::fmt::Debug for SymbolTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "SymbolTable {:p} {{\n", self)?;
        for (symbol_id, value_la) in self.symbol_map.iter() {
            use crate::st::Stringifiable;
            write!(f, "    {:?} => {}\n", symbol_id, value_la.read().unwrap().stringify())?;
        }
        match &self.parent_symbol_table_lao {
            Some(parent_symbol_table_la) => {
                write!(f, "}} with parent {:#?}", parent_symbol_table_la.read().unwrap())?;
            },
            None => {
                write!(f, "}}\n")?;
            }
        }
        Ok(())
    }
}
