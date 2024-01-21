use std::fs;

mod lexer;
use lexer::Lexer;

mod token;
use crate::token::Token;

fn main() {
    let filepath = "./src/source.c";
    let contents = fs::read_to_string(filepath).expect("Failed to read file");
    let mut c_lexer = Lexer::new(&contents);
    c_lexer.print_input();
    c_lexer.print_input_length();

    loop {
        let tk = c_lexer.next_token();
        println!("{:?}", tk);

        if tk == Token::EOF {
            break;
        }
    }
}
