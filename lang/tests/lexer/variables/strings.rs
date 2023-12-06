use lang::lexer::{tokens::TokenKind, Lexer};

#[test]
#[allow(clippy::unwrap_used)]
fn test_normal() {
    let input = r#"
        let a = "Hello, World!";
     "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<_> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::String("Hello, World!".into()),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
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

    let actual_token_kinds: Vec<_> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::String("Hello, \"World!\"".into()),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
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

    let actual_token_kinds: Vec<_> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::String("Hello, \nWorld!".into()),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
