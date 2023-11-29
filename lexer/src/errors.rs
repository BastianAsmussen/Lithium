use thiserror::Error;

/// An lexing error.
///
/// # Variants
///
/// * `InvalidToken` - An invalid token.
/// * `UnexpectedCharacter` - An unexpected character.
/// * `UnterminatedString` - An unterminated string.
/// * `ParseError` - A number parsing error.
/// * `IoError` - An IO error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("Invalid token at line {line}, column {column}!")]
    InvalidToken { line: usize, column: usize },
    #[error("Unexpected character '{char}' at line {line}, column {column}!", char = char.unwrap_or(' '))]
    UnexpectedCharacter {
        char: Option<char>,
        line: usize,
        column: usize,
    },
    #[error("Unterminated string at line {line}, column {column}!")]
    UnterminatedString { line: usize, column: usize },
    #[error("Failed to parse number!")]
    ParseError(#[from] std::num::ParseIntError),
    #[error("IO error!")]
    IoError(#[from] std::io::Error),
}
