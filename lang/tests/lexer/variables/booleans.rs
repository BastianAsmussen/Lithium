use lang::lexer::token::Kind;
use lang::lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_true() {
    let input = r"
        let truthy = true;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("truthy".into()),
        Kind::Assign,
        Kind::True,
        Kind::Semicolon,
        Kind::EndOfFile,
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

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Variable,
        Kind::Identifier("falsy".into()),
        Kind::Assign,
        Kind::False,
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
