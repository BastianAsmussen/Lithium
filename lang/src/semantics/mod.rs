use crate::lexer::tokens::{Token, TokenKind};
use crate::parser::ast::AST;
use crate::parser::{Expression, Statement};
use crate::semantics::errors::Error;
use crate::semantics::scopes::Scope;
use crate::semantics::symbols::SymbolKind;

pub mod errors;
pub mod scopes;
pub mod symbols;

pub trait Visitor {
    /// Visits a statement.
    ///
    /// # Arguments
    ///
    /// * `statement` - The statement to visit.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the visit.
    ///
    /// # Errors
    ///
    /// * If the scope is invalid.
    /// * If the symbol is undefined.
    /// * If the symbol is already defined.
    fn visit_statement(&mut self, statement: &Statement) -> Result<(), Error>;

    /// Visits an expression.
    ///
    /// # Arguments
    ///
    /// * `expression` - The expression to visit.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the visit.
    ///
    /// # Errors
    ///
    /// * If the scope is invalid.
    /// * If the symbol is undefined.
    /// * If the symbol is already defined.
    fn visit_expression(&mut self, expression: &Expression) -> Result<(), Error>;
}

/// A semantic analyzer which analyzes the AST.
///
/// # Fields
///
/// * `ast` - The AST to analyze.
/// * `scopes` - The scopes found.
#[derive(Debug)]
pub struct SemanticAnalyzer<'a> {
    ast: &'a AST,
    scopes: Vec<Scope>,
}

impl<'a> SemanticAnalyzer<'a> {
    /// Creates a new semantic analyzer.
    ///
    /// # Arguments
    ///
    /// * `ast` - The AST to analyze.
    ///
    /// # Returns
    ///
    /// * `SemanticAnalyzer` - The new semantic analyzer.
    #[must_use]
    pub fn new(ast: &'a AST) -> Self {
        Self {
            ast,
            scopes: vec![Scope::default()],
        }
    }
    fn current_scope(&mut self) -> Result<&mut Scope, Error> {
        self.scopes.last_mut().ok_or(Error::InvalidScope)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    /// Analyzes the AST.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the analysis.
    ///
    /// # Errors
    ///
    /// * If the scope is invalid.
    pub fn analyze(&mut self) -> Result<(), Error> {
        for statement in &self.ast.statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn get_symbol(&mut self, token: &Token) -> Result<&mut SymbolKind, Error> {
        let name = token.token_kind.to_string();

        for scope in self.scopes.iter_mut().rev() {
            if let Some(symbol) = scope.symbol_table.get_mut(&*name) {
                return Ok(symbol);
            }
        }

        Err(Error::UndefinedSymbol {
            name,
            line: token.line,
            column: token.column,
        })
    }
}

impl<'a> Visitor for SemanticAnalyzer<'a> {
    #[allow(clippy::expect_used)]
    fn visit_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression { expression } => self.visit_expression(expression),
            Statement::Variable { name, initializer } => {
                let is_initialized = initializer.is_some();
                if is_initialized {
                    self.visit_expression(initializer.as_ref().expect("Initializer is None!"))?;
                }

                self.current_scope()?.define(
                    &name.token_kind.to_string(),
                    SymbolKind::Variable { is_initialized },
                );

                Ok(())
            }
            Statement::Block { statements } => {
                self.begin_scope();
                for statement in statements {
                    self.visit_statement(statement)?;
                }
                self.end_scope();

                Ok(())
            }
            Statement::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.visit_expression(condition)?;
                self.visit_statement(then_branch)?;

                if let Some(else_branch) = else_branch {
                    self.visit_statement(else_branch)?;
                }

                Ok(())
            }
            Statement::While { condition, body } => {
                self.visit_expression(condition)?;
                self.visit_statement(body)?;

                Ok(())
            }
            Statement::For {
                initializer,
                condition,
                increment,
                body,
            } => {
                if let Some(initializer) = initializer {
                    self.visit_statement(initializer)?;
                }
                if let Some(condition) = condition {
                    self.visit_expression(condition)?;
                }
                if let Some(increment) = increment {
                    self.visit_expression(increment)?;
                }
                self.visit_statement(body)?;

                Ok(())
            }
            Statement::Function {
                name,
                parameters,
                return_type,
                body,
            } => {
                self.current_scope()?.define(
                    &name.token_kind.to_string(),
                    SymbolKind::Function {
                        return_type: return_type.as_ref().cloned(),
                    },
                );

                self.begin_scope();

                // Define the parameters.
                for (name, kind) in parameters {
                    if !matches!(kind.token_kind, TokenKind::Identifier(_)) {
                        return Err(Error::InvalidParameterKind {
                            name: name.token_kind.to_string(),
                            line: name.line,
                            column: name.column,
                        });
                    }

                    self.current_scope()?.define(
                        &name.token_kind.to_string(),
                        SymbolKind::Variable { is_initialized: true },
                    );
                }

                // Examine the body.
                self.visit_statement(body)?;
                
                self.end_scope();

                Ok(())
            }
            Statement::Return { value, .. } => {
                if let Some(value) = value {
                    self.visit_expression(value)?;
                }

                Ok(())
            }
            Statement::Break | Statement::Continue => Ok(()),
        }
    }

    fn visit_expression(&mut self, expression: &Expression) -> Result<(), Error> {
        match expression {
            Expression::Literal(_) => {}
            Expression::Variable { name } => {
                // If the variable is not defined, this will return an error.
                let symbol = self.get_symbol(name)?;

                // If the variable is not initialized, this will return an error.
                if let SymbolKind::Variable { is_initialized } = &symbol {
                    if !is_initialized {
                        return Err(Error::UninitializedVariable {
                            name: name.token_kind.to_string(),
                            line: name.line,
                            column: name.column,
                        });
                    }
                }
            }
            Expression::Assignment { name, value } => {
                self.visit_expression(value)?;

                // If the variable is not defined, this will return an error.
                let symbol = self.get_symbol(name)?;

                if matches!(symbol, SymbolKind::Function { .. }) {
                    return Err(Error::InvalidAssignment {
                        name: name.token_kind.to_string(),
                        line: name.line,
                        column: name.column,
                    });
                }
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                self.visit_expression(left)?;

                // Make sure the operator makes sense.
                match operator.token_kind {
                    TokenKind::Plus | TokenKind::Minus | TokenKind::Star | TokenKind::Slash => {
                        self.visit_expression(right)?;
                    }
                    TokenKind::Equality
                    | TokenKind::NotEqual
                    | TokenKind::LessThan
                    | TokenKind::LessThanOrEqual
                    | TokenKind::GreaterThan
                    | TokenKind::GreaterThanOrEqual => {}
                    _ => {
                        return Err(Error::InvalidOperator {
                            operator: operator.token_kind.to_string(),
                            line: operator.line,
                            column: operator.column,
                        });
                    }
                }
            }
            Expression::Unary { operator, right } => {
                // Make sure the operator makes sense.
                match operator.token_kind {
                    TokenKind::Plus | TokenKind::Minus | TokenKind::LogicalNot => {
                        self.visit_expression(right)?;
                    }
                    _ => {
                        return Err(Error::InvalidOperator {
                            operator: operator.token_kind.to_string(),
                            line: operator.line,
                            column: operator.column,
                        });
                    }
                }
            }
            Expression::Call { callee, arguments } => {
                self.visit_expression(callee)?;

                for argument in arguments {
                    self.visit_expression(argument)?;
                }
            }
            Expression::Grouping { expression } => {
                self.visit_expression(expression)?;
            }
        }

        Ok(())
    }
}
