use thiserror::Error;

/// A compile-time error.
///
/// # Variants
///
/// * `IO`: An IO error occurred.
/// * `Parse`: A number failed to parse.
/// * `Lexer(LexerError)`: A lexer error.
#[derive(Debug, Error)]
pub enum Error {
    #[error("IO Error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Parse Error: {0}")]
    Parse(#[from] std::num::ParseFloatError),
    #[error("Lexer Error: {0}")]
    Lexer(LexerError),
}

/// A lexer error.
///
/// # Variants
///
/// * `UnexpectedEoF(String)`: The lexer found an unexpected End of File.
/// * `UnexpectedCharacter(char)`: An unexpected character was found.
/// * `UnterminatedString`: A string was found to never end.
/// * `InvalidToken(String)`: The lexer fails to get a token of some sort.
/// * `InvalidFile(String)`: A given file is invalid.
#[derive(Debug, Error)]
pub enum LexerError {
    #[error("Unexpected End of File: {0}")]
    UnexpectedEoF(String),
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Unterminated string!")]
    UnterminatedString,
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Invalid file: {0}")]
    InvalidFile(String),
}
