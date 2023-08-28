use crate::lang::lexer::{Literal, Token, TokenType};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Expression(Expression),
    Print(Expression),
    Variable(Token, Option<Expression>),
    Block(Vec<Statement>),
    If(Expression, Box<Statement>, Option<Box<Statement>>),
    While(Expression, Box<Statement>),
    Break,
    Continue,
    Function(Token, Vec<Token>, Vec<Statement>),
    Return(Token, Option<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Binary(Box<Expression>, Token, Box<Expression>),
    Grouping(Box<Expression>),
    Literal(Literal),
    Unary(Token, Box<Expression>),
    Variable(Token),
    Assign(Token, Box<Expression>),
    Call(Box<Expression>, Token, Vec<Expression>),
}

impl Parser {
    /// Create a new parser.
    ///
    /// # Arguments
    /// * `tokens` - The tokens to parse.
    pub fn new(tokens: &[Token]) -> Self {
        Self {
            tokens: tokens.to_vec(),
            current: 0,
        }
    }

    /// Parse the tokens.
    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration());
        }

        statements
    }

    /// Parse a declaration.
    fn declaration(&mut self) -> Statement {
        if self.matches(&[TokenType::Variable]) {
            self.variable_declaration()
        } else if self.matches(&[TokenType::Function]) {
            self.function_declaration("function")
        } else {
            self.statement()
        }
    }

    /// Parse a variable declaration.
    fn variable_declaration(&mut self) -> Statement {
        let name = self.consume(TokenType::Identifier, "Expected variable name.");

        let initializer = if self.matches(&[TokenType::Equal]) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(
            TokenType::Semicolon,
            "Expected ';' after variable declaration.",
        );

        Statement::Variable(name, initializer)
    }

    /// Parse a function declaration.
    fn function_declaration(&mut self, kind: &str) -> Statement {
        let name = self.consume(TokenType::Identifier, &format!("Expected {} name.", kind));

        self.consume(
            TokenType::LeftParenthesis,
            &format!("Expected '(' after {} name.", kind),
        );

        let mut parameters = Vec::new();

        if !self.check(TokenType::RightParenthesis) {
            loop {
                if parameters.len() >= 255 {
                    self.error(self.peek(), "Cannot have more than 255 parameters.");
                }

                // Push the parameter name, with the type as a literal. Types come after a colon.
                let name = self.consume(TokenType::Identifier, "Expected parameter name.");
                // If the next token is a colon, then we have a type.
                let literal = if self.matches(&[TokenType::Colon]) {
                    let token = self.peek();

                    self.advance();

                    Some(token.literal.unwrap())
                } else {
                    None
                };

                parameters.push(Token::new(
                    TokenType::Identifier,
                    &name.lexeme,
                    literal,
                    name.line,
                    name.column,
                ));

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after parameters.",
        );

        self.consume(
            TokenType::LeftCurlyBrace,
            &format!("Expected '{{' before {} body.", kind),
        );

        let body = self.block();

        Statement::Function(name, parameters, body)
    }

    /// Parse a statement.
    fn statement(&mut self) -> Statement {
        if self.matches(&[TokenType::Print]) {
            self.print_statement()
        } else if self.matches(&[TokenType::LeftCurlyBrace]) {
            Statement::Block(self.block())
        } else if self.matches(&[TokenType::If]) {
            self.if_statement()
        } else if self.matches(&[TokenType::While]) {
            self.while_statement()
        } else if self.matches(&[TokenType::Break]) {
            self.break_statement()
        } else if self.matches(&[TokenType::Continue]) {
            self.continue_statement()
        } else if self.matches(&[TokenType::Return]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    /// Parse a print statement.
    fn print_statement(&mut self) -> Statement {
        let value = self.expression();

        self.consume(TokenType::Semicolon, "Expected ';' after value.");

        Statement::Print(value)
    }

    /// Parse an if statement.
    fn if_statement(&mut self) -> Statement {
        self.consume(TokenType::LeftParenthesis, "Expected '(' after 'if'.");

        let condition = self.expression();

        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after if condition.",
        );

        let then_branch = Box::new(self.statement());

        let else_branch = if self.matches(&[TokenType::Else]) {
            Some(Box::new(self.statement()))
        } else {
            None
        };

        Statement::If(condition, then_branch, else_branch)
    }

    /// Parse a while statement.
    fn while_statement(&mut self) -> Statement {
        self.consume(TokenType::LeftParenthesis, "Expected '(' after 'while'.");

        let condition = self.expression();

        self.consume(
            TokenType::RightParenthesis,
            "Expected ')' after while condition.",
        );

        let body = Box::new(self.statement());

        Statement::While(condition, body)
    }

    /// Parse a break statement.
    fn break_statement(&mut self) -> Statement {
        self.consume(TokenType::Semicolon, "Expected ';' after 'break'.");

        Statement::Break
    }

    /// Parse a continue statement.
    fn continue_statement(&mut self) -> Statement {
        self.consume(TokenType::Semicolon, "Expected ';' after 'continue'.");

        Statement::Continue
    }

    /// Parse a return statement.
    fn return_statement(&mut self) -> Statement {
        let keyword = self.previous();

        let value = if !self.check(TokenType::Semicolon) {
            Some(self.expression())
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return value.");

        Statement::Return(keyword, value)
    }

    /// Parse an expression statement.
    fn expression_statement(&mut self) -> Statement {
        let value = self.expression();

        self.consume(TokenType::Semicolon, "Expected ';' after value.");

        Statement::Expression(value)
    }

    /// Parse a block.
    fn block(&mut self) -> Vec<Statement> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightCurlyBrace) && !self.is_at_end() {
            statements.push(self.declaration());
        }

        self.consume(TokenType::RightCurlyBrace, "Expected '}' after block.");

        statements
    }

    /// Parse an expression.
    fn expression(&mut self) -> Expression {
        self.assignment()
    }

    /// Parse an assignment expression.
    fn assignment(&mut self) -> Expression {
        let expression = self.or();

        if self.matches(&[TokenType::Equal]) {
            let equals = self.previous();
            let value = self.assignment();

            if let Expression::Variable(name) = expression {
                return Expression::Assign(name, Box::new(value));
            }

            self.error(equals, "Invalid assignment target.");
        }

        expression
    }

    /// Parse an or expression.
    fn or(&mut self) -> Expression {
        let mut expression = self.and();

        while self.matches(&[TokenType::LogicalOr]) {
            let operator = self.previous();
            let right = self.and();

            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    /// Parse an and expression.
    fn and(&mut self) -> Expression {
        let mut expression = self.equality();

        while self.matches(&[TokenType::LogicalAnd]) {
            let operator = self.previous();
            let right = self.equality();

            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    /// Parse an equality expression.
    fn equality(&mut self) -> Expression {
        let mut expression = self.comparison();

        while self.matches(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison();

            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    /// Parse a comparison expression.
    fn comparison(&mut self) -> Expression {
        let mut expression = self.term();

        while self.matches(&[
            TokenType::GreaterThan,
            TokenType::GreaterThanOrEqual,
            TokenType::LessThan,
            TokenType::LessThanOrEqual,
        ]) {
            let operator = self.previous();
            let right = self.term();

            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    /// Parse a term expression.
    fn term(&mut self) -> Expression {
        let mut expression = self.factor();

        while self.matches(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.factor();

            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    /// Parse a factor expression.
    fn factor(&mut self) -> Expression {
        let mut expression = self.unary();

        while self.matches(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.unary();

            expression = Expression::Binary(Box::new(expression), operator, Box::new(right));
        }

        expression
    }

    /// Parse a unary expression.
    fn unary(&mut self) -> Expression {
        if self.matches(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = self.unary();

            return Expression::Unary(operator, Box::new(right));
        }

        self.call()
    }

    /// Parse a call expression.
    fn call(&mut self) -> Expression {
        let mut expression = self.primary();

        loop {
            if self.matches(&[TokenType::LeftParenthesis]) {
                expression = self.finish_call(expression);
            } else {
                break;
            }
        }

        expression
    }

    /// Finish parsing a call expression.
    fn finish_call(&mut self, callee: Expression) -> Expression {
        let mut arguments = Vec::new();

        if !self.check(TokenType::RightParenthesis) {
            loop {
                if arguments.len() >= 255 {
                    self.error(self.peek(), "Cannot have more than 255 arguments.");
                }

                arguments.push(self.expression());

                if !self.matches(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParenthesis, "Expected ')' after arguments.");

        Expression::Call(Box::new(callee), paren, arguments)
    }

    /// Parse a primary expression.
    fn primary(&mut self) -> Expression {
        if self.matches(&[TokenType::False]) {
            return Expression::Literal(Literal::Boolean(false));
        }

        if self.matches(&[TokenType::True]) {
            return Expression::Literal(Literal::Boolean(true));
        }

        if self.matches(&[TokenType::None]) {
            return Expression::Literal(Literal::None);
        }

        if self.matches(&[TokenType::Number, TokenType::String]) {
            return Expression::Literal(self.previous().literal.unwrap());
        }

        if self.matches(&[TokenType::Identifier]) {
            return Expression::Variable(self.previous());
        }

        if self.matches(&[TokenType::LeftParenthesis]) {
            let expression = self.expression();

            self.consume(
                TokenType::RightParenthesis,
                "Expected ')' after expression.",
            );

            return Expression::Grouping(Box::new(expression));
        }

        self.error(self.peek(), "Expected expression.");

        unreachable!()
    }

    /// Consume a token if it matches the given token types.
    ///
    /// # Arguments
    /// * `types` - The token types to match.
    ///
    /// # Returns
    /// * `true` if the token was consumed.
    /// * `false` if the token was not consumed.
    fn matches(&mut self, types: &[TokenType]) -> bool {
        for token_type in types {
            if self.check(*token_type) {
                self.advance();

                return true;
            }
        }

        false
    }

    /// Consume a token if it matches the given token type.
    ///
    /// # Arguments
    /// * `token_type` - The token type to match.
    ///
    /// # Returns
    /// * The token if it was consumed.
    /// * `None` if the token was not consumed.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Token {
        if self.check(token_type) {
            return self.advance();
        }

        self.error(self.peek(), message);

        unreachable!()
    }

    /// Check if the current token matches the given token type.
    ///
    /// # Arguments
    /// * `token_type` - The token type to match.
    ///
    /// # Returns
    /// * `true` if the token matches.
    /// * `false` if the token does not match.
    fn check(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().token_type == token_type
    }

    /// Advance the current token.
    ///
    /// # Returns
    /// * The previous token.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous()
    }

    /// Check if the current token is the end of the file.
    ///
    /// # Returns
    /// * `true` if the current token is the end of the file.
    /// * `false` if the current token is not the end of the file.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    /// Get the current token.
    ///
    /// # Returns
    /// * The current token.
    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    /// Get the previous token.
    ///
    /// # Returns
    /// * The previous token.
    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    /// Report an error.
    ///
    /// # Arguments
    /// * `token` - The token where the error occurred.
    /// * `message` - The error message.
    fn error(&self, token: Token, message: &str) {
        if token.token_type == TokenType::EndOfFile {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at '{}'", token.lexeme), message);
        }
    }

    /// Report an error.
    ///
    /// # Arguments
    /// * `line` - The line where the error occurred.
    /// * `location` - The location where the error occurred.
    /// * `message` - The error message.
    fn report(&self, line: usize, location: &str, message: &str) {
        println!("[line {}] Error{}: {}", line, location, message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let tokens = vec![
            Token::new(TokenType::Print, "print", None, 1, 0),
            Token::new(TokenType::LeftParenthesis, "(", None, 1, 5),
            Token::new(
                TokenType::String,
                "Hello, world!",
                Some(Literal::String("Hello, world!".to_string())),
                1,
                6,
            ),
            Token::new(TokenType::RightParenthesis, ")", None, 1, 21),
            Token::new(TokenType::Semicolon, ";", None, 1, 22),
            Token::new(TokenType::EndOfFile, "", None, 1, 23),
        ];

        let mut parser = Parser::new(&tokens);

        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![Statement::Print(Expression::Grouping(Box::new(
                Expression::Literal(Literal::String("Hello, world!".to_string()))
            ))),]
        );
    }

    #[test]
    fn test_parse_variable_declaration() {
        let tokens = vec![
            Token::new(TokenType::Variable, "var", None, 1, 0),
            Token::new(TokenType::Identifier, "a", None, 1, 4),
            Token::new(TokenType::Equal, "=", None, 1, 6),
            Token::new(TokenType::Number, "1", Some(Literal::Number(1.0)), 1, 8),
            Token::new(TokenType::Semicolon, ";", None, 1, 9),
            Token::new(TokenType::EndOfFile, "", None, 1, 10),
        ];

        let mut parser = Parser::new(&tokens);

        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![Statement::Variable(
                Token::new(TokenType::Identifier, "a", None, 1, 4),
                Some(Expression::Literal(Literal::Number(1.0)))
            ),]
        );
    }

    #[test]
    fn test_parse_function_declaration() {
        let tokens = vec![
            Token::new(TokenType::Function, "function", None, 1, 0),
            Token::new(TokenType::Identifier, "a", None, 1, 9),
            Token::new(TokenType::LeftParenthesis, "(", None, 1, 10),
            Token::new(TokenType::RightParenthesis, ")", None, 1, 11),
            Token::new(TokenType::LeftCurlyBrace, "{", None, 1, 12),
            Token::new(TokenType::RightCurlyBrace, "}", None, 1, 13),
            Token::new(TokenType::EndOfFile, "", None, 1, 14),
        ];

        let mut parser = Parser::new(&tokens);

        let statements = parser.parse();

        assert_eq!(
            statements,
            vec![Statement::Function(
                Token::new(TokenType::Identifier, "a", None, 1, 9),
                vec![],
                vec![]
            ),]
        );
    }

    #[test]
    fn test_parse_block() {
        let tokens = vec![
            Token::new(TokenType::LeftCurlyBrace, "{", None, 1, 0),
            Token::new(TokenType::RightCurlyBrace, "}", None, 1, 1),
            Token::new(TokenType::EndOfFile, "", None, 1, 2),
        ];

        let mut parser = Parser::new(&tokens);

        let statements = parser.parse();

        assert_eq!(statements, vec![Statement::Block(vec![]),]);
    }
}
