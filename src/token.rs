#![allow(unused)]
#[derive(Debug, PartialEq)]
pub enum Preprocessor {
    Define,
    Include,
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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
    LAngleBracket,
    RAngleBracket,
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    Divide,
    Multiply,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    String(String),
    Char(char),
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Identifier(String),
    Literal(Literal),
    Operator(Operator),
    Punctuation(Punctuation),
    Character(char),
    Number(i64),
    Preprocessor(Preprocessor),
    EOF,
}
