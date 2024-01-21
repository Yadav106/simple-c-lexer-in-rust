#![allow(unused)]
use crate::token::Token;

pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer { input, position: 0 }
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

        if current_char.is_digit(10) {
            return Token::Number(self.read_number());
        }

        match current_char {
            '#' => {
                return Token::Preprocessor(self.read_preprocessor());
            }
            _ => {
                println!("Invalid char {}", current_char);
                return Token::EOF;
            }
        }

        return Token::EOF;
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
            self.position += 1;
        }
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
                println!("Invalid preprocessor {}", preprocessor_str);
                crate::token::Preprocessor::Invalid
            }
        }
    }
}
