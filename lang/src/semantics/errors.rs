use thiserror::Error;

/// A semantic error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("The scope is invalid!")]
    InvalidScope,
    #[error("The symbol '{name}' is undefined at line {line} and column {column}!")]
    UndefinedSymbol {
        name: String,
        line: usize,
        column: usize,
    },
    #[error("Cannot assign to '{name}' at line {line} and column {column}!")]
    InvalidAssignment {
        name: String,
        line: usize,
        column: usize,
    },
    #[error(
        "The operator '{operator}' is not a valid operator at line {line} and column {column}!"
    )]
    InvalidOperator {
        operator: String,
        line: usize,
        column: usize,
    },
    #[error("The variable '{name}' hasn't been initialized at line {line} and column {column}!")]
    UninitializedVariable {
        name: String,
        line: usize,
        column: usize,
    },
    #[error("The variable '{name}' is invalid at line {line} and column {column}!")]
    InvalidVariable {
        name: String,
        line: usize,
        column: usize,
    },
    #[error("The parameter '{name}' is invalid at line {line} and column {column}!")]
    InvalidParameterKind {
        name: String,
        line: usize,
        column: usize,
    },
}
