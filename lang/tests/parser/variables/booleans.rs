use lang::lexer::{token::Kind, token::Token, Lexer};
use lang::parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_true() {
    let input = r"
        let yes = true;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![Statement::Variable {
        name: Token::new(Kind::Identifier("yes".into()), 1, 7),
        initializer: Some(Expression::Literal(Literal::Boolean(true))),
    }];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_false() {
    let input = r"
        let no = false;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![Statement::Variable {
        name: Token::new(Kind::Identifier("no".into()), 1, 6),
        initializer: Some(Expression::Literal(Literal::Boolean(false))),
    }];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_not() {
    let input = r"
        let yes = !false;
        let no = !true;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Unary {
                operator: Token::new(Kind::LogicalNot, 1, 10),
                right: Box::from(Expression::Literal(Literal::Boolean(false))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 2, 15),
            initializer: Some(Expression::Unary {
                operator: Token::new(Kind::LogicalNot, 2, 18),
                right: Box::from(Expression::Literal(Literal::Boolean(true))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_and() {
    let input = r"
        let yes = true && true;
        let no = true && false;
        let no = false && true;
        let no = false && false;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(true))),
                operator: Token::new(Kind::LogicalAnd, 1, 16),
                right: Box::from(Expression::Literal(Literal::Boolean(true))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 2, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(true))),
                operator: Token::new(Kind::LogicalAnd, 2, 24),
                right: Box::from(Expression::Literal(Literal::Boolean(false))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 3, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(false))),
                operator: Token::new(Kind::LogicalAnd, 3, 25),
                right: Box::from(Expression::Literal(Literal::Boolean(true))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 4, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(false))),
                operator: Token::new(Kind::LogicalAnd, 4, 25),
                right: Box::from(Expression::Literal(Literal::Boolean(false))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_or() {
    let input = r"
        let yes = true || true;
        let yes = true || false;
        let yes = false || true;
        let no = false || false;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(true))),
                operator: Token::new(Kind::LogicalOr, 1, 16),
                right: Box::from(Expression::Literal(Literal::Boolean(true))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 2, 16),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(true))),
                operator: Token::new(Kind::LogicalOr, 2, 25),
                right: Box::from(Expression::Literal(Literal::Boolean(false))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 3, 16),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(false))),
                operator: Token::new(Kind::LogicalOr, 3, 26),
                right: Box::from(Expression::Literal(Literal::Boolean(true))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 4, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Boolean(false))),
                operator: Token::new(Kind::LogicalOr, 4, 25),
                right: Box::from(Expression::Literal(Literal::Boolean(false))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_greater_than() {
    let input = r"
        let yes = 1 < 2;
        let no = 2 < 1;
        let no = 2 < 2;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(1.0))),
                operator: Token::new(Kind::LessThan, 1, 12),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 2, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(2.0))),
                operator: Token::new(Kind::LessThan, 2, 20),
                right: Box::from(Expression::Literal(Literal::Number(1.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 3, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(2.0))),
                operator: Token::new(Kind::LessThan, 3, 20),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_greater_than_or_equal() {
    let input = r"
        let yes = 1 <= 2;
        let yes = 2 <= 2;
        let no = 3 <= 2;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(1.0))),
                operator: Token::new(Kind::LessThanOrEqual, 1, 13),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 2, 16),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(2.0))),
                operator: Token::new(Kind::LessThanOrEqual, 2, 22),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 3, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(3.0))),
                operator: Token::new(Kind::LessThanOrEqual, 3, 21),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_less_than() {
    let input = r"
        let yes = 1 > 2;
        let no = 2 > 1;
        let no = 2 > 2;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(1.0))),
                operator: Token::new(Kind::GreaterThan, 1, 12),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 2, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(2.0))),
                operator: Token::new(Kind::GreaterThan, 2, 20),
                right: Box::from(Expression::Literal(Literal::Number(1.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 3, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(2.0))),
                operator: Token::new(Kind::GreaterThan, 3, 20),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_less_than_or_equal() {
    let input = r"
        let yes = 1 >= 2;
        let yes = 2 >= 2;
        let no = 3 >= 2;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 1, 7),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(1.0))),
                operator: Token::new(Kind::GreaterThanOrEqual, 1, 13),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 2, 16),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(2.0))),
                operator: Token::new(Kind::GreaterThanOrEqual, 2, 22),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 3, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Literal(Literal::Number(3.0))),
                operator: Token::new(Kind::GreaterThanOrEqual, 3, 21),
                right: Box::from(Expression::Literal(Literal::Number(2.0))),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_comparison() {
    let input = r"
        let a = 1;
        let b = 2;

        let yes = a != b;
        let no = a == b;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![
        Statement::Variable {
            name: Token::new(Kind::Identifier("a".into()), 1, 5),
            initializer: Some(Expression::Literal(Literal::Number(1.0))),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("b".into()), 2, 14),
            initializer: Some(Expression::Literal(Literal::Number(2.0))),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("yes".into()), 4, 16),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("a".into()), 4, 20),
                }),
                operator: Token::new(Kind::NotEqual, 4, 22),
                right: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("b".into()), 4, 25),
                }),
            }),
        },
        Statement::Variable {
            name: Token::new(Kind::Identifier("no".into()), 5, 15),
            initializer: Some(Expression::Binary {
                left: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("a".into()), 5, 19),
                }),
                operator: Token::new(Kind::Equality, 5, 21),
                right: Box::from(Expression::Variable {
                    name: Token::new(Kind::Identifier("b".into()), 5, 24),
                }),
            }),
        },
    ];

    assert_eq!(actual_ast, expected_ast);
}
