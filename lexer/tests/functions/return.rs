use lexer::token::TokenKind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_return() {
    let input = r"
        fn factorial(n: int) -> int {
            if n == 0 {
                return 1;
            }

            return n * factorial(n - 1);
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Function,
        TokenKind::Identifier("factorial".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("n".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::RightParenthesis,
        TokenKind::Arrow,
        TokenKind::Identifier("int".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::If,
        TokenKind::Identifier("n".into()),
        TokenKind::Equality,
        TokenKind::Integer(0),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Return,
        TokenKind::Identifier("n".into()),
        TokenKind::Asterisk,
        TokenKind::Identifier("factorial".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("n".into()),
        TokenKind::Minus,
        TokenKind::Integer(1),
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
