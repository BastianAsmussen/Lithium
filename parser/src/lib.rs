use lexer::token::Token;

/// A parser for generating an AST from a stream of tokens.
///
/// # Fields
///
/// * `tokens` - The stream of tokens to parse.
/// * `current` - The current index of the token stream.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser from a stream of tokens.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The stream of tokens to parse.
    ///
    /// # Returns
    ///
    /// * `Self` - The new parser.
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }
}
