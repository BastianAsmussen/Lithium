use crate::errors::Error;
use lexer::token::{Token, TokenKind};

pub mod errors;

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

    /// Parses the stream of tokens into an AST.
    ///
    /// # Returns
    ///
    /// * `Result<AST, Error>` - The result of parsing the stream of tokens into an AST.
    ///
    /// # Errors
    ///
    /// * `Error` - The error that occurred while parsing the stream of tokens into an AST.
    pub fn parse(&mut self) -> Result<AST, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(AST { statements })
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::EndOfFile
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }
}

/// An AST.
///
/// # Fields
///
/// * `statements` - The statements in the AST.
#[derive(Debug)]
pub struct AST {
    statements: Vec<Statement>,
}

#[derive(Debug)]
struct Statement(Expression);

#[derive(Debug)]
struct Expression(Literal);

#[derive(Debug)]
enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

impl Parser {
    fn declaration(&mut self) -> Result<Statement, Error> {
        let expression = self.expression()?;

        self.consume(&TokenKind::Semicolon, "Expected ';' after expression.")?;

        Ok(Statement(expression))
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.primary()
    }

    fn primary(&mut self) -> Result<Expression, Error> {
        let token = self.advance();

        match &token.kind {
            TokenKind::Integer(number) => Ok(Expression(Literal::Number(*number as f64))),
            TokenKind::Float(number) => Ok(Expression(Literal::Number(*number))),
            TokenKind::String(string) => Ok(Expression(Literal::String(string.clone()))),
            TokenKind::True => Ok(Expression(Literal::Boolean(true))),
            TokenKind::False => Ok(Expression(Literal::Boolean(false))),
            _ => Err(Error::InvalidToken {
                message: format!("Expected expression, found '{:?}'.", token.kind),
                line: token.line,
                column: token.column,
            }),
        }
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn consume(&mut self, kind: &TokenKind, message: &str) -> Result<&Token, Error> {
        if self.check(kind) {
            Ok(self.advance())
        } else {
            Err(Error::InvalidToken {
                message: message.to_string(),
                line: self.peek().line,
                column: self.peek().column,
            })
        }
    }

    fn check(&self, kind: &TokenKind) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().kind == *kind
        }
    }
}
