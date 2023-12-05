use lang::lexer::{token::Kind, Lexer};

#[test]
#[allow(clippy::unwrap_used)]
fn test_function() {
    let input = r"
        fn add(a: int, b: int) -> int {
            return a + b;
        }
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<Kind> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Function,
        Kind::Identifier("add".into()),
        Kind::LeftParenthesis,
        Kind::Identifier("a".into()),
        Kind::Colon,
        Kind::Identifier("int".into()),
        Kind::Comma,
        Kind::Identifier("b".into()),
        Kind::Colon,
        Kind::Identifier("int".into()),
        Kind::RightParenthesis,
        Kind::Arrow,
        Kind::Identifier("int".into()),
        Kind::LeftCurlyBrace,
        Kind::Return,
        Kind::Identifier("a".into()),
        Kind::Plus,
        Kind::Identifier("b".into()),
        Kind::Semicolon,
        Kind::RightCurlyBrace,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
