mod ast;
mod lexer;
mod parser;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::parser::Parser;
use std::io::prelude::*;

use std::io::stdin;

fn main() {
    // Note: run with echo <input string> | cargo run
    let mut stdin = stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    let chars = buffer.chars();

    let mut lexer = Lexer::new(chars);
    let mut parser = Parser::new(&mut lexer);
    loop {
        parser.main_loop();
        // let tok = lexer.get_token();
        // let id = &lexer.identifier_str;
        // let num = lexer.num_val;
        // println!("{id:?} {num:?} {tok:?}");
        // if tok == Token::Eof {
        //     break;
        // }
    }
}
