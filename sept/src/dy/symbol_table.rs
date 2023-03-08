use crate::{dy, Result};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub struct SymbolTable {
    pub unqualified_name: String,
    symbol_map: HashMap<String, Arc<RwLock<dy::Value>>>,
    parent_symbol_table_lao: Option<Arc<RwLock<SymbolTable>>>,
}

impl SymbolTable {
    /// Constructs the global symbol table
    pub(crate) fn new_global_symbol_table() -> Self {
        Self {
            unqualified_name: String::new(),
            symbol_map: HashMap::new(),
            parent_symbol_table_lao: None,
        }
    }
    /// Constructs a new, empty SymbolTable with no parent.
    pub fn new_without_parent(unqualified_name: String) -> Result<Self> {
        anyhow::ensure!(
            !unqualified_name.is_empty(),
            "SymbolTable unqualified_name must not be empty"
        );
        anyhow::ensure!(
            !unqualified_name.contains(':'),
            "SymbolTable unqualified_name (which was {:?}) is not allowed contain ':'",
            unqualified_name
        );
        Ok(Self {
            unqualified_name,
            symbol_map: HashMap::new(),
            parent_symbol_table_lao: None,
        })
    }
    /// Constructs a new, empty SymbolTable with given parent.
    pub fn new_with_parent(
        unqualified_name: String,
        parent_symbol_table_la: Arc<RwLock<SymbolTable>>,
    ) -> Result<Self> {
        anyhow::ensure!(
            !unqualified_name.is_empty(),
            "SymbolTable unqualified_name must not be empty"
        );
        anyhow::ensure!(
            !unqualified_name.contains(':'),
            "SymbolTable unqualified_name (which was {:?}) is not allowed contain ':'",
            unqualified_name
        );
        Ok(Self {
            unqualified_name,
            symbol_map: HashMap::new(),
            parent_symbol_table_lao: Some(parent_symbol_table_la),
        })
    }

    /// Returns true if this SymbolTable is the global symbol table (i.e. identical to the one
    /// owned by GLOBAL_SYMBOL_TABLE_LA).
    pub fn is_global_symbol_table(&self) -> bool {
        std::ptr::eq(self, &*dy::GLOBAL_SYMBOL_TABLE_LA.read().unwrap())
    }
    /// This is the path from the root
    pub fn path(&self) -> String {
        let mut retval = String::new();
        self.path_impl(&mut retval);
        retval
    }
    fn path_impl(&self, buffer: &mut String) {
        // If this is the global symbol table, then its path is "".
        if self.is_global_symbol_table() {
            assert!(buffer.is_empty());
            return;
        }

        // Otherwise, recurse into the parent symbol table if there is one.
        if let Some(parent_symbol_table_la) = self.parent_symbol_table_lao.as_ref() {
            parent_symbol_table_la.read().unwrap().path_impl(buffer);
            buffer.push_str("::");
        }
        // Append this symbol table's name.
        buffer.push_str(self.unqualified_name.as_str());
    }
    /// This simply appends the given symbol_id to the path of this SymbolTable.
    // TODO: Make a newtype called SymbolId that does the validation.
    pub fn unresolved_symbol_path(&self, symbol_id: &str) -> Result<String> {
        anyhow::ensure!(
            !symbol_id.is_empty(),
            "symbol_id is not allowed to be empty"
        );
        anyhow::ensure!(
            !symbol_id.contains(':'),
            "symbol_id (which was {:?}) is not allowed to contain ':'",
            symbol_id
        );
        let mut retval = self.path();
        retval.push_str("::");
        retval.push_str(symbol_id);
        Ok(retval)
    }
    /// This resolves the given symbol_id to the SymbolTable in which that symbol is defined, and returns the
    /// path of that symbol.
    pub fn resolved_symbol_path(&self, symbol_id: &str) -> Result<String> {
        anyhow::ensure!(
            !symbol_id.is_empty(),
            "symbol_id is not allowed to be empty"
        );
        anyhow::ensure!(
            !symbol_id.contains(':'),
            "symbol_id (which was {:?}) is not allowed to contain ':'",
            symbol_id
        );
        if self.symbol_map.contains_key(symbol_id) {
            let mut retval = String::new();
            self.path_impl(&mut retval);
            retval.push_str("::");
            retval.push_str(symbol_id);
            Ok(retval)
        } else {
            match &self.parent_symbol_table_lao {
                Some(parent_symbol_table_la) => Ok(parent_symbol_table_la
                    .read()
                    .unwrap()
                    .resolved_symbol_path(symbol_id)?),
                None => {
                    anyhow::bail!("unresolved symbol {:?}", symbol_id);
                }
            }
        }
    }
    pub fn resolved_symbol(&self, symbol_id: &str) -> Result<Arc<RwLock<dy::Value>>> {
        anyhow::ensure!(
            !symbol_id.contains(':'),
            "symbol_id (which was {:?}) is not allowed to contain ':'",
            symbol_id
        );
        match self.symbol_map.get(symbol_id) {
            Some(value) => Ok(value.clone()),
            None => match &self.parent_symbol_table_lao {
                Some(parent_symbol_table_la) => Ok(parent_symbol_table_la
                    .read()
                    .unwrap()
                    .resolved_symbol(symbol_id)?),
                None => {
                    anyhow::bail!("unresolved symbol {:?}", symbol_id);
                }
            },
        }
    }
    pub fn resolved_symbol_mut(&mut self, symbol_id: &str) -> Result<Arc<RwLock<dy::Value>>> {
        match self.symbol_map.get_mut(symbol_id) {
            Some(value) => Ok(value.clone()),
            None => match &self.parent_symbol_table_lao {
                Some(parent_symbol_table_la) => Ok(parent_symbol_table_la
                    .write()
                    .unwrap()
                    .resolved_symbol_mut(symbol_id)?),
                None => {
                    anyhow::bail!("unresolved symbol {:?}", symbol_id);
                }
            },
        }
    }

    pub fn has_parent_symbol_table(&self) -> bool {
        self.parent_symbol_table_lao.is_some()
    }
    pub fn symbol_is_defined(&self, symbol_id: &str) -> bool {
        self.symbol_map.contains_key(symbol_id)
            || match &self.parent_symbol_table_lao {
                Some(parent_symbol_table_la) => parent_symbol_table_la
                    .read()
                    .unwrap()
                    .symbol_is_defined(symbol_id),
                None => false,
            }
    }
    pub fn define_symbol(&mut self, symbol_id: impl Into<String>, value: dy::Value) -> Result<()> {
        let symbol_id_string: String = symbol_id.into();
        // TODO: Use HashMap::try_insert whenever that becomes a stabilized feature.
        anyhow::ensure!(
            !self.symbol_map.contains_key(&symbol_id_string),
            "symbol {:?} is already defined; can't redefine",
            symbol_id_string
        );
        self.symbol_map
            .insert(symbol_id_string, Arc::new(RwLock::new(value)));
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
            write!(
                f,
                "    {:?} => {}\n",
                symbol_id,
                value_la.read().unwrap().stringify()
            )?;
        }
        match &self.parent_symbol_table_lao {
            Some(parent_symbol_table_la) => {
                write!(
                    f,
                    "}} with parent {:#?}",
                    parent_symbol_table_la.read().unwrap()
                )?;
            }
            None => {
                write!(f, "}}\n")?;
            }
        }
        Ok(())
    }
}
