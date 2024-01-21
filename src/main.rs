use std::fs;

mod lexer;
mod token;

fn main() {
    let filepath = "./src/source.c";
    let contents = fs::read_to_string(filepath).expect("Failed to read file");

    println!("The file contains {}", contents);
}
