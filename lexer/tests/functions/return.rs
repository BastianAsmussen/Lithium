use lexer::token::Kind;
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

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Function,
        Kind::Identifier("factorial".into()),
        Kind::LeftParenthesis,
        Kind::Identifier("n".into()),
        Kind::Colon,
        Kind::Identifier("int".into()),
        Kind::RightParenthesis,
        Kind::Arrow,
        Kind::Identifier("int".into()),
        Kind::LeftCurlyBrace,
        Kind::If,
        Kind::Identifier("n".into()),
        Kind::Equality,
        Kind::Integer(0),
        Kind::LeftCurlyBrace,
        Kind::Return,
        Kind::Integer(1),
        Kind::Semicolon,
        Kind::RightCurlyBrace,
        Kind::Return,
        Kind::Identifier("n".into()),
        Kind::Star,
        Kind::Identifier("factorial".into()),
        Kind::LeftParenthesis,
        Kind::Identifier("n".into()),
        Kind::Minus,
        Kind::Integer(1),
        Kind::RightParenthesis,
        Kind::Semicolon,
        Kind::RightCurlyBrace,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
