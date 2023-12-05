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
        lexer::token::Kind::Variable,
        lexer::token::Kind::Identifier("a".into()),
        lexer::token::Kind::Assign,
        lexer::token::Kind::String("Hello, World!".into()),
        lexer::token::Kind::Semicolon,
        lexer::token::Kind::EndOfFile,
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
        lexer::token::Kind::Variable,
        lexer::token::Kind::Identifier("a".into()),
        lexer::token::Kind::Assign,
        lexer::token::Kind::String("Hello, \"World!\"".into()),
        lexer::token::Kind::Semicolon,
        lexer::token::Kind::EndOfFile,
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
        lexer::token::Kind::Variable,
        lexer::token::Kind::Identifier("a".into()),
        lexer::token::Kind::Assign,
        lexer::token::Kind::String("Hello, \nWorld!".into()),
        lexer::token::Kind::Semicolon,
        lexer::token::Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
