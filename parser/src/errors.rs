use thiserror::Error;

/// An parsing error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid token at line {line}, column {column}! ({message})")]
    InvalidToken {
        message: String,
        line: usize,
        column: usize,
    },
}
