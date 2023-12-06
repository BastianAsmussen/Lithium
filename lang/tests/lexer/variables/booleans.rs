use lang::lexer::tokens::TokenKind;
use lang::lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_true() {
    let input = r"
        let truthy = true;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("truthy".into()),
        TokenKind::Assign,
        TokenKind::True,
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_false() {
    let input = r"
        let falsy = false;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> = tokens
        .iter()
        .map(|token| token.token_kind.clone())
        .collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("falsy".into()),
        TokenKind::Assign,
        TokenKind::False,
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
