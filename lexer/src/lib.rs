use crate::token::{Token, TokenKind};
use utils::errors::Error;

pub mod token;

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
            column: 1,
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

            identifier.push(c);
            self.advance();
        }

        let token_type = match identifier.as_str() {
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "in" => TokenKind::In,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "return" => TokenKind::Return,
            "fn" => TokenKind::Function,
            "let" => TokenKind::Variable,
            _ => TokenKind::Identifier(identifier),
        };

        Token::new(token_type, self.line, self.column)
    }

    fn read_number(&mut self) -> Result<Token, Error> {
        let mut number = String::new();
        while let Some(c) = self.current_char() {
            if !c.is_numeric() {
                break;
            }

            number.push(c);
            self.advance();
        }

        Ok(Token::new(
            TokenKind::Integer(number.parse()?),
            self.line,
            self.column,
        ))
    }

    fn read_string(&mut self) -> Result<Token, Error> {
        self.advance();

        let mut string = String::new();
        while let Some(c) = self.current_char() {
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

    fn read_operator(&mut self) -> Result<Token, Error> {
        let current_char = self.current_char();
        let kind = match current_char {
            Some('+') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::PlusEqual
                } else {
                    TokenKind::Plus
                }
            }
            Some('-') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::MinusEqual
                } else {
                    TokenKind::Minus
                }
            }
            Some('*') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::AsteriskEqual
                } else {
                    TokenKind::Asterisk
                }
            }
            Some('/') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::SlashEqual
                } else {
                    TokenKind::Slash
                }
            }
            Some('%') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::PercentEqual
                } else {
                    TokenKind::Percent
                }
            }
            Some('^') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::CaretEqual
                } else {
                    TokenKind::Caret
                }
            }
            Some('!') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::BangEqual
                } else {
                    TokenKind::Bang
                }
            }
            Some('=') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::EqualEqual
                } else {
                    TokenKind::Equal
                }
            }
            Some('<') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::LessEqual
                } else {
                    TokenKind::Less
                }
            }
            Some('>') => {
                if self.next_char() == Some('=') {
                    self.advance();

                    TokenKind::GreaterEqual
                } else {
                    TokenKind::Greater
                }
            }
            Some('&') => TokenKind::And,
            Some('|') => TokenKind::Or,
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
        self.skip_whitespace();

        let current_char = self.current_char();
        let token = match current_char {
            Some(c) if c.is_alphabetic() || c == '_' => self.read_identifier(),
            Some(c) if c.is_numeric() => self.read_number()?,
            Some('"') => self.read_string()?,
            _ => self.read_operator()?, // Assume it's an operator until proven otherwise.
        };

        Ok(token)
    }

    /// Runs a lexical analysis on the input.
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
    pub fn lex(&mut self) -> Result<Vec<Token>, Error> {
        let mut tokens = Vec::new();
        while self.position < self.input.len() {
            let token = self.read_token()?;
            println!("{token:?}");

            tokens.push(token);
        }

        tokens.push(Token::new(TokenKind::EOF, self.line, self.column));

        Ok(tokens)
    }
}
