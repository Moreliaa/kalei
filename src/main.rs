mod ast;
mod codegen;
mod lexer;
mod logger;
mod parser;
use crate::lexer::Lexer;
use crate::parser::Parser;

fn main() {
    let mut lexer = Lexer::new();
    let mut parser = Parser::new(&mut lexer);
    parser.main_loop();
}
