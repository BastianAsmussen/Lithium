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
    let tokens = lexer.tokenize().unwrap();

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
        TokenKind::Arrow,
        TokenKind::Identifier("int".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::Identifier("a".into()),
        TokenKind::Plus,
        TokenKind::Identifier("b".into()),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_variables() {
    let input = r#"
        let a = 10;
        let b = true;
        let c = "Hello, world!";
        "#;

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
        TokenKind::Variable,
        TokenKind::Identifier("b".into()),
        TokenKind::Assign,
        TokenKind::True,
        TokenKind::Semicolon,
        TokenKind::Variable,
        TokenKind::Identifier("c".into()),
        TokenKind::Assign,
        TokenKind::String("Hello, world!".into()),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
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
        TokenKind::LessThan,
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
        TokenKind::GreaterThan,
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
        TokenKind::EndOfFile,
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
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::Integer(0),
        TokenKind::Semicolon,
        TokenKind::While,
        TokenKind::Identifier("a".into()),
        TokenKind::LessThan,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("a".into()),
        TokenKind::AddAssign,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_for() {
    let input = r"
        for i in 0 to 10 {
            println(i);
        }
        ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::For,
        TokenKind::Identifier("i".into()),
        TokenKind::Range,
        TokenKind::Integer(0),
        TokenKind::To,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::Identifier("println".into()),
        TokenKind::LeftParenthesis,
        TokenKind::Identifier("i".into()),
        TokenKind::RightParenthesis,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
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
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::Integer(0),
        TokenKind::Semicolon,
        TokenKind::While,
        TokenKind::Identifier("a".into()),
        TokenKind::LessThan,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::Equality,
        TokenKind::Integer(5),
        TokenKind::LeftCurlyBrace,
        TokenKind::Break,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Identifier("a".into()),
        TokenKind::AddAssign,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
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
    let tokens = lexer.tokenize().unwrap();

    let actual_token_kinds: Vec<TokenKind> =
        tokens.iter().map(|token| token.kind.clone()).collect();
    let expected_token_kinds = [
        TokenKind::Variable,
        TokenKind::Identifier("a".into()),
        TokenKind::Assign,
        TokenKind::Integer(0),
        TokenKind::Semicolon,
        TokenKind::While,
        TokenKind::Identifier("a".into()),
        TokenKind::LessThan,
        TokenKind::Integer(10),
        TokenKind::LeftCurlyBrace,
        TokenKind::If,
        TokenKind::Identifier("a".into()),
        TokenKind::Equality,
        TokenKind::Integer(5),
        TokenKind::LeftCurlyBrace,
        TokenKind::Continue,
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::Identifier("a".into()),
        TokenKind::AddAssign,
        TokenKind::Integer(1),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
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
    let tokens = lexer.tokenize().unwrap();

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
        TokenKind::Arrow,
        TokenKind::Identifier("int".into()),
        TokenKind::LeftCurlyBrace,
        TokenKind::Return,
        TokenKind::Identifier("a".into()),
        TokenKind::Plus,
        TokenKind::Identifier("b".into()),
        TokenKind::Semicolon,
        TokenKind::RightCurlyBrace,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_comments() {
    let input = r"
        // This is a single-line comment.
        let a = 10; // This is another single-line comment.

        /*
         This is a multi-line comment,
         it can span multiple lines!
        */

        let b = 20; /* This is another multi-line comment, it doesnt span multiple lines. */
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
        TokenKind::Variable,
        TokenKind::Identifier("b".into()),
        TokenKind::Assign,
        TokenKind::Integer(20),
        TokenKind::Semicolon,
        TokenKind::EndOfFile,
    ];

    assert_eq!(actual_token_kinds, expected_token_kinds);
}
