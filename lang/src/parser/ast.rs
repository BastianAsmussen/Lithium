use crate::parser::Statement;

/// The abstract syntax tree.
///
/// # Fields
///
/// * `statements` - The statements in the AST.
#[derive(Debug, PartialEq)]
pub struct AST {
    pub statements: Vec<Statement>,
}

impl AST {
    #[must_use]
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }
}
