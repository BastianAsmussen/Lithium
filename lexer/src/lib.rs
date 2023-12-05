use crate::errors::Error;
use crate::token::{Kind, Token};

pub mod errors;
pub mod token;

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
            "true" => Kind::True,
            "false" => Kind::False,
            "if" => Kind::If,
            "else" => Kind::Else,
            "while" => Kind::While,
            "for" => Kind::For,
            "in" => Kind::Range, // "in" is a keyword because it's used in "for" loops.
            "to" => Kind::To,    // "to" is a keyword because it's used in "for" loops.
            "break" => Kind::Break,
            "continue" => Kind::Continue,
            "return" => Kind::Return,
            "fn" => Kind::Function,
            "let" => Kind::Variable,
            _ => Kind::Identifier(identifier),
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

            Kind::Float(number)
        } else if number.contains('b') || number.contains('B') {
            let number = i64::from_str_radix(&number[2..], 2)?;

            Kind::Integer(number)
        } else if number.contains('o') || number.contains('O') {
            let number = i64::from_str_radix(&number[2..], 8)?;

            Kind::Integer(number)
        } else if number.contains('x') || number.contains('X') {
            let number = i64::from_str_radix(&number[2..], 16)?;

            Kind::Integer(number)
        } else {
            let number = number.parse::<i64>()?;

            Kind::Integer(number)
        };

        // If the number is negative, then we need to negate it.
        let token_kind = if is_negative {
            match token_kind {
                Kind::Integer(number) => Kind::Integer(-number),
                Kind::Float(number) => Kind::Float(-number),
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

                return Ok(Token::new(Kind::String(string), self.line, self.column));
            }

            string.push(c);
            self.advance();
        }

        Err(Error::UnterminatedString {
            line: self.line,
            column: self.column,
        })
    }

    fn handle_plus(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::AddAssign
            }
            Some('+') => {
                self.advance();

                Kind::Increment
            }
            _ => Kind::Plus,
        }
    }

    fn handle_minus(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::SubtractAssign
            }
            Some('-') => {
                self.advance();

                Kind::Decrement
            }
            Some('>') => {
                self.advance();

                Kind::Arrow
            }
            _ => Kind::Minus,
        }
    }

    fn handle_asterisk(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::MultiplyAssign
            }
            Some('*') => {
                self.advance();

                Kind::Power
            }
            _ => Kind::Star,
        }
    }

    fn handle_slash(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::DivisionAssign
            }
            Some('/') => {
                // Single-line comment, ignore until a new-line.
                while let Some(c) = self.current_char() {
                    if c == '\n' {
                        break;
                    }

                    self.advance();
                }

                Kind::Comment
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

                Kind::Comment
            }
            _ => Kind::Slash,
        }
    }

    fn handle_percent(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::ModuloAssign
            }
            _ => Kind::Percent,
        }
    }

    fn handle_caret(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::BitwiseXorAssign
            }
            _ => Kind::BitwiseXor,
        }
    }

    fn handle_bang(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::NotEqual
            }
            _ => Kind::LogicalNot,
        }
    }

    fn handle_equal(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::Equality
            }
            _ => Kind::Assign,
        }
    }

    fn handle_less_than(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::LessThanOrEqual
            }
            Some('<') => {
                self.advance();

                if self.next_char() == Some('=') {
                    self.advance();

                    Kind::BitwiseLeftShiftAssign
                } else {
                    Kind::BitwiseLeftShift
                }
            }
            _ => Kind::LessThan,
        }
    }

    fn handle_greater_than(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::GreaterThanOrEqual
            }
            Some('>') => {
                self.advance();

                if self.next_char() == Some('=') {
                    self.advance();

                    Kind::BitwiseRightShiftAssign
                } else {
                    Kind::BitwiseRightShift
                }
            }
            _ => Kind::GreaterThan,
        }
    }

    fn handle_ampersand(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::BitwiseAndAssign
            }
            Some('&') => {
                self.advance();

                Kind::LogicalAnd
            }
            _ => Kind::BitwiseAnd,
        }
    }

    fn handle_pipe(&mut self) -> Kind {
        match self.next_char() {
            Some('=') => {
                self.advance();

                Kind::BitwiseOrAssign
            }
            Some('|') => {
                self.advance();

                Kind::LogicalOr
            }
            _ => Kind::BitwiseOr,
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
            Some('(') => Kind::LeftParenthesis,
            Some(')') => Kind::RightParenthesis,
            Some('{') => Kind::LeftCurlyBrace,
            Some('}') => Kind::RightCurlyBrace,
            Some('[') => Kind::LeftBracket,
            Some(']') => Kind::RightBracket,
            Some(',') => Kind::Comma,
            Some('.') => Kind::Dot,
            Some(':') => Kind::Colon,
            Some(';') => Kind::Semicolon,
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
            None => Token::new(Kind::EndOfFile, self.line, self.column),
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
            if token.kind == Kind::Comment {
                continue;
            }

            tokens.push(token);
        }

        // Make sure the last token is an EOF token.
        if let Some(token) = tokens.last() {
            if token.kind != Kind::EndOfFile {
                tokens.push(Token::new(Kind::EndOfFile, self.line, self.column));
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
