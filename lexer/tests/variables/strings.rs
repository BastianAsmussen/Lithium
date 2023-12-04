use lexer::Lexer;

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
        lexer::token::TokenKind::Variable,
        lexer::token::TokenKind::Identifier("a".into()),
        lexer::token::TokenKind::Assign,
        lexer::token::TokenKind::String("Hello, World!".into()),
        lexer::token::TokenKind::Semicolon,
        lexer::token::TokenKind::EndOfFile,
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
        lexer::token::TokenKind::Variable,
        lexer::token::TokenKind::Identifier("a".into()),
        lexer::token::TokenKind::Assign,
        lexer::token::TokenKind::String("Hello, \"World!\"".into()),
        lexer::token::TokenKind::Semicolon,
        lexer::token::TokenKind::EndOfFile,
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
        lexer::token::TokenKind::Variable,
        lexer::token::TokenKind::Identifier("a".into()),
        lexer::token::TokenKind::Assign,
        lexer::token::TokenKind::String("Hello, \nWorld!".into()),
        lexer::token::TokenKind::Semicolon,
        lexer::token::TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
