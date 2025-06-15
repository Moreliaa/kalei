mod lexer;
use crate::lexer::Lexer;
use crate::lexer::Token;
use std::io::prelude::*;
use std::io::stdin;

fn main() {
    let mut stdin = stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    let mut chars = buffer.chars();
    let mut lexer = Lexer::new();
    loop {
        let tok = lexer.get_token(&mut chars);
        let id = &lexer.identifier_str;
        let num = lexer.num_val;
        println!("{id:?} {num:?} {tok:?}");
        match tok {
            Token::Eof => break,
            _ => {}
        }
    }
}
