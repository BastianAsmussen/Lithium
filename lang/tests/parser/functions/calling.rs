use lang::lexer::{tokens::Token, tokens::TokenKind, Lexer};
use lang::parser::ast::AST;
use lang::parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_greet() {
    let input = r#"
        fn greet(name: str) {
            print("Hello, " + name + "!");
        }

        greet("World");
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = AST::new(vec![
        Statement::Function {
            name: Token::new(TokenKind::Identifier("greet".into()), 1, 8),
            parameters: vec![(
                Token::new(TokenKind::Identifier("name".into()), 1, 13),
                Token::new(TokenKind::Identifier("str".into()), 1, 18),
            )],
            return_type: None,
            body: Box::from(Statement::Block {
                statements: vec![Statement::Expression {
                    expression: Expression::Call {
                        callee: Box::from(Expression::Variable {
                            name: Token::new(TokenKind::Identifier("print".into()), 2, 18),
                        }),
                        arguments: vec![Expression::Binary {
                            left: Box::from(Expression::Binary {
                                left: Box::from(Expression::Literal(Literal::String(
                                    "Hello, ".into(),
                                ))),
                                operator: Token::new(TokenKind::Plus, 2, 29),
                                right: Box::from(Expression::Variable {
                                    name: Token::new(TokenKind::Identifier("name".into()), 2, 35),
                                }),
                            }),
                            operator: Token::new(TokenKind::Plus, 2, 36),
                            right: Box::from(Expression::Literal(Literal::String("!".into()))),
                        }],
                    },
                }],
            }),
        },
        Statement::Expression {
            expression: Expression::Call {
                callee: Box::from(Expression::Variable {
                    name: Token::new(TokenKind::Identifier("greet".into()), 5, 14),
                }),
                arguments: vec![Expression::Literal(Literal::String("World".into()))],
            },
        },
    ]);

    assert_eq!(actual_ast, expected_ast);
}
