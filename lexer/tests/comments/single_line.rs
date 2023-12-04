use lexer::token::TokenKind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_single_line() {
    let input = r"
        // This is a single-line comment.
        let a = 10; // This is another single-line comment.
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::Integer(10),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
