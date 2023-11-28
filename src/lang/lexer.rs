use crate::lang::errors::{Error, LexerError};
use std::{
    fmt::{Display, Formatter},
    path::Path,
};

/// The file extension source files must end with.
pub const FILE_EXTENSION: &str = "lt";

/// An enumeration of all the possible tokens in the language.
///
/// # Variants
///
/// * `LeftParenthesis`: A left parenthesis token, `(`.
/// * `RightParenthesis`: A right parenthesis token, `)`.
/// * `LeftCurlyBrace`: A left curly brace token, `{`.
/// * `RightCurlyBrace`: A right curly brace token, `}`.
/// * `Semicolon`: A semicolon token, `;`.
/// * `Comma`: A comma token, `,`.
/// * `Colon`: A colon token, `:`.
/// * `Plus`: A plus token, `+`.
/// * `Minus`: A minus token, `-`.
/// * `Star`: A star token, `*`.
/// * `Slash`: A slash token, `/`.
/// * `Percent`: A percent token, `%`.
/// * `BitwiseXor`: A bitwise XOR token, `^`.
/// * `BitwiseAnd`: A bitwise AND token, `&`.
/// * `BitwiseOr`: A bitwise OR token, `|`.
/// * `LogicalAnd`: A logical AND token, `&&`.
/// * `LogicalOr`: A logical OR token, `||`.
/// * `Bang`: A bang token, `!`.
/// * `BangEqual`: A bang equal token, `!=`.
/// * `Equal`: An equal token, `=`.
/// * `EqualEqual`: An equal equal token, `==`.
/// * `GreaterThan`: A greater than token, `>`.
/// * `GreaterThanOrEqual`: A greater than or equal token, `>=`.
/// * `LessThan`: A less than token, `<`.
/// * `LessThanOrEqual`: A less than or equal token, `<=`.
/// * `Increment`: An increment token, `++`.
/// * `Decrement`: A decrement token, `--`.
/// * `BitwiseLeftShift`: A bitwise left shift token, `<<`.
/// * `BitwiseRightShift`: A bitwise right shift token, `>>`.
/// * `BitwiseRightShiftEqual`: A bitwise right shift equal token, `>>=`.
/// * `BitwiseLeftShiftEqual`: A bitwise left shift equal token, `<<=`.
/// * `PlusEqual`: A plus equal token, `+=`.
/// * `MinusEqual`: A minus equal token, `-=`.
/// * `StarEqual`: A star equal token, `*=`.
/// * `SlashEqual`: A slash equal token, `/=`.
/// * `PercentEqual`: A percent equal token, `%=`.
/// * `BitwiseAndEqual`: A bitwise AND equal token, `&=`.
/// * `BitwiseOrEqual`: A bitwise OR equal token, `|=`.
/// * `BitwiseXorEqual`: A bitwise XOR equal token, `^=`.
/// * `Identifier`: An identifier token.
/// * `String`: A string token.
/// * `Number`: A number token.
/// * `If`: An if token, `if`.
/// * `Else`: An else token, `else`.
/// * `Switch`: A switch token, `switch`.
/// * `Case`: A case token, `case`.
/// * `Default`: A default token, `_`.
/// * `ExpressionArrow`: An expression arrow token, `=>`.
/// * `True`: A true token, `true`.
/// * `False`: A false token, `false`.
/// * `Arrow`: An arrow token, `->`.
/// * `Return`: A return token, `return`.
/// * `While`: A while token, `while`.
/// * `For`: A for token, `for`.
/// * `In`: An in token, `in`.
/// * `To`: A to token, `to`.
/// * `Break`: A break token, `break`.
/// * `Continue`: A continue token, `continue`.
/// * `Function`: A function token, `fn`.
/// * `Variable`: A variable token, `let`.
/// * `Constant`: A constant token, `const`.
/// * `EndOfFile`: An end of file token.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TokenType {
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    Semicolon,
    Comma,
    Colon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    BitwiseXor,
    BitwiseAnd,
    BitwiseOr,
    LogicalAnd,
    LogicalOr,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Increment,
    Decrement,
    BitwiseLeftShift,
    BitwiseRightShift,
    BitwiseRightShiftEqual,
    BitwiseLeftShiftEqual,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentEqual,
    BitwiseAndEqual,
    BitwiseOrEqual,
    BitwiseXorEqual,
    Identifier,
    String,
    Number,
    If,
    Else,
    Switch,
    Case,
    Default,
    ExpressionArrow,
    True,
    False,
    Arrow,
    Return,
    While,
    For,
    In,
    To,
    Break,
    Continue,
    Function,
    Variable,
    Constant,
    EndOfFile,
}

/// Representation of a literal.
///
/// # Variants
///
/// * `String(String)`: A string literal.
/// * `Number(f64)`: A number literal.
/// * `Boolean(bool)`: A boolean literal.
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
}

impl Eq for Literal {
    fn assert_receiver_is_total_eq(&self) {}
}

impl Display for Literal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(string) => write!(f, "{string}"),
            Self::Number(number) => write!(f, "{number}"),
            Self::Boolean(boolean) => write!(f, "{boolean}"),
        }
    }
}

/// Representation of a token, with its type, lexeme, literal, line, and column.
///
/// # Fields
///
/// * `token_type`: The type of the token.
/// * `lexeme`: The value of the token.
/// * `literal`: The literal value of the token, if any.
///
/// * `line`: What line the token exists at.
/// * `column`: What column the token exists at.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,

    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Arguments
    ///
    /// * `token_type`: The type of the token.
    /// * `lexeme`: The value of the token.
    /// * `literal`: The literal value of the token, if any.
    ///
    /// * `line`: What line the token exists at.
    /// * `column`: What column the token exists at.
    ///
    /// # Returns
    ///
    /// * `Token`: The new token.
    pub fn new(
        token_type: TokenType,
        lexeme: &str,
        literal: Option<Literal>,

        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme: lexeme.to_string(),
            literal,

            line,
            column,
        }
    }
}

/// Representation of a lexer.
///
/// # Fields
///
/// * `source`: The source code to scan.
/// * `tokens`: The vector of tokens.
///
/// * `start`: The start of the current token.
/// * `current`: The current character.
///
/// * `line`: The current line.
/// * `column`: The current column.
pub struct Lexer {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,

    line: usize,
    column: usize,
}

impl Lexer {
    /// Creates a new lexer.
    ///
    /// # Arguments
    ///
    /// * `source`: The source code to scan.
    ///
    /// # Returns
    ///
    /// * `Lexer`: The new lexer.
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            tokens: Vec::new(),

            start: 0,
            current: 0,

            line: 1,
            column: 1,
        }
    }

    /// Scans the source code and returns a vector of tokens.
    ///
    /// # Returns
    ///
    /// * `Result<&Vec<Token>, Error>`: The vector of tokens, or an error.
    ///
    /// # Errors
    ///
    /// * If there are no more tokens to scan.
    /// * Lexer tried to advance past the end of the source code.
    /// * If an unexpected character appears.
    pub fn scan_tokens(&mut self) -> Result<&Vec<Token>, Error> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token::new(
            TokenType::EndOfFile,
            "",
            None,
            self.line,
            self.column,
        ));

        Ok(&self.tokens)
    }

    /// Scans a single token.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>`: The result of the operation.
    ///
    /// # Errors
    ///
    /// * If there are no more tokens to scan.
    /// * Lexer tried to advance past the end of the source code.
    /// * If an unexpected character appears.
    fn scan_token(&mut self) -> Result<(), Error> {
        let value = self
            .advance()
            .ok_or(Error::Lexer(LexerError::UnexpectedEoF(
                "No more tokens to scan!".into(),
            )))?;

        // Check if it's a single-character token.
        if let Ok(token_type) = Self::get_single_char_token(value) {
            return self.add_token(token_type);
        };
        // Check if it's a multi-character token.
        if let Ok(token_type) = self.get_multi_char_token(value) {
            return self.add_token(token_type);
        };

        match value {
            // Literals.
            '"' => self.string()?,
            '0'..='9' => self.number()?,
            'a'..='z' | 'A'..='Z' | '_' => self.identifier()?,

            // Whitespace.
            ' ' | '\r' | '\t' => (),
            '\n' => self.new_line(),
            '/' => {
                if self.match_char('/') {
                    // Single-line comments.
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_char('*') {
                    // Multi-line comments.
                    while self.peek_next(1) != Some("*/".into()) && !self.is_at_end() {
                        if self.peek() == Some('\n') {
                            self.new_line();
                        }

                        self.advance();
                    }

                    if self.is_at_end() {
                        return Err(Error::Lexer(LexerError::UnexpectedEoF(
                            "Lexer tried to advance past the end of the source code!".into(),
                        )));
                    }

                    self.advance();
                    self.advance();
                } else if self.match_char('=') {
                    // Division assignment.
                    self.add_token(TokenType::SlashEqual)?;
                } else {
                    // Division.
                    self.add_token(TokenType::Slash)?;
                }
            }
            _ => return Err(Error::Lexer(LexerError::UnexpectedCharacter(value))),
        }

        Ok(())
    }

    /// Extracts a single-character token from the source code.
    ///
    /// # Arguments
    ///
    /// * `value` - The current character.
    ///
    /// # Returns
    ///
    /// * `Result<TokenType, Error>`: The type of the token, or an error.
    ///
    /// # Errors
    ///
    /// * If the character is not a valid single-character token.
    const fn get_single_char_token(value: char) -> Result<TokenType, Error> {
        let token_type = match value {
            '(' => TokenType::LeftParenthesis,
            ')' => TokenType::RightParenthesis,
            '{' => TokenType::LeftCurlyBrace,
            '}' => TokenType::RightCurlyBrace,
            ':' => TokenType::Colon,
            ';' => TokenType::Semicolon,
            ',' => TokenType::Comma,
            _ => return Err(Error::Lexer(LexerError::UnexpectedCharacter(value))),
        };

        Ok(token_type)
    }

    /// Extracts a multi-character token from the source code.
    ///
    /// # Arguments
    ///
    /// * `value` - The current character.
    ///
    /// # Returns
    ///
    /// * `Result<TokenType, Error>`: The type of the token, or an error.
    ///
    /// # Errors
    ///
    /// * If the character is not a valid multi-character token.
    fn get_multi_char_token(&mut self, value: char) -> Result<TokenType, Error> {
        let token_type = match value {
            '!' => {
                if self.match_char('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_char('=') {
                    TokenType::EqualEqual
                } else if self.match_char('>') {
                    TokenType::ExpressionArrow
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_char('=') {
                    TokenType::LessThanOrEqual
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        TokenType::BitwiseLeftShiftEqual
                    } else {
                        TokenType::BitwiseLeftShift
                    }
                } else {
                    TokenType::LessThan
                }
            }
            '>' => {
                if self.match_char('=') {
                    TokenType::GreaterThanOrEqual
                } else if self.match_char('>') {
                    if self.match_char('=') {
                        TokenType::BitwiseRightShiftEqual
                    } else {
                        TokenType::BitwiseRightShift
                    }
                } else {
                    TokenType::GreaterThan
                }
            }
            '+' => {
                if self.match_char('+') {
                    TokenType::Increment
                } else if self.match_char('=') {
                    TokenType::PlusEqual
                } else {
                    TokenType::Plus
                }
            }
            '-' => {
                if self.match_char('-') {
                    TokenType::Decrement
                } else if self.match_char('=') {
                    TokenType::MinusEqual
                } else if self.match_char('>') {
                    TokenType::Arrow
                } else {
                    TokenType::Minus
                }
            }
            '*' => {
                if self.match_char('=') {
                    TokenType::StarEqual
                } else {
                    TokenType::Star
                }
            }
            '%' => {
                if self.match_char('=') {
                    TokenType::PercentEqual
                } else {
                    TokenType::Percent
                }
            }
            '&' => {
                if self.match_char('&') {
                    TokenType::LogicalAnd
                } else if self.match_char('=') {
                    TokenType::BitwiseAndEqual
                } else {
                    TokenType::BitwiseAnd
                }
            }
            '|' => {
                if self.match_char('|') {
                    TokenType::LogicalOr
                } else if self.match_char('=') {
                    TokenType::BitwiseOrEqual
                } else {
                    TokenType::BitwiseOr
                }
            }
            '^' => {
                if self.match_char('=') {
                    TokenType::BitwiseXorEqual
                } else {
                    TokenType::BitwiseXor
                }
            }
            _ => return Err(Error::Lexer(LexerError::UnexpectedCharacter(value))),
        };

        Ok(token_type)
    }

    /// Advances the lexer by one character.
    ///
    /// # Returns
    ///
    /// * `Option<char>`: The character that was advanced to, if any.
    fn advance(&mut self) -> Option<char> {
        self.current += 1;
        self.column += 1;

        self.source.chars().nth(self.current - 1)
    }

    /// Adds a token to the vector of tokens.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>`: The result of the operation.
    ///
    /// # Errors
    ///
    /// * If the token cannot be converted to a number literal.
    fn add_token(&mut self, token_type: TokenType) -> Result<(), Error> {
        let text = &self.source[self.start..self.current];
        let literal = match token_type {
            TokenType::String => Some(Literal::String(text.to_string())),
            TokenType::Number => Some(Literal::Number(text.parse()?)),
            TokenType::True => Some(Literal::Boolean(true)),
            TokenType::False => Some(Literal::Boolean(false)),
            _ => None,
        };

        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line,
            self.column,
        ));

        Ok(())
    }

    /// Checks if the next character matches the given character.
    ///
    /// # Arguments
    ///
    /// * `expected` - The character to match.
    ///
    /// # Returns
    ///
    /// * `bool`: True if the next character matches the given character, false otherwise.
    fn match_char(&mut self, expected: char) -> bool {
        // If we've reached the end of the file, we cannot match the character.
        if self.is_at_end() {
            return false;
        }

        // If the current character doesn't match the expected character, it fails.
        if self.source.chars().nth(self.current) != Some(expected) {
            return false;
        }

        // Increment the current character and column because we've now checked the character.
        self.current += 1;
        self.column += 1;

        true
    }

    /// Checks if we are at the end of the source code.
    ///
    /// # Returns
    ///
    /// * `bool`: True if the current character is greater than or equal to the length of the source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Peeks at the current character without advancing the lexer.
    ///
    /// # Returns
    ///
    /// * `Option<char>`: The current character, if any.
    fn peek(&self) -> Option<char> {
        self.source.chars().nth(self.current)
    }

    /// Peeks at the next `n` characters after the current character without advancing the lexer.
    ///
    /// # Arguments
    ///
    /// * `n`: How far to peek into the source.
    ///
    /// # Returns
    ///
    /// * `Option<String>`: The next `n` characters, if any.
    fn peek_next(&self, n: usize) -> Option<String> {
        (self.current..=n)
            .map(|i| self.source.chars().nth(i))
            .filter(Option::is_some)
            .collect()
    }

    /// Lexes a string token.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>`: The result of the operation.
    ///
    /// # Errors
    ///
    /// * If the string is unterminated.
    /// * If we cannot get the last token.
    fn string(&mut self) -> Result<(), Error> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\\') {
                self.advance();
                self.column += 1;
            }

            if self.peek() == Some('\n') {
                self.new_line();
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(Error::Lexer(LexerError::UnterminatedString));
        }

        self.advance();

        self.add_token(TokenType::String)?;
        let value = &self.source[self.start + 1..self.current - 1];

        // Set the last token to the string literal.
        self.tokens
            .last_mut()
            .ok_or(Error::Lexer(LexerError::InvalidToken(
                "Failed to get last token!".into(),
            )))?
            .literal = Some(Literal::String(value.to_string()));

        Ok(())
    }

    /// Lexes a number token.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>`: The result of the operation.
    ///
    /// # Errors
    ///
    /// * If the lexer fails to peek at the current, next or last token.
    /// * If the number fails to parse.
    fn number(&mut self) -> Result<(), Error> {
        // Keep reading until the number ends.
        let start = self.current - 1;
        loop {
            let Some(next) = self.peek() else {
                break;
            };
            if !next.is_ascii_digit() && next != '.' {
                break;
            }

            self.advance();
        }

        self.add_token(TokenType::Number)?;
        let value = &self.source[start..self.current];

        // Set the last token to the number literal.
        self.tokens
            .last_mut()
            .ok_or(Error::Lexer(LexerError::InvalidToken(
                "Failed to get last token!".into(),
            )))?
            .literal = Some(Literal::Number(value.parse()?));

        Ok(())
    }

    /// Lexes an identifier.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>`: The result of the operation.
    ///
    /// # Errors
    ///
    /// * If the lexer fails to peek at the current token.
    fn identifier(&mut self) -> Result<(), Error> {
        // While the current token is alphanumeric or an _, we are reading an identifier.
        while self
            .peek()
            .ok_or(Error::Lexer(LexerError::InvalidToken(
                "Failed to peek at current token!".into(),
            )))?
            .is_alphanumeric()
            || self.peek() == Some('_')
        {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = match text {
            "fn" => TokenType::Function,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "switch" => TokenType::Switch,
            "case" => TokenType::Case,
            "_" => TokenType::Default,
            "while" => TokenType::While,
            "continue" => TokenType::Continue,
            "break" => TokenType::Break,
            "for" => TokenType::For,
            "in" => TokenType::In,
            "to" => TokenType::To,
            "str" => TokenType::String,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "return" => TokenType::Return,
            "let" => TokenType::Variable,
            "const" => TokenType::Constant,
            _ => TokenType::Identifier,
        };

        self.add_token(token_type)?;

        Ok(())
    }

    /// Increments the [`line`] count and sets the [`column`] back to `1`.
    fn new_line(&mut self) {
        self.line += 1;
        self.column = 1;
    }
}

/// Reads the contents of a given file.
///
/// # Arguments
///
/// * `path`: The path of the file.
///
/// # Returns
///
/// * `Result<String, Error>`: The contents of the file, or an error.
///
/// # Errors
///
/// * If the file doesn't have a file extension.
/// * If we cannot convert the file extension to a `&str`.
/// * If the file extension doesn't match [`FILE_EXTENSION`].
pub fn read_file(path: &str) -> Result<String, Error> {
    // Get the file extension.
    let extension = Path::new(path)
        .extension()
        .ok_or(Error::Lexer(LexerError::InvalidFile(
            "{path} doesn't have a file extension!".into(),
        )))?
        .to_str()
        .ok_or(Error::Lexer(LexerError::InvalidFile(
            "Failed to convert extension to string slice!".into(),
        )))?;

    // Check if the file extension is valid.
    if extension != FILE_EXTENSION {
        return Err(Error::Lexer(LexerError::InvalidFile(format!(
            "{path} isn't a valid {FILE_EXTENSION} file, found {extension}!"
        ))));
    }

    // Read the file.
    let contents = std::fs::read_to_string(path)?;

    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::expect_used, clippy::cognitive_complexity)]
    fn test_scan_tokens() {
        let source =
            "let a = 1 + 2 - 3 * 4 / 5 == 6 != 7 < 8 <= 9 > 10 >= 11 % 12 && true != false;";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().expect("Failed to scan tokens!");

        assert_eq!(tokens.len(), 31 + 1); // 31 tokens, plus EOF.

        assert_eq!(tokens[0].token_type, TokenType::Variable); // let
        assert_eq!(tokens[1].token_type, TokenType::Identifier); // a
        assert_eq!(tokens[2].token_type, TokenType::Equal); // =
        assert_eq!(tokens[3].token_type, TokenType::Number); // 1
        assert_eq!(tokens[4].token_type, TokenType::Plus); // +
        assert_eq!(tokens[5].token_type, TokenType::Number); // 2
        assert_eq!(tokens[6].token_type, TokenType::Minus); // -
        assert_eq!(tokens[7].token_type, TokenType::Number); // 3
        assert_eq!(tokens[8].token_type, TokenType::Star); // *
        assert_eq!(tokens[9].token_type, TokenType::Number); // 4
        assert_eq!(tokens[10].token_type, TokenType::Slash); // /
        assert_eq!(tokens[11].token_type, TokenType::Number); // 5
        assert_eq!(tokens[12].token_type, TokenType::EqualEqual); // ==
        assert_eq!(tokens[13].token_type, TokenType::Number); // 6
        assert_eq!(tokens[14].token_type, TokenType::BangEqual); // !=
        assert_eq!(tokens[15].token_type, TokenType::Number); // 7
        assert_eq!(tokens[16].token_type, TokenType::LessThan); // <
        assert_eq!(tokens[17].token_type, TokenType::Number); // 8
        assert_eq!(tokens[18].token_type, TokenType::LessThanOrEqual); // <=
        assert_eq!(tokens[19].token_type, TokenType::Number); // 9
        assert_eq!(tokens[20].token_type, TokenType::GreaterThan); // >
        assert_eq!(tokens[21].token_type, TokenType::Number); // 10
        assert_eq!(tokens[22].token_type, TokenType::GreaterThanOrEqual); // >=
        assert_eq!(tokens[23].token_type, TokenType::Number); // 11
        assert_eq!(tokens[24].token_type, TokenType::Percent); // %
        assert_eq!(tokens[25].token_type, TokenType::Number); // 12
        assert_eq!(tokens[26].token_type, TokenType::LogicalAnd); // &&
        assert_eq!(tokens[27].token_type, TokenType::True); // true
        assert_eq!(tokens[28].token_type, TokenType::BangEqual); // !=
        assert_eq!(tokens[29].token_type, TokenType::False); // false
        assert_eq!(tokens[30].token_type, TokenType::Semicolon); // ;
        assert_eq!(tokens[31].token_type, TokenType::EndOfFile); // EOF
    }

    #[test]
    #[allow(clippy::expect_used, clippy::cognitive_complexity)]
    fn test_scan_tokens_with_strings() {
        let source = r#"
            let a = "Hello, world!";
            let b = "Hello, \"world\"!";
            let c = "Hello, \nworld!";
            let d = "Hello, \tworld!";
            let e = "Hello, \\world!";
            let f = "Hello, \rworld!";
            let g = "Hello, \0world!";
            let h = "Hello, \x00world!";
        "#;
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().expect("Failed to scan tokens!");

        assert_eq!(tokens.len(), 8 * 5 + 1); // 8 lines, 5 tokens per line, plus EOF.

        assert_eq!(tokens[0].token_type, TokenType::Variable); // let
        assert_eq!(tokens[1].token_type, TokenType::Identifier); // a
        assert_eq!(tokens[2].token_type, TokenType::Equal); // =
        assert_eq!(tokens[3].token_type, TokenType::String); // "Hello, world!"
        assert_eq!(tokens[4].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[5].token_type, TokenType::Variable); // let
        assert_eq!(tokens[6].token_type, TokenType::Identifier); // b
        assert_eq!(tokens[7].token_type, TokenType::Equal); // =
        assert_eq!(tokens[8].token_type, TokenType::String); // "Hello, \"world\"!"
        assert_eq!(tokens[9].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[10].token_type, TokenType::Variable); // let
        assert_eq!(tokens[11].token_type, TokenType::Identifier); // c
        assert_eq!(tokens[12].token_type, TokenType::Equal); // =
        assert_eq!(tokens[13].token_type, TokenType::String); // "Hello, \nworld!"
        assert_eq!(tokens[14].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[15].token_type, TokenType::Variable); // let
        assert_eq!(tokens[16].token_type, TokenType::Identifier); // d
        assert_eq!(tokens[17].token_type, TokenType::Equal); // =
        assert_eq!(tokens[18].token_type, TokenType::String); // "Hello, \tworld!"
        assert_eq!(tokens[19].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[20].token_type, TokenType::Variable); // let
        assert_eq!(tokens[21].token_type, TokenType::Identifier); // e
        assert_eq!(tokens[22].token_type, TokenType::Equal); // =
        assert_eq!(tokens[23].token_type, TokenType::String); // "Hello, \\world!"
        assert_eq!(tokens[24].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[25].token_type, TokenType::Variable); // let
        assert_eq!(tokens[26].token_type, TokenType::Identifier); // f
        assert_eq!(tokens[27].token_type, TokenType::Equal); // =
        assert_eq!(tokens[28].token_type, TokenType::String); // "Hello, \rworld!"
        assert_eq!(tokens[29].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[30].token_type, TokenType::Variable); // let
        assert_eq!(tokens[31].token_type, TokenType::Identifier); // g
        assert_eq!(tokens[32].token_type, TokenType::Equal); // =
        assert_eq!(tokens[33].token_type, TokenType::String); // "Hello, \0world!"
        assert_eq!(tokens[34].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[35].token_type, TokenType::Variable); // let
        assert_eq!(tokens[36].token_type, TokenType::Identifier); // h
        assert_eq!(tokens[37].token_type, TokenType::Equal); // =
        assert_eq!(tokens[38].token_type, TokenType::String); // "Hello, \x00world!"
        assert_eq!(tokens[39].token_type, TokenType::Semicolon); // ;

        assert_eq!(tokens[40].token_type, TokenType::EndOfFile); // EOF
    }
}
