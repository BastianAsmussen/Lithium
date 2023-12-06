use crate::semantics::errors::Error;
use std::collections::HashMap;
use std::fmt::Display;

/// A symbol table is a data structure used by a compiler to keep track of scope and binding information about names.
///
/// # Fields
///
/// * `symbols` - The symbols in the symbol table.
#[derive(Debug, Default, Clone, Eq, PartialEq)]
pub struct SymbolTable {
    pub symbols: HashMap<String, Symbol>,
}

impl SymbolTable {
    /// Creates a new symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbols` - The symbols in the symbol table.
    ///
    /// # Returns
    ///
    /// * The new symbol table.
    #[must_use]
    pub const fn new(symbols: HashMap<String, Symbol>) -> Self {
        Self { symbols }
    }

    /// Adds a symbol to the symbol table.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to add.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of adding the symbol.
    ///
    /// # Errors
    ///
    /// * `SymbolAlreadyDefined` - The symbol is already defined.
    pub fn add_symbol(&mut self, symbol: Symbol) -> Result<(), Error> {
        if symbol.symbol_kind == SymbolKind::BuiltIn {
            return Ok(());
        }

        if self.symbols.contains_key(&symbol.name) {
            Err(Error::SymbolAlreadyDefined { name: symbol.name })
        } else {
            self.symbols.insert(symbol.name.clone(), symbol);

            Ok(())
        }
    }

    /// Sets a symbol in the symbol table, overwriting the previous symbol.
    ///
    /// # Arguments
    ///
    /// * `symbol` - The symbol to set.
    pub fn set_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    /// Gets a symbol from the symbol table.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol to get.
    ///
    /// # Returns
    ///
    /// * `Result<&Symbol, Error>` - The result of getting the symbol.
    ///
    /// # Errors
    ///
    /// * `SymbolNotDefined` - The symbol is not defined.
    pub fn get_symbol(&self, name: &str) -> Result<&Symbol, Error> {
        self.symbols
            .get(name)
            .ok_or_else(|| Error::SymbolNotDefined {
                name: name.to_string(),
            })
    }
}

/// A symbol is a name that is associated with a value.
///
/// # Fields
///
/// * `name` - The name of the symbol.
/// * `symbol_kind` - The kind of the symbol.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub symbol_kind: SymbolKind,
}

impl Symbol {
    /// Creates a new symbol.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the symbol.
    /// * `symbol_kind` - The kind of the symbol.
    ///
    /// # Returns
    ///
    /// * The new symbol.
    #[must_use]
    pub fn new(name: &str, symbol_kind: SymbolKind) -> Self {
        Self {
            name: name.to_string(),
            symbol_kind,
        }
    }
}

/// The kind of a symbol.
///
/// # Variants
///
/// * `BuiltIn` - A built-in symbol.
/// * `Variable` - A variable.
/// * `Function` - A function.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SymbolKind {
    BuiltIn,
    Variable,
    Function,
}

/// A scope is a region of the program where a binding of a name to an entity is valid.
///
/// # Variants
///
/// * `Root` - The root scope.
/// * `Child` - A child scope.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Scope {
    Root,
    Child(Box<Scope>),
}

impl Display for Scope {
    /// Formats the scope.
    ///
    /// # Arguments
    ///
    /// * `self` - The scope.
    ///
    /// # Returns
    ///
    /// * `String` - The formatted scope.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Root => write!(f, "global"),
            Self::Child(scope) => write!(f, "{scope}"),
        }
    }
}
