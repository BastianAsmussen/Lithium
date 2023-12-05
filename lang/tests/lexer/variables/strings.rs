use lang::lexer::{token::Kind, Lexer};

#[test]
#[allow(clippy::unwrap_used)]
fn test_normal() {
    let input = r#"
        let a = "Hello, World!";
     "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<_> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("a".into()),
        Kind::Assign,
        Kind::String("Hello, World!".into()),
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_escape() {
    let input = r#"
        let a = "Hello, \"World!\"";
     "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<_> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("a".into()),
        Kind::Assign,
        Kind::String("Hello, \"World!\"".into()),
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_newline() {
    let input = r#"
        let a = "Hello, \nWorld!";
     "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<_> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("a".into()),
        Kind::Assign,
        Kind::String("Hello, \nWorld!".into()),
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
