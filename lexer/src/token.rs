/// A token kind, which is a type of token.
///
/// # Variants
///
/// * `Identifier(String)` - An identifier, such as `foo` or `bar`.
/// * `Integer(i64)` - An integer, such as `123` or `456`.
/// * `Float(f64)` - A float, such as `1.23` or `4.56`.
/// * `String(String)` - A string, such as `"foo"` or `"bar"`.
/// * `Plus` - A plus sign, `+`.
/// * `PlusEqual` - A plus-equal sign, `+=`.
/// * `Minus` - A minus sign, `-`.
/// * `MinusEqual` - A minus-equal sign, `-=`.
/// * `Asterisk` - An asterisk, `*`.
/// * `AsteriskEqual` - An asterisk-equal sign, `*=`.
/// * `Slash` - A slash, `/`.
/// * `SlashEqual` - A slash-equal sign, `/=`.
/// * `Percent` - A percent sign, `%`.
/// * `PercentEqual` - A percent-equal sign, `%=`.
/// * `Caret` - A caret, `^`.
/// * `CaretEqual` - A caret-equal sign, `^=`.
/// * `Bang` - A bang, `!`.
/// * `BangEqual` - A bang-equal sign, `!=`.
/// * `Equal` - An equal sign, `=`.
/// * `EqualEqual` - An equal-equal sign, `==`.
/// * `Less` - A less-than sign, `<`.
/// * `LessEqual` - A less-than-equal sign, `<=`.
/// * `Greater` - A greater-than sign, `>`.
/// * `GreaterEqual` - A greater-than-equal sign, `>=`.
/// * `And` - An and sign, `&`.
/// * `Or` - An or sign, `|`.
/// * `LeftParenthesis` - A left parenthesis, `(`.
/// * `RightParenthesis` - A right parenthesis, `)`.
/// * `LeftCurlyBrace` - A left curly brace, `{`.
/// * `RightCurlyBrace` - A right curly brace, `}`.
/// * `LeftBracket` - A left bracket, `[`.
/// * `RightBracket` - A right bracket, `]`.
/// * `Comma` - A comma, `,`.
/// * `Dot` - A dot, `.`.
/// * `Colon` - A colon, `:`.
/// * `Semicolon` - A semicolon, `;`.
/// * `Arrow` - An arrow, `->`.
/// * `True` - A true keyword, `true`.
/// * `False` - A false keyword, `false`.
/// * `If` - An if keyword, `if`.
/// * `Else` - An else keyword, `else`.
/// * `While` - A while keyword, `while`.
/// * `For` - A for keyword, `for`.
/// * `In` - An in keyword, `in`.
/// * `Break` - A break keyword, `break`.
/// * `Continue` - A continue keyword, `continue`.
/// * `Return` - A return keyword, `return`.
/// * `Function` - A function keyword, `function`.
/// * `Variable` - A variable keyword, `variable`.
/// * `EOF` - An end-of-file token.
#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),
    Plus,
    PlusEqual,
    Minus,
    MinusEqual,
    Asterisk,
    AsteriskEqual,
    Slash,
    SlashEqual,
    Percent,
    PercentEqual,
    Caret,
    CaretEqual,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
    LeftParenthesis,
    RightParenthesis,
    LeftCurlyBrace,
    RightCurlyBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    True,
    False,
    If,
    Else,
    While,
    For,
    In,
    Break,
    Continue,
    Return,
    Function,
    Variable,
    EOF,
}

/// A token, which is a single unit of a program.
///
/// # Fields
///
/// * `kind` - The kind of token.
/// * `line` - The line number of the token.
/// * `column` - The column number of the token.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Creates a new token.
    ///
    /// # Arguments
    ///
    /// * `kind` - The kind of token.
    /// * `line` - The line number of the token.
    /// * `column` - The column number of the token.
    ///
    /// # Returns
    ///
    /// * `Token` - The new token.
    #[must_use]
    pub const fn new(kind: TokenKind, line: usize, column: usize) -> Self {
        Self { kind, line, column }
    }
}
