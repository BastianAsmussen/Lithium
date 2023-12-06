use crate::semantics::symbols::Scope;
use thiserror::Error;

/// A semantic error.
///
/// # Variants
///
/// * `SymbolNotDefined` - A symbol is not defined.
/// * `SymbolAlreadyDefined` - A symbol is already defined.
/// * `ScopeNotDefined` - A scope is not defined.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Symbol '{name}' is not defined!")]
    SymbolNotDefined { name: String },
    #[error("Symbol '{name}' is already defined!")]
    SymbolAlreadyDefined { name: String },
    #[error("Scope '{scope}' is not defined!")]
    ScopeNotDefined { scope: Scope },
}
