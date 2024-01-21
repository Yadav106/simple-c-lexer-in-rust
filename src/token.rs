#![allow(unused)]
#[derive(Debug)]
pub enum Keyword {
    Auto,
    Break,
    Case,
    Char,
    Const,
    Continue,
    Default,
    Do,
    Double,
    Else,
    Enum,
    Extern,
    Float,
    For,
    Goto,
    If,
    Int,
    Long,
    Register,
    Return,
    Short,
    Signed,
    Sizeof,
    Static,
    Struct,
    Switch,
    Typedef,
    Union,
    Unsigned,
    Void,
    Volatile,
    While,
}

#[derive(Debug)]
pub enum Punctuation {
    Comma,
    Dot,
    Semicolon,
    RParanthesis,
    LParanthesis,
    RBraces,
    LBraces,
    RSqBracket,
    LSqBracket,
}

#[derive(Debug)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
    Lesser,
    Greater,
}

#[derive(Debug)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(String),
    Operator(Operator),
    Punctuation(Punctuation),
    Character(char),
    Number(i64),
}
