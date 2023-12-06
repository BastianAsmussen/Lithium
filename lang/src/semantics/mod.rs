use crate::parser::ast::AST;
use crate::parser::{Expression, Literal, Statement};
use crate::semantics::errors::Error;
use crate::semantics::symbols::{Scope, Symbol, SymbolKind, SymbolTable};
use std::collections::HashMap;

pub mod errors;
pub mod symbols;

/// The semantic analyzer.
///
/// # Fields
///
/// * `ast` - The abstract syntax tree of the program.
///
/// * `scopes` - The current scope.
/// * `current_scope` - The current scope.
#[derive(Debug)]
pub struct Semantics<'a> {
    ast: &'a AST,

    scopes: HashMap<Scope, SymbolTable>,
    current_scope: Scope,
}

impl<'a> Semantics<'a> {
    /// Creates a new semantic analyzer.
    ///
    /// # Arguments
    ///
    /// * `ast` - The abstract syntax tree of the program.
    ///
    /// # Returns
    ///
    /// * The new semantic analyzer.
    #[must_use]
    pub fn new(ast: &'a AST) -> Self {
        Self {
            ast,

            scopes: std::iter::once(&(Scope::Root, SymbolTable::default()))
                .cloned()
                .collect(),
            current_scope: Scope::Root,
        }
    }

    /// Analyzes the program.
    ///
    /// # Arguments
    ///
    /// * `self` - The semantic analyzer.
    ///
    /// # Returns
    ///
    /// * `Result<(), Error>` - The result of the analysis.
    ///
    /// # Errors
    ///
    /// * `ScopeNotDefined` - The scope is not defined.
    /// * `SymbolNotDefined` - The symbol is not defined.
    /// * `SymbolAlreadyDefined` - The symbol is already defined.
    pub fn analyze(&mut self) -> Result<(), Error> {
        for statement in &self.ast.statements {
            self.visit_statement(statement)?;
        }

        Ok(())
    }

    fn visit_statement(&mut self, statement: &'a Statement) -> Result<(), Error> {
        match statement {
            Statement::Expression { expression } => self.visit_expression(expression),
            Statement::Variable { name, initializer } => {
                if let Some(initializer) = initializer {
                    self.visit_expression(initializer)?;
                }

                self.get_current_symbol_table()?.add_symbol(Symbol::new(
                    &name.token_kind.to_string(),
                    SymbolKind::Variable,
                ))
            }
            Statement::Block { statements } => {
                self.update_current_scope();

                for statement in statements {
                    self.visit_statement(statement)?;
                }

                self.revert_current_scope();

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
            Statement::Return { keyword, value } => {
                self.get_current_symbol_table()?.add_symbol(Symbol::new(
                    &keyword.token_kind.to_string(),
                    SymbolKind::BuiltIn,
                ))?;

                if let Some(value) = value {
                    self.visit_expression(value)?;
                }

                Ok(())
            }
            Statement::Function {
                name,
                parameters,
                return_type,
                body,
            } => {
                self.get_current_symbol_table()?.add_symbol(Symbol::new(
                    &name.token_kind.to_string(),
                    SymbolKind::Function,
                ))?;

                self.update_current_scope();

                for (name, kind) in parameters {
                    self.get_current_symbol_table()?.add_symbol(Symbol::new(
                        &name.token_kind.to_string(),
                        SymbolKind::Variable,
                    ))?;
                    self.get_current_symbol_table()?.add_symbol(Symbol::new(
                        &kind.token_kind.to_string(),
                        SymbolKind::BuiltIn,
                    ))?;
                }

                if let Some(return_type) = return_type {
                    self.get_current_symbol_table()?.add_symbol(Symbol::new(
                        &return_type.token_kind.to_string(),
                        SymbolKind::BuiltIn,
                    ))?;
                }

                self.visit_statement(body)?;

                self.revert_current_scope();

                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn visit_expression(&mut self, expression: &'a Expression) -> Result<(), Error> {
        match expression {
            Expression::Literal(literal) => self.visit_literal(literal),
            Expression::Unary { operator, right } => {
                self.get_current_symbol_table()?.add_symbol(Symbol::new(
                    &operator.token_kind.to_string(),
                    SymbolKind::BuiltIn,
                ))?;

                self.visit_expression(right)
            }
            Expression::Binary {
                left,
                operator,
                right,
            } => {
                self.get_current_symbol_table()?.add_symbol(Symbol::new(
                    &operator.token_kind.to_string(),
                    SymbolKind::BuiltIn,
                ))?;

                self.visit_expression(left)?;
                self.visit_expression(right)
            }
            Expression::Grouping { expression } => self.visit_expression(expression),
            Expression::Assignment { name, value } => {
                self.visit_expression(value)?;

                self.get_current_symbol_table()?.add_symbol(Symbol::new(
                    &name.token_kind.to_string(),
                    SymbolKind::Variable,
                ))
            }
            Expression::Variable { name } => {
                self.get_current_symbol_table()?
                    .get_symbol(&name.token_kind.to_string())?;

                Ok(())
            }
            Expression::Call { callee, arguments } => {
                self.visit_expression(callee)?;

                for argument in arguments {
                    self.visit_expression(argument)?;
                }

                Ok(())
            }
        }
    }

    fn visit_literal(&mut self, literal: &'a Literal) -> Result<(), Error> {
        let symbol = match literal {
            Literal::String(string) => Symbol::new(string, SymbolKind::BuiltIn),
            Literal::Number(number) => Symbol::new(&number.to_string(), SymbolKind::BuiltIn),
            Literal::Boolean(boolean) => Symbol::new(&boolean.to_string(), SymbolKind::BuiltIn),
        };

        self.get_current_symbol_table()?.add_symbol(symbol)
    }

    /// Gets the current scope.
    ///
    /// # Returns
    ///
    /// * `Result<&SymbolTable, Error>` - The current scope.
    fn get_current_symbol_table(&mut self) -> Result<&mut SymbolTable, Error> {
        match self.scopes.get_mut(&self.current_scope) {
            Some(symbol_table) => Ok(symbol_table),
            None => Err(Error::ScopeNotDefined {
                scope: self.current_scope.clone(),
            }),
        }
    }

    /// Updates the current scope.
    fn update_current_scope(&mut self) {
        self.current_scope = Scope::Child(Box::new(self.current_scope.clone()));
        self.scopes
            .insert(self.current_scope.clone(), SymbolTable::default());
    }

    /// Reverts the current scope to its parent.
    fn revert_current_scope(&mut self) {
        self.current_scope = match &self.current_scope {
            Scope::Child(parent) => *parent.clone(),
            Scope::Root => Scope::Root,
        };
    }
}
