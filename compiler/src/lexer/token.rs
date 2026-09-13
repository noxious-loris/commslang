#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Keywords
    System,
    Signal,
    Channel,
    Receiver,
    Transmitter,
    Connect,
    Simulate,
    Library,
    Function,
    If,
    Else,
    For,
    While,
    Return,
    Const,
    Let,
    True,
    False,

    // Literals
    Identifier(String),
    Integer(i64),
    Float(f64),
    String(String),

    // Units
    Hz,
    KHz,
    MHz,
    GHz,
    W,
    DB,
    DBm,

    // Punctuation
    LBrace,
    RBrace,
    LParen,
    RParen,
    Colon,
    Semicolon,
    Comma,

    // Operators
    Assign,
    Arrow,

    Plus,
    Minus,
    Multiply,
    Divide,

    Equal,
    NotEqual,

    Less,
    Greater,
    LessEqual,
    GreaterEqual,

    // End of file
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub line: usize,
    pub column: usize,
}
