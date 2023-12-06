use lang::lexer::Lexer;
use lang::parser::errors::Error;
use lang::parser::Parser;

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_semicolon() {
    let input = r"
        let x = 1
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    assert_eq!(
        ast,
        Err(Error::UnexpectedToken {
            line: 1,
            column: 9,
            message: "Expected ';' after variable declaration.".into(),
        })
    );
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_expression() {
    let input = r"
        let x = ;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    assert_eq!(
        ast,
        Err(Error::UnexpectedToken {
            line: 1,
            column: 8,
            message: "Expected expression.".into(),
        })
    );
}

#[test]
#[allow(clippy::unwrap_used)]
fn test_no_name() {
    let input = r"
        let = 1;
    ";

    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();

    let mut parser = Parser::new(&tokens);
    let ast = parser.parse();

    assert_eq!(
        ast,
        Err(Error::UnexpectedToken {
            line: 1,
            column: 4,
            message: "Expected variable name.".into(),
        })
    );
}
