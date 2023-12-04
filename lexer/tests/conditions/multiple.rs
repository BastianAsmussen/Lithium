use lexer::token::TokenKind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_multiple() {
    let input = r"
        let a = 10;
        if a == 10 {
            // ...
        } else if a == 20 {
            // ...
        } else {
            // ...
        }
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
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::Equality,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::RightCurlyBrace,
        TokenKind::Else,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::Equality,
        TokenKind::Integer(20),
        TokenKind::LeftCurlyBrace,
        TokenKind::RightCurlyBrace,
        TokenKind::Else,
        TokenKind::LeftCurlyBrace,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
