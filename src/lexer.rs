#![allow(unused)]

use crate::token::{Keyword, Literal, Token};

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    line_number: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            input,
            position: 0,
            line_number: 1,
        }
    }

    pub fn print_input(&self) {
        println!("{}", self.input);
    }

    pub fn print_input_length(&self) {
        println!("The size of input is {}", self.input.len());
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Token::EOF;
        }

        let current_char = self.input.chars().nth(self.position).unwrap();

        match current_char {
            '#' => {
                return Token::Preprocessor(self.read_preprocessor());
            }
            '\'' | '"' => {
                return Token::Literal(self.read_literal());
            }
            c if c.is_digit(10) => {
                return Token::Number(self.read_number());
            }
            c if c.is_ascii_punctuation() => {
                return self.read_punctuation();
            }
            c if c.is_alphabetic() => {
                return self.read_alphabet();
            }
            _ => {
                panic!("Invalid char {} on line {}", current_char, self.line_number);
            }
        }
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_whitespace()
        {
            if let '\n' = self.input.chars().nth(self.position).unwrap() {
                self.line_number += 1;
            }
            self.position += 1;
        }
    }

    fn read_alphabet(&mut self) -> Token {
        let start = self.position;

        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_alphabetic()
        {
            self.position += 1;
        }

        let current_word = &self.input[start..self.position];

        match current_word {
            "auto" => return Token::Keyword(Keyword::Auto),
            "break" => return Token::Keyword(Keyword::Break),
            "case" => return Token::Keyword(Keyword::Case),
            "char" => return Token::Keyword(Keyword::Char),
            "const" => return Token::Keyword(Keyword::Const),
            "continue" => return Token::Keyword(Keyword::Continue),
            "default" => return Token::Keyword(Keyword::Default),
            "do" => return Token::Keyword(Keyword::Do),
            "double" => return Token::Keyword(Keyword::Double),
            "else" => return Token::Keyword(Keyword::Else),
            "enum" => return Token::Keyword(Keyword::Enum),
            "extern" => return Token::Keyword(Keyword::Extern),
            "float" => return Token::Keyword(Keyword::Float),
            "for" => return Token::Keyword(Keyword::For),
            "goto" => return Token::Keyword(Keyword::Goto),
            "if" => return Token::Keyword(Keyword::If),
            "int" => return Token::Keyword(Keyword::Int),
            "long" => return Token::Keyword(Keyword::Long),
            "register" => return Token::Keyword(Keyword::Register),
            "return" => return Token::Keyword(Keyword::Return),
            "short" => return Token::Keyword(Keyword::Short),
            "signed" => return Token::Keyword(Keyword::Signed),
            "sizeof" => return Token::Keyword(Keyword::Sizeof),
            "static" => return Token::Keyword(Keyword::Sizeof),
            "struct" => return Token::Keyword(Keyword::Struct),
            "switch" => return Token::Keyword(Keyword::Switch),
            "typedef" => return Token::Keyword(Keyword::Typedef),
            "union" => return Token::Keyword(Keyword::Union),
            "unsigned" => return Token::Keyword(Keyword::Unsigned),
            "void" => return Token::Keyword(Keyword::Void),
            "volatile" => return Token::Keyword(Keyword::Volatile),
            "while" => return Token::Keyword(Keyword::While),
            _ => {}
        }

        return Token::Identifier(self.input[start..self.position].to_string());
    }

    fn read_punctuation(&mut self) -> Token {
        let current_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1;
        match current_char {
            '/' => {
                // logic for comment
                if self.position < self.input.len() - 1 {
                    if self.input.chars().nth(self.position).unwrap() == '/' {
                        while self.position < self.input.len()
                            && self.input.chars().nth(self.position).unwrap() != '\n'
                        {
                            self.position += 1;
                        }
                        return Token::Comment;
                    }
                }
                return Token::Operator(crate::token::Operator::Divide);
            }
            '+' => return Token::Operator(crate::token::Operator::Plus),
            '-' => return Token::Operator(crate::token::Operator::Minus),
            '*' => return Token::Operator(crate::token::Operator::Multiply),
            ',' => return Token::Punctuation(crate::token::Punctuation::Comma),
            '.' => return Token::Punctuation(crate::token::Punctuation::Dot),
            ';' => return Token::Punctuation(crate::token::Punctuation::Semicolon),
            '(' => return Token::Punctuation(crate::token::Punctuation::LParanthesis),
            ')' => return Token::Punctuation(crate::token::Punctuation::RParanthesis),
            '{' => return Token::Punctuation(crate::token::Punctuation::LBraces),
            '}' => return Token::Punctuation(crate::token::Punctuation::RBraces),
            '[' => return Token::Punctuation(crate::token::Punctuation::LSqBracket),
            ']' => return Token::Punctuation(crate::token::Punctuation::RSqBracket),
            '<' => return Token::Punctuation(crate::token::Punctuation::LAngleBracket),
            '>' => return Token::Punctuation(crate::token::Punctuation::RAngleBracket),
            _ => panic!("Invalid character {}", current_char),
        }
    }

    fn read_literal(&mut self) -> crate::token::Literal {
        let start = self.position;
        let initial_char = self.input.chars().nth(self.position).unwrap();

        self.position += 1;

        while self.position < self.input.len() {
            match self.input.chars().nth(self.position).unwrap() {
                '\n' => {
                    panic!(
                        "line {} : Requires ` {} ` in  ` {} `",
                        self.line_number,
                        initial_char,
                        &self.input[start..self.position],
                    );
                }
                current_character if current_character == initial_char => {
                    self.position += 1;
                    match initial_char {
                        '"' => {
                            // string literal
                            return Literal::String(
                                self.input[start + 1..self.position - 1].to_string(),
                            );
                        }
                        '\'' => {
                            // character literal
                            let valid_char_len = 3;
                            let char_len = self.position - start;

                            if char_len < valid_char_len {
                                panic!("line {} : char cannot be empty", self.line_number);
                            }
                            if char_len != valid_char_len {
                                panic!("line {}, More than one character in data type char, use \" for including multiple characters : {}", self.line_number, self.input[start+1..self.position-1].to_string());
                            }
                            return Literal::Char(
                                self.input.chars().nth(self.position - 2).unwrap(),
                            );
                        }
                        _ => {
                            panic!(
                                "line {} : Invalid syntax {}",
                                self.line_number,
                                self.input[start..self.position].to_string()
                            );
                        }
                    }
                }
                _ => {
                    self.position += 1;
                }
            }
        }

        return Literal::String(String::from(""));
    }

    fn read_number(&mut self) -> i64 {
        let start = self.position;
        while self.position < self.input.len()
            && self.input.chars().nth(self.position).unwrap().is_digit(10)
        {
            self.position += 1;
        }
        let num_str = &self.input[start..self.position];
        num_str.parse().expect("Failed to parse number")
    }

    fn read_preprocessor(&mut self) -> crate::token::Preprocessor {
        let start = self.position;
        self.position += 1; // to skip the # character
        while self.position < self.input.len()
            && self
                .input
                .chars()
                .nth(self.position)
                .unwrap()
                .is_alphabetic()
        {
            self.position += 1;
        }
        let preprocessor_str = &self.input[start..self.position];

        match preprocessor_str {
            "#include" => crate::token::Preprocessor::Include,
            "#define" => crate::token::Preprocessor::Define,
            _ => {
                panic!("Invalid preprocessor {}", preprocessor_str)
            }
        }
    }
}
