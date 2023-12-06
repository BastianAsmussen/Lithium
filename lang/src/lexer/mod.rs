use crate::lexer::tokens::{Token, TokenKind};
use errors::Error;

pub mod errors;
pub mod tokens;

static ALLOWED_NUMBER_CHARS: &[char] = &['+', '-', 'e', 'E', '.', 'b', 'B', 'o', 'O', 'x', 'X'];

/// A lexer.
///
/// # Fields
///
/// * `input` - The input to lex.
/// * `position` - The current position in the input.
/// * `line` - The current line.
/// * `column` - The current column.
#[derive(Debug, Clone)]
pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    /// Creates a new lexer.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to lex.
    ///
    /// # Returns
    ///
    /// * `Lexer<'a>` - The lexer.
    #[must_use]
    pub fn new(input: &'a str) -> Self {
        Self {
            input: input.trim(), // Sanitize the input.
            position: 0,
            line: 1,
            column: 0,
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.chars().nth(self.position)
    }

    fn next_char(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn advance(&mut self) {
        if self.current_char() == Some('\n') {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.position += 1;
    }

    fn read_identifier(&mut self) -> Token {
        let mut identifier = String::new();
        while let Some(c) = self.current_char() {
            if !c.is_alphanumeric() && c != '_' {
                break;
            }

            self.advance();

            identifier.push(c);
        }

        let token_kind = match identifier.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::Range, // "in" is a keyword because it's used in "for" loops.
            "to" => TokenKind::To,    // "to" is a keyword because it's used in "for" loops.
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "fn" => TokenKind::Function,
            "let" => TokenKind::Variable,
            _ => TokenKind::Identifier(identifier),
        };

        Token::new(token_kind, self.line, self.column)
    }

    fn read_number(&mut self) -> Result<Option<Token>, Error> {
        let is_positive = self.current_char() == Some('+');
        let is_negative = self.current_char() == Some('-');
        if is_positive || is_negative {
            if let Some(c) = self.next_char() {
                if !c.is_numeric() {
                    return Ok(None);
                }
            }

            self.advance();
        }

        let mut number = String::new();
        while let Some(c) = self.current_char() {
            if !c.is_numeric() && !ALLOWED_NUMBER_CHARS.contains(&c) {
                break;
            }

            self.advance();

            number.push(c);
        }

        // Make sure the number isn't empty.
        let number = number.trim();
        if number.is_empty() {
            return Err(Error::UnexpectedCharacter {
                char: self.current_char(),
                line: self.line,
                column: self.column,
            });
        }

        let token_kind = if number.contains('.') || number.contains('e') || number.contains('E') {
            let number = number.parse::<f64>()?;

            TokenKind::Float(number)
        } else if number.contains('b') || number.contains('B') {
            let number = i64::from_str_radix(&number[2..], 2)?;

            TokenKind::Integer(number)
        } else if number.contains('o') || number.contains('O') {
            let number = i64::from_str_radix(&number[2..], 8)?;

            TokenKind::Integer(number)
        } else if number.contains('x') || number.contains('X') {
            let number = i64::from_str_radix(&number[2..], 16)?;

            TokenKind::Integer(number)
        } else {
            let number = number.parse::<i64>()?;

            TokenKind::Integer(number)
        };

        // If the number is negative, then we need to negate it.
        let token_kind = if is_negative {
            match token_kind {
                TokenKind::Integer(number) => TokenKind::Integer(-number),
                TokenKind::Float(number) => TokenKind::Float(-number),
                _ => unreachable!(),
            }
        } else {
            token_kind
        };

        Ok(Some(Token::new(token_kind, self.line, self.column)))
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        self.advance();

        let mut string = String::new();
        while let Some(c) = self.current_char() {
            if c == '\\' {
                // Escape character.
                self.advance();

                let Some(c) = self.current_char() else {
                    return Err(Error::UnterminatedString {
                        line: self.line,
                        column: self.column,
                    });
                };

                match c {
                    'n' => string.push('\n'),
                    'r' => string.push('\r'),
                    't' => string.push('\t'),
                    '\\' => string.push('\\'),
                    '"' => string.push('"'),
                    _ => {
                        return Err(Error::UnexpectedCharacter {
                            char: Some(c),
                            line: self.line,
                            column: self.column,
                        })
                    }
                }

                self.advance();
                continue;
            }

            if c == '"' {
                self.advance();

                return Ok(Token::new(
                    TokenKind::String(string),
                    self.line,
                    self.column,
                ));
            }

            string.push(c);
            self.advance();
        }

        Err(Error::UnterminatedString {
            line: self.line,
            column: self.column,
        })
    }

    fn handle_plus(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::AddAssign
            }
            Some('+') => {
                self.advance();

                TokenKind::Increment
            }
            _ => TokenKind::Plus,
        }
    }

    fn handle_minus(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::SubtractAssign
            }
            Some('-') => {
                self.advance();

                TokenKind::Decrement
            }
            Some('>') => {
                self.advance();

                TokenKind::Arrow
            }
            _ => TokenKind::Minus,
        }
    }

    fn handle_asterisk(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::MultiplyAssign
            }
            Some('*') => {
                self.advance();

                TokenKind::Power
            }
            _ => TokenKind::Star,
        }
    }

    fn handle_slash(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::DivisionAssign
            }
            Some('/') => {
                // Single-line comment, ignore until a new-line.
                while let Some(c) = self.current_char() {
                    if c == '\n' {
                        break;
                    }

                    self.advance();
                }

                TokenKind::Comment
            }
            Some('*') => {
                self.advance();
                self.advance();

                // Multi-line comment, ignore until a */ is found.
                while self.current_char() != Some('*') && self.next_char() != Some('/') {
                    // Advance to the next character.
                    self.advance();
                }

                // Skip over the */ by advancing twice.
                self.advance();
                self.advance();

                TokenKind::Comment
            }
            _ => TokenKind::Slash,
        }
    }

    fn handle_percent(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::ModuloAssign
            }
            _ => TokenKind::Percent,
        }
    }

    fn handle_caret(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::BitwiseXorAssign
            }
            _ => TokenKind::BitwiseXor,
        }
    }

    fn handle_bang(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::NotEqual
            }
            _ => TokenKind::LogicalNot,
        }
    }

    fn handle_equal(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::Equality
            }
            _ => TokenKind::Assign,
        }
    }

    fn handle_less_than(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::LessThanOrEqual
            }
            Some('<') => {
                self.advance();

                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::BitwiseLeftShiftAssign
                } else {
                    TokenKind::BitwiseLeftShift
                }
            }
            _ => TokenKind::LessThan,
        }
    }

    fn handle_greater_than(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::GreaterThanOrEqual
            }
            Some('>') => {
                self.advance();

                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::BitwiseRightShiftAssign
                } else {
                    TokenKind::BitwiseRightShift
                }
            }
            _ => TokenKind::GreaterThan,
        }
    }

    fn handle_ampersand(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::BitwiseAndAssign
            }
            Some('&') => {
                self.advance();

                TokenKind::LogicalAnd
            }
            _ => TokenKind::BitwiseAnd,
        }
    }

    fn handle_pipe(&mut self) -> TokenKind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                TokenKind::BitwiseOrAssign
            }
            Some('|') => {
                self.advance();

                TokenKind::LogicalOr
            }
            _ => TokenKind::BitwiseOr,
        }
    }

    fn read_operator(&mut self) -> Result<Token, Error> {
        let current_char = self.current_char();
        let kind = match current_char {
            Some('+') => self.handle_plus(),
            Some('-') => self.handle_minus(),
            Some('*') => self.handle_asterisk(),
            Some('/') => self.handle_slash(),
            Some('%') => self.handle_percent(),
            Some('^') => self.handle_caret(),
            Some('!') => self.handle_bang(),
            Some('=') => self.handle_equal(),
            Some('<') => self.handle_less_than(),
            Some('>') => self.handle_greater_than(),
            Some('&') => self.handle_ampersand(),
            Some('|') => self.handle_pipe(),
            Some('(') => TokenKind::LeftParenthesis,
            Some(')') => TokenKind::RightParenthesis,
            Some('{') => TokenKind::LeftCurlyBrace,
            Some('}') => TokenKind::RightCurlyBrace,
            Some('[') => TokenKind::LeftBracket,
            Some(']') => TokenKind::RightBracket,
            Some(',') => TokenKind::Comma,
            Some('.') => TokenKind::Dot,
            Some(':') => TokenKind::Colon,
            Some(';') => TokenKind::Semicolon,
            _ => {
                return Err(Error::UnexpectedCharacter {
                    char: current_char,
                    line: self.line,
                    column: self.column,
                })?
            }
        };

        let token = Token::new(kind, self.line, self.column);

        // Advance to the next character.
        self.advance();

        Ok(token)
    }

    fn skip_whitespace(&mut self) {
        while let Some(c) = self.current_char() {
            if !c.is_whitespace() {
                break;
            }

            self.advance();
        }
    }

    fn read_token(&mut self) -> Result<Token, Error> {
        // Skip whitespace.
        self.skip_whitespace();

        let current_char = self.current_char();
        let token = match current_char {
            Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),
            Some(c) if c.is_numeric() || c == '+' || c == '-' => {
                // If we receive a None, then we know it's not a number, so we can read an operator.
                let Some(token) = self.read_number()? else {
                    return self.read_operator();
                };

                token
            }
            Some('"') => self.read_string()?,
            None => Token::new(TokenKind::EndOfFile, self.line, self.column),
            _ => self.read_operator()?,
        };

        Ok(token)
    }

    const fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }

    /// Tokenizes the input.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Token>, Error>` - The tokens, or an error.
    ///
    /// # Errors
    ///
    /// * If an invalid token is found.
    /// * If an unterminated string is found.
    /// * If a number fails to parse.
    /// * If an unexpected character is found.
    pub fn tokenize(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        while !self.is_at_end() {
            let token = self.read_token()?;
            if token.token_kind == TokenKind::Comment {
                continue;
            }

            tokens.push(token);
        }

        // Make sure the last token is an EOF token.
        if let Some(token) = tokens.last() {
            if token.token_kind != TokenKind::EndOfFile {
                tokens.push(Token::new(TokenKind::EndOfFile, self.line, self.column));
            }
        }

        Ok(tokens)
    }
}

/// Reads a `.lt` file and returns its contents.
///
/// # Arguments
///
/// * `path` - The path to the file.
///
/// # Returns
///
/// * `Result<String, Error>` - The contents of the file, or an error.
///
/// # Errors
///
/// * If the file doesn't exist.
/// * If the file doesn't end in `.lt`.
/// * If the file can't be read.
pub fn read_file(path: &str) -> Result<String, Error> {
    let path = std::path::Path::new(path);

    // If the file doesn't exist, return an error.
    if !path.exists() {
        return Err(Error::InvalidFilePath);
    }

    // If the file doesn't end in ".lt", return an error.
    if !path.extension().is_some_and(|ext| ext == "lt") {
        return Err(Error::InvalidFileExtension);
    }

    // Read the file.
    let contents = std::fs::read_to_string(path)?;

    Ok(contents)
}
