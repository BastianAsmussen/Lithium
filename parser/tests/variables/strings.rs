use lexer::token::{Kind, Token};
use lexer::Lexer;
use parser::{Expression, Literal, Parser, Statement};

#[test]
#[allow(clippy::unwrap_used)]
fn test_string() {
    let input = r#"
        let greeting = "Hello, World!";
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(tokens);
    let actual_ast = parser.parse().unwrap();
    let expected_ast = vec![Statement::Variable {
        name: Token::new(Kind::Identifier("greeting".into()), 1, 12),
        initializer: Some(Expression::Literal(Literal::String("Hello, World!".into()))),
    }];

    assert_eq!(actual_ast, expected_ast);
}
