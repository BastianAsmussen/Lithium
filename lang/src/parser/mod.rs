pub mod errors;

use crate::lexer::token::{Kind, Token};
use errors::Error;

/// A literal expression.
///
/// # Variants
///
/// * `Number` - A number literal.
/// * `String` - A string literal.
/// * `Boolean` - A boolean literal.
#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    String(String),
    Boolean(bool),
}

/// An expression.
///
/// # Variants
///
/// * `Literal` - A literal expression.
/// * `Unary` - A unary expression.
/// * `Binary` - A binary expression.
/// * `Grouping` - A grouping expression.
#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal(Literal),
    Unary {
        operator: Token,
        right: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: Token,
        right: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Assignment {
        name: Token,
        value: Box<Expression>,
    },
    Variable {
        name: Token,
    },
    Call {
        callee: Box<Expression>,
        arguments: Vec<Expression>,
    },
    Function {
        name: Token,
        parameters: Vec<Token>,
        body: Box<Statement>,
    },
}

/// A statement.
///
/// # Variants
///
/// * `Expression` - An expression statement.
/// * `Variable` - A variable statement.
/// * `Block` - A block statement.
/// * `If` - An if statement.
/// * `While` - A while statement.
/// * `For` - A for statement.
/// * `Break` - A break statement.
/// * `Continue` - A continue statement.
/// * `Return` - A return statement.
/// * `Function` - A function statement.
#[derive(Debug, PartialEq)]
pub enum Statement {
    Expression {
        expression: Expression,
    },
    Variable {
        name: Token,
        initializer: Option<Expression>,
    },
    Block {
        statements: Vec<Statement>,
    },
    If {
        condition: Expression,
        then_branch: Box<Statement>,
        else_branch: Option<Box<Statement>>,
    },
    While {
        condition: Expression,
        body: Box<Statement>,
    },
    For {
        initializer: Option<Box<Statement>>,
        condition: Option<Expression>,
        increment: Option<Expression>,
        body: Box<Statement>,
    },
    Break,
    Continue,
    Return {
        keyword: Token,
        value: Option<Expression>,
    },
    Function {
        name: Token,
        parameters: Vec<(Token, Token)>,
        return_type: Option<Token>,
        body: Box<Statement>,
    },
}

/// A parser for Lithium.
///
/// # Fields
///
/// * `tokens` - The tokens to parse.
/// * `current` - The index of the current token.
#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    /// Creates a new parser.
    ///
    /// # Arguments
    ///
    /// * `tokens` - The tokens to parse.
    ///
    /// # Returns
    ///
    /// * The new parser.
    #[must_use]
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    /// Parses the tokens into an AST.
    ///
    /// # Returns
    ///
    /// * `Result<Vec<Statement>, Error>` - The parsed statements, or an error.
    ///
    /// # Errors
    ///
    /// * `Error::UnexpectedToken` - The parser encountered an unexpected token.
    /// * `Error::InvalidAssignmentTarget` - The parser encountered an invalid assignment target.
    pub fn parse(&mut self) -> Result<Vec<Statement>, Error> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        Ok(statements)
    }

    fn declaration(&mut self) -> Result<Statement, Error> {
        if self.matches(&[Kind::Variable]) {
            self.variable_declaration()
        } else if self.matches(&[Kind::Function]) {
            self.function_declaration()
        } else {
            self.statement()
        }
    }

    fn variable_declaration(&mut self) -> Result<Statement, Error> {
        let name = match self.peek().kind {
            Kind::Identifier(_) => self.advance(),
            _ => {
                return Err(Error::UnexpectedToken {
                    line: self.peek().line,
                    column: self.peek().column,
                    message: "Expected variable name.".to_string(),
                })
            }
        };

        let initializer = if self.matches(&[Kind::Assign]) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(&Kind::Semicolon, "Expected ';' after variable declaration.")?;

        Ok(Statement::Variable { name, initializer })
    }

    fn function_declaration(&mut self) -> Result<Statement, Error> {
        let name = match self.peek().kind {
            Kind::Identifier(_) => self.advance(),
            _ => {
                return Err(Error::UnexpectedToken {
                    line: self.peek().line,
                    column: self.peek().column,
                    message: "Expected function name.".to_string(),
                })
            }
        };
        self.consume(&Kind::LeftParenthesis, "Expected '(' after function name.")?;

        let mut parameters = Vec::new();
        if !self.check(&Kind::RightParenthesis) {
            loop {
                let name = match self.peek().kind {
                    Kind::Identifier(_) => self.advance(),
                    _ => {
                        return Err(Error::UnexpectedToken {
                            line: self.peek().line,
                            column: self.peek().column,
                            message: "Expected parameter name.".to_string(),
                        })
                    }
                };
                self.consume(&Kind::Colon, "Expected ':' after parameter name.")?;
                let r#type = match self.peek().kind {
                    Kind::Identifier(_) => self.advance(),
                    _ => {
                        return Err(Error::UnexpectedToken {
                            line: self.peek().line,
                            column: self.peek().column,
                            message: "Expected parameter type.".to_string(),
                        })
                    }
                };

                parameters.push((name, r#type));

                if !self.matches(&[Kind::Comma]) {
                    break;
                }
            }
        }

        self.consume(&Kind::RightParenthesis, "Expected ')' after parameters.")?;

        // Return type, if any.
        let return_type = if self.matches(&[Kind::Arrow]) {
            Some(match self.peek().kind {
                Kind::Identifier(_) => self.advance(),
                _ => {
                    return Err(Error::UnexpectedToken {
                        line: self.peek().line,
                        column: self.peek().column,
                        message: "Expected return type.".to_string(),
                    })
                }
            })
        } else {
            None
        };

        self.consume(&Kind::LeftCurlyBrace, "Expected '{' before function body.")?;

        let body = self.block()?;

        Ok(Statement::Function {
            name,
            parameters,
            return_type,
            body: Box::new(body),
        })
    }

    fn statement(&mut self) -> Result<Statement, Error> {
        if self.matches(&[Kind::LeftCurlyBrace]) {
            self.block()
        } else if self.matches(&[Kind::If]) {
            self.if_statement()
        } else if self.matches(&[Kind::While]) {
            self.while_statement()
        } else if self.matches(&[Kind::For]) {
            self.for_statement()
        } else if self.matches(&[Kind::Break]) {
            self.break_statement()
        } else if self.matches(&[Kind::Continue]) {
            self.continue_statement()
        } else if self.matches(&[Kind::Return]) {
            self.return_statement()
        } else {
            self.expression_statement()
        }
    }

    fn block(&mut self) -> Result<Statement, Error> {
        let mut statements = Vec::new();

        while !self.check(&Kind::RightCurlyBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&Kind::RightCurlyBrace, "Expected '}' after block.")?;

        Ok(Statement::Block { statements })
    }

    fn if_statement(&mut self) -> Result<Statement, Error> {
        self.consume(&Kind::LeftParenthesis, "Expected '(' after 'if'.")?;

        let condition = self.expression()?;

        self.consume(&Kind::RightParenthesis, "Expected ')' after if condition.")?;

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.matches(&[Kind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Statement::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Statement, Error> {
        self.consume(&Kind::LeftParenthesis, "Expected '(' after 'while'.")?;

        let condition = self.expression()?;

        self.consume(
            &Kind::RightParenthesis,
            "Expected ')' after while condition.",
        )?;

        let body = Box::new(self.statement()?);

        Ok(Statement::While { condition, body })
    }

    fn for_statement(&mut self) -> Result<Statement, Error> {
        self.consume(&Kind::LeftParenthesis, "Expected '(' after 'for'.")?;

        let initializer = if self.matches(&[Kind::Semicolon]) {
            None
        } else if self.matches(&[Kind::Variable]) {
            Some(Box::new(self.variable_declaration()?))
        } else {
            Some(Box::new(self.expression_statement()?))
        };

        let condition = if self.check(&Kind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&Kind::Semicolon, "Expected ';' after loop condition.")?;

        let increment = if self.check(&Kind::RightParenthesis) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&Kind::RightParenthesis, "Expected ')' after for clauses.")?;

        let body = Box::new(self.statement()?);

        Ok(Statement::For {
            initializer,
            condition,
            increment,
            body,
        })
    }

    fn break_statement(&mut self) -> Result<Statement, Error> {
        self.consume(&Kind::Semicolon, "Expected ';' after 'break'.")?;

        Ok(Statement::Break)
    }

    fn continue_statement(&mut self) -> Result<Statement, Error> {
        self.consume(&Kind::Semicolon, "Expected ';' after 'continue'.")?;

        Ok(Statement::Continue)
    }

    fn return_statement(&mut self) -> Result<Statement, Error> {
        let keyword = self.previous().clone();

        let value = if self.check(&Kind::Semicolon) {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(&Kind::Semicolon, "Expected ';' after return value.")?;

        Ok(Statement::Return { keyword, value })
    }

    fn expression_statement(&mut self) -> Result<Statement, Error> {
        let expression = self.expression()?;

        self.consume(&Kind::Semicolon, "Expected ';' after expression.")?;

        Ok(Statement::Expression { expression })
    }

    fn expression(&mut self) -> Result<Expression, Error> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, Error> {
        let expression = self.or()?;

        if self.matches(&[Kind::Assign]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            if let Expression::Variable { name } = expression {
                return Ok(Expression::Assignment {
                    name,
                    value: Box::new(value),
                });
            }

            return Err(Error::InvalidAssignmentTarget {
                line: equals.line,
                column: equals.column,
            });
        }

        Ok(expression)
    }

    fn or(&mut self) -> Result<Expression, Error> {
        let mut expression = self.and()?;

        while self.matches(&[Kind::LogicalOr]) {
            let operator = self.previous().clone();
            let right = self.and()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn and(&mut self) -> Result<Expression, Error> {
        let mut expression = self.equality()?;

        while self.matches(&[Kind::LogicalAnd]) {
            let operator = self.previous().clone();
            let right = self.equality()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn equality(&mut self) -> Result<Expression, Error> {
        let mut expression = self.comparison()?;

        while self.matches(&[Kind::Equality, Kind::NotEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn comparison(&mut self) -> Result<Expression, Error> {
        let mut expression = self.term()?;

        while self.matches(&[
            Kind::LessThan,
            Kind::LessThanOrEqual,
            Kind::GreaterThan,
            Kind::GreaterThanOrEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn term(&mut self) -> Result<Expression, Error> {
        let mut expression = self.factor()?;

        while self.matches(&[Kind::Plus, Kind::Minus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn factor(&mut self) -> Result<Expression, Error> {
        let mut expression = self.unary()?;

        while self.matches(&[Kind::Star, Kind::Slash, Kind::Percent]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            expression = Expression::Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            };
        }

        Ok(expression)
    }

    fn unary(&mut self) -> Result<Expression, Error> {
        if self.matches(&[Kind::Minus, Kind::LogicalNot]) {
            let operator = self.previous().clone();
            let right = self.unary()?;

            return Ok(Expression::Unary {
                operator,
                right: Box::new(right),
            });
        }

        self.call()
    }

    fn call(&mut self) -> Result<Expression, Error> {
        let mut expression = self.primary()?;

        loop {
            if self.matches(&[Kind::LeftParenthesis]) {
                expression = self.finish_call(expression)?;
            } else {
                break;
            }
        }

        Ok(expression)
    }

    fn finish_call(&mut self, callee: Expression) -> Result<Expression, Error> {
        let mut arguments = Vec::new();

        if !self.check(&Kind::RightParenthesis) {
            loop {
                arguments.push(self.expression()?);

                if !self.matches(&[Kind::Comma]) {
                    break;
                }
            }
        }

        self.consume(&Kind::RightParenthesis, "Expected ')' after arguments.")?;

        Ok(Expression::Call {
            callee: Box::new(callee),
            arguments,
        })
    }

    #[allow(clippy::cast_precision_loss)]
    fn primary(&mut self) -> Result<Expression, Error> {
        let next = self.peek().clone();
        match next.kind {
            Kind::False => {
                self.advance();

                Ok(Expression::Literal(Literal::Boolean(false)))
            }
            Kind::True => {
                self.advance();

                Ok(Expression::Literal(Literal::Boolean(true)))
            }
            Kind::Float(value) => {
                self.advance();

                Ok(Expression::Literal(Literal::Number(value)))
            }
            Kind::Integer(value) => {
                self.advance();

                Ok(Expression::Literal(Literal::Number(value as f64)))
            }
            Kind::String(value) => {
                self.advance();

                Ok(Expression::Literal(Literal::String(value)))
            }
            Kind::Identifier(_) => {
                self.advance();

                Ok(Expression::Variable {
                    name: self.previous().clone(),
                })
            }
            Kind::LeftParenthesis => {
                self.advance();

                let expression = self.expression()?;

                self.consume(&Kind::RightParenthesis, "Expected ')' after expression.")?;

                Ok(Expression::Grouping {
                    expression: Box::new(expression),
                })
            }
            _ => Err(Error::UnexpectedToken {
                line: self.peek().line,
                column: self.peek().column,
                message: "Expected expression.".to_string(),
            }),
        }
    }

    fn consume(&mut self, kind: &Kind, message: &str) -> Result<Token, Error> {
        if self.check(kind) {
            return Ok(self.advance());
        }

        Err(Error::UnexpectedToken {
            line: self.peek().line,
            column: self.peek().column,
            message: message.to_string(),
        })
    }

    fn matches(&mut self, kinds: &[Kind]) -> bool {
        for kind in kinds {
            if self.check(kind) {
                self.advance();

                return true;
            }
        }

        false
    }

    fn check(&self, kind: &Kind) -> bool {
        if self.is_at_end() {
            return false;
        }

        self.peek().kind == *kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }

        self.previous().clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == Kind::EndOfFile
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}
