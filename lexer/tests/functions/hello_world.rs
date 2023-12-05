use lexer::Lexer;

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
        lexer::token::Kind::Function,
        lexer::token::Kind::Identifier("greet".into()),
        lexer::token::Kind::LeftParenthesis,
        lexer::token::Kind::Identifier("name".into()),
        lexer::token::Kind::Colon,
        lexer::token::Kind::Identifier("str".into()),
        lexer::token::Kind::RightParenthesis,
        lexer::token::Kind::Arrow,
        lexer::token::Kind::Identifier("str".into()),
        lexer::token::Kind::LeftCurlyBrace,
        lexer::token::Kind::Return,
        lexer::token::Kind::String("Hello, ".into()),
        lexer::token::Kind::Plus,
        lexer::token::Kind::Identifier("name".into()),
        lexer::token::Kind::Plus,
        lexer::token::Kind::String("!".into()),
        lexer::token::Kind::Semicolon,
        lexer::token::Kind::RightCurlyBrace,
        lexer::token::Kind::Identifier("print".into()),
        lexer::token::Kind::LeftParenthesis,
        lexer::token::Kind::Identifier("greet".into()),
        lexer::token::Kind::LeftParenthesis,
        lexer::token::Kind::String("World".into()),
        lexer::token::Kind::RightParenthesis,
        lexer::token::Kind::RightParenthesis,
        lexer::token::Kind::Semicolon,
        lexer::token::Kind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
