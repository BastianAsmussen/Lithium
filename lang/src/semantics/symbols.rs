use crate::lexer::tokens::Token;

/// A symbol kind which defines the type of a symbol.
///
/// # Variants
///
/// * `Variable` - A variable.
/// * `Function` - A function.
#[derive(Debug)]
pub enum SymbolKind {
    Variable {
        is_initialized: bool,
    },
    Function {
        return_type: Option<Token>,
    },
}
