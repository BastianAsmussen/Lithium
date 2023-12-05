use std::num::ParseFloatError;
use thiserror::Error;

/// A parsing error.
///
/// # Variants
///
/// * `UnexpectedToken` - An unexpected token was encountered.
/// * `FunctionParameterLength` - A function has too many parameters.
/// * `InvalidAssignmentTarget` - An invalid assignment target was encountered.
/// * `ParseFloat` - Failed to parse a float.
/// * `UnexpectedEndOfFile` - Unexpected end of file.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Unexpected token at line {line} and column {column}: {message}")]
    UnexpectedToken {
        line: usize,
        column: usize,
        message: String,
    },
    #[error("Invalid function parameter length at line {line} and column {column}")]
    FunctionParameterLength { line: usize, column: usize },
    #[error("Invalid assignment target at line {line} and column {column}!")]
    InvalidAssignmentTarget { line: usize, column: usize },
    #[error("Failed to parse float!")]
    ParseFloat(#[from] ParseFloatError),
    #[error("Unexpected end of file!")]
    UnexpectedEndOfFile,
}
