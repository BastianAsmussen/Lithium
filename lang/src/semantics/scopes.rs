use crate::semantics::symbols::SymbolKind;
use std::collections::HashMap;

/// A scope which defines the scope of a symbol.
///
/// # Fields
///
/// * `symbol_table` - A symbol table which maps a symbol to its kind.
#[derive(Debug, Default)]
pub struct Scope {
    pub symbol_table: HashMap<String, SymbolKind>,
}

impl Scope {
    /// Adds a symbol to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    /// * `kind` - The kind of the symbol.
    pub fn define(&mut self, name: &str, kind: SymbolKind) {
        self.symbol_table.insert(name.to_string(), kind);
    }

    /// Checks if a symbol is defined in the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    ///
    /// # Returns
    ///
    /// * `bool` - Whether the symbol is defined in the symbol table.
    #[must_use]
    pub fn is_defined(&self, name: &str) -> bool {
        self.symbol_table.contains_key(name)
    }
}
