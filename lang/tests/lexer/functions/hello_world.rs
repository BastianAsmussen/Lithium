use lang::lexer::{token::Kind, Lexer};

#[test]
#[allow(clippy::unwrap_used)]
fn test_hello_world() {
    let input = r#"
        // This is a comment.
        fn greet(name: str) -> str {
            return "Hello, " + name + "!";
        }

        print(greet("World"));
    "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<_> = tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        Kind::Function,
        Kind::Identifier("greet".into()),
        Kind::LeftParenthesis,
        Kind::Identifier("name".into()),
        Kind::Colon,
        Kind::Identifier("str".into()),
        Kind::RightParenthesis,
        Kind::Arrow,
        Kind::Identifier("str".into()),
        Kind::LeftCurlyBrace,
        Kind::Return,
        Kind::String("Hello, ".into()),
        Kind::Plus,
        Kind::Identifier("name".into()),
        Kind::Plus,
        Kind::String("!".into()),
        Kind::Semicolon,
        Kind::RightCurlyBrace,
        Kind::Identifier("print".into()),
        Kind::LeftParenthesis,
        Kind::Identifier("greet".into()),
        Kind::LeftParenthesis,
        Kind::String("World".into()),
        Kind::RightParenthesis,
        Kind::RightParenthesis,
        Kind::Semicolon,
        Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
