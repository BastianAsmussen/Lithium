use lexer::token::{Kind, Token};
use lexer::Lexer;
use parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_if() {
    let input = r#"
        let x = 1;

        if (x == 1) {
            print("x is 1.");
        }
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("x".into()), 1, 5),
            initializer: Some(Expression::Literal(Literal::Number(1.0))),
        },
        Statement::If {
            condition: Expression::Binary {
                left: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("x".into()), 3, 14),
                }),
                operator: Token::new(Kind::Equality, 3, 16),
                right: Box::from(Expression::Literal(Literal::Number(1.0))),
            },
            then_branch: Box::from(Statement::Block {
                statements: vec![Statement::Expression {
                    expression: Expression::Call {
                        callee: Box::from(Expression::Variable {
                            name: Token::new(Kind::Identifier("print".into()), 4, 18),
                        }),
                        arguments: vec![Expression::Literal(Literal::String("x is 1.".into()))],
                    },
                }],
            }),
            else_branch: None,
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_if_else() {
    let input = r#"
        let x = 1;

        if (x == 1) {
            print("x is 1.");
        } else {
            print("x is not 1.");
        }
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("x".into()), 1, 5),
            initializer: Some(Expression::Literal(Literal::Number(1.0))),
        },
        Statement::If {
            condition: Expression::Binary {
                left: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("x".into()), 3, 14),
                }),
                operator: Token::new(Kind::Equality, 3, 16),
                right: Box::from(Expression::Literal(Literal::Number(1.0))),
            },
            then_branch: Box::from(Statement::Block {
                statements: vec![Statement::Expression {
                    expression: Expression::Call {
                        callee: Box::from(Expression::Variable {
                            name: Token::new(Kind::Identifier("print".into()), 4, 18),
                        }),
                        arguments: vec![Expression::Literal(Literal::String("x is 1.".into()))],
                    },
                }],
            }),
            else_branch: Some(Box::from(Statement::Block {
                statements: vec![Statement::Expression {
                    expression: Expression::Call {
                        callee: Box::from(Expression::Variable {
                            name: Token::new(Kind::Identifier("print".into()), 6, 18),
                        }),
                        arguments: vec![Expression::Literal(Literal::String("x is not 1.".into()))],
                    },
                }],
            })),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_else_if() {
    let input = r#"
        let x = 1;

        if (x == 1) {
            print("x is 1.");
        } else if (x == 2) {
            print("x is 2.");
        } else {
            print("x is not 1 or 2.");
        }
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("x".into()), 1, 5),
            initializer: Some(Expression::Literal(Literal::Number(1.0))),
        },
        Statement::If {
            condition: Expression::Binary {
                left: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("x".into()), 3, 14),
                }),
                operator: Token::new(Kind::Equality, 3, 16),
                right: Box::from(Expression::Literal(Literal::Number(1.0))),
            },
            then_branch: Box::from(Statement::Block {
                statements: vec![Statement::Expression {
                    expression: Expression::Call {
                        callee: Box::from(Expression::Variable {
                            name: Token::new(Kind::Identifier("print".into()), 4, 18),
                        }),
                        arguments: vec![Expression::Literal(Literal::String("x is 1.".into()))],
                    },
                }],
            }),
            else_branch: Some(Box::from(Statement::If {
                condition: Expression::Binary {
                    left: Box::from(Expression::Variable {
                        name: Token::new(Kind::Identifier("x".into()), 5, 21),
                    }),
                    operator: Token::new(Kind::Equality, 5, 23),
                    right: Box::from(Expression::Literal(Literal::Number(2.0))),
                },
                then_branch: Box::from(Statement::Block {
                    statements: vec![Statement::Expression {
                        expression: Expression::Call {
                            callee: Box::from(Expression::Variable {
                                name: Token::new(Kind::Identifier("print".into()), 6, 18),
                            }),
                            arguments: vec![Expression::Literal(Literal::String("x is 2.".into()))],
                        },
                    }],
                }),
                else_branch: Some(Box::from(Statement::Block {
                    statements: vec![Statement::Expression {
                        expression: Expression::Call {
                            callee: Box::from(Expression::Variable {
                                name: Token::new(Kind::Identifier("print".into()), 8, 18),
                            }),
                            arguments: vec![Expression::Literal(Literal::String(
                                "x is not 1 or 2.".into(),
                            ))],
                        },
                    }],
                })),
            })),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_else_only() {
    let input = r#"
        else {
            print("Hello, World!");
        }
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let ast = parser.parse();

    assert!(ast.is_err());
}
