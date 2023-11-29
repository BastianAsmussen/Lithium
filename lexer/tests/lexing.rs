use lexer::token::TokenKind;
use lexer::Lexer;

#[test]
#[allow(clippy::unwrap_used)]
fn test_function() {
    let input = r"
        fn add(a: int, b: int) -> int {
            return a + b;
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Function,
        TokenKind::Identifier("add".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("a".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::Comma,
        TokenKind::Identifier("b".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::RightParenthesis,
        TokenKind::Minus,
        TokenKind::Greater,
        TokenKind::Identifier("int".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::Identifier("a".into()),
        TokenKind::Plus,
        TokenKind::Identifier("b".into()),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_variable() {
    let input = r#"
        let a = 10;
        let b = true;
        let c = "Hello, world!";
        "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Equal,
        TokenKind::Integer(10),
        TokenKind::Semicolon,
        TokenKind::Variable,
        TokenKind::Identifier("b".into()),
        TokenKind::Equal,
        TokenKind::True,
        TokenKind::Semicolon,
        TokenKind::Variable,
        TokenKind::Identifier("c".into()),
        TokenKind::Equal,
        TokenKind::String("Hello, world!".into()),
        TokenKind::Semicolon,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_if_else() {
    let input = r#"
        let a = 10;
        if a < 10 {
            println("a is less than 10!");
        } else if a > 10 {
            println("a is greater than 10!");
        } else {
            println("a is equal to 10!");
        }
        "#;

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Equal,
        TokenKind::Integer(10),
        TokenKind::Semicolon,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::Less,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("println".into()),
        TokenKind::LeftParenthesis,
        TokenKind::String("a is less than 10!".into()),
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Else,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::Greater,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("println".into()),
        TokenKind::LeftParenthesis,
        TokenKind::String("a is greater than 10!".into()),
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Else,
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("println".into()),
        TokenKind::LeftParenthesis,
        TokenKind::String("a is equal to 10!".into()),
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_while() {
    let input = r"
        let a = 0;
        while a < 10 {
            a += 1;
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Equal,
        TokenKind::Integer(0),
        TokenKind::Semicolon,
        TokenKind::While,
        TokenKind::Identifier("a".into()),
        TokenKind::Less,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("a".into()),
        TokenKind::PlusEqual,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_for() {
    let input = r"
        for i in 0..10 {
            println(i);
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::For,
        TokenKind::Identifier("i".into()),
        TokenKind::In,
        TokenKind::Integer(0),
        TokenKind::Dot,
        TokenKind::Dot,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("println".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("i".into()),
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_break() {
    let input = r"
        let a = 0;
        while a < 10 {
            if a == 5 {
                break;
            }

            a += 1;
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Equal,
        TokenKind::Integer(0),
        TokenKind::Semicolon,
        TokenKind::While,
        TokenKind::Identifier("a".into()),
        TokenKind::Less,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::EqualEqual,
        TokenKind::Integer(5),
        TokenKind::LeftCurlyBrace,
        TokenKind::Break,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Identifier("a".into()),
        TokenKind::PlusEqual,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_continue() {
    let input = r"
        let a = 0;
        while a < 10 {
            if a == 5 {
                continue;
            }

            a += 1;
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Equal,
        TokenKind::Integer(0),
        TokenKind::Semicolon,
        TokenKind::While,
        TokenKind::Identifier("a".into()),
        TokenKind::Less,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::EqualEqual,
        TokenKind::Integer(5),
        TokenKind::LeftCurlyBrace,
        TokenKind::Continue,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Identifier("a".into()),
        TokenKind::PlusEqual,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_return() {
    let input = r"
        fn add(a: int, b: int) -> int {
            return a + b;
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Function,
        TokenKind::Identifier("add".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("a".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::Comma,
        TokenKind::Identifier("b".into()),
        TokenKind::Colon,
        TokenKind::Identifier("int".into()),
        TokenKind::RightParenthesis,
        TokenKind::Minus,
        TokenKind::Greater,
        TokenKind::Identifier("int".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::Identifier("a".into()),
        TokenKind::Plus,
        TokenKind::Identifier("b".into()),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_comments() {
    let input = r"
        // This is a comment.
        let a = 10; // This is another comment.
        /* This is a multi-line comment.
        It spans multiple lines. */
        let b = 20; /* This is another multi-line comment.
        It also spans multiple lines. */
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.lex().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Equal,
        TokenKind::Integer(10),
        TokenKind::Semicolon,
        TokenKind::Variable,
        TokenKind::Identifier("b".into()),
        TokenKind::Equal,
        TokenKind::Integer(20),
        TokenKind::Semicolon,
        TokenKind::EOF,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
