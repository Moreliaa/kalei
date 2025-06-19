use crate::{ast::*, lexer::*};
use std::str::Chars;

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    cur_token: Option<Token>,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Parser<'a> {
        Parser {
            lexer,
            cur_token: None,
        }
    }

    fn parse_number_expr(&mut self) -> NumberExprAst {
        let result = NumberExprAst::new(self.lexer.num_val);
        self.read_token();
        result
    }

    fn parse_binary_expr(&mut self) -> BinaryExprAst {
        todo!()
    }

    fn parse_primary(&mut self) -> Box<dyn Expr> {
        if let Some(tok) = &self.cur_token {
            match tok {
                Token::Number => Box::new(self.parse_number_expr()),
                // Token::Character => {
                //     if let Some(c) = &self.lexer.last_char {
                //         match c {
                //             '+' | '-' | '*' | '/' => Box::new(self.parse_binary_expr()),
                //             _ => todo!(),
                //         }
                //     } else {
                //         panic!("Expected character");
                //     }
                // }
                _ => todo!(),
            }
        } else {
            panic!("Expected token");
        }
    }

    fn parse_binary_op_rhs(&mut self, expr_precedence: u8, lhs: Box<dyn Expr>) -> Box<dyn Expr> {
        let mut lhs = lhs;
        loop {
            let tok_precedence: u8 = 0; // TODO get from table for current token
            if tok_precedence < expr_precedence {
                return lhs;
            }

            // found a bin op
            let bin_op_char = self.lexer.last_char.unwrap();
            self.read_token();

            let mut rhs = self.parse_primary();

            let next_precedence: u8 = 0; // TODO
            if tok_precedence < next_precedence {
                // TODO what happens for large expressions with tok_precedence + 1?
                rhs = self.parse_binary_op_rhs(tok_precedence + 1, rhs);
            }
            lhs = Box::new(BinaryExprAst::new(bin_op_char, lhs, rhs));
        }
    }

    fn parse_expr(&mut self) -> Box<dyn Expr> {
        let lhs: Box<dyn Expr> = self.parse_primary();
        return self.parse_binary_op_rhs(0, lhs);
    }

    fn parse_top_level_expr(&mut self) -> Box<dyn Expr> {
        // TODO should define an anonymous function
        self.parse_expr()
    }

    fn log_verbose(&self, msg: String) {
        println!("{}", msg);
    }

    fn read_token(&mut self) {
        self.cur_token = Some(self.lexer.get_token());
        self.log_verbose(format!("Read a token {:?}", self.cur_token));
    }

    pub fn main_loop(&mut self) {
        self.read_token();
        loop {
            if let Some(tok) = &self.cur_token {
                let result = match tok {
                    Token::Eof => break,
                    Token::Number | Token::Character => self.parse_top_level_expr(),
                    _ => todo!(),
                };
            }
        }
    }
}
