use thiserror::Error;

/// An lexing error.
///
/// # Variants
///
/// * `InvalidToken` - An invalid token.
/// * `UnexpectedCharacter` - An unexpected character.
/// * `UnterminatedString` - An unterminated string.
/// * `ParseIntError` - An integer parsing error.
/// * `ParseFloatError` - A float parsing error.
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
    #[error("Failed to parse integer!")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Failed to parse float!")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("I/O error!")]
    IoError(#[from] std::io::Error),
    #[error("Invalid file extension!")]
    InvalidFileExtension,
    #[error("File doesn't exist!")]
    InvalidFilePath,
}
