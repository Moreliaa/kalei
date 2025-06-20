use crate::{ast::*, lexer::*, treeprinter::*};

const USE_VERBOSE_LOGS: bool = false;

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

    // numberexpr ::= number
    fn parse_number_expr(&mut self) -> NumberExprAst {
        let result = NumberExprAst::new(self.lexer.num_val);
        self.read_token();
        result
    }

    // parenthesisexpr ::= '(' expr ')'
    fn parse_parenthesis_expr(&mut self) -> Box<dyn Expr> {
        self.read_token(); // eat (
        let result = self.parse_expr();
        if self.lexer.identifier_str != ")" {
            panic!("Expected ')'");
        }
        self.read_token(); // eat )
        result
    }

    // variable references and function calls
    // identifierexpr ::= identifier
    // identifierexpr ::= identifier '(' expr* ')'
    fn parse_identifier_expr(&mut self) -> Box<dyn Expr> {
        let identifier = self.lexer.identifier_str.clone();
        self.read_token(); // eat identifier
        if self.lexer.identifier_str != "(" {
            return Box::new(VariableExprAst::new(identifier));
        }

        // function call
        self.read_token(); // eat (

        let mut args: Vec<Box<dyn Expr>> = vec![];
        if self.lexer.identifier_str != ")" {
            loop {
                let arg = self.parse_expr();
                args.push(arg);

                if self.lexer.identifier_str == ")" {
                    break;
                }

                if self.lexer.identifier_str != "," {
                    panic!("Expected ','");
                }

                self.read_token();
            }
        }

        self.read_token(); // eat )
        Box::new(FunctionCallExprAst::new(identifier, args))
    }

    // primary ::= numberexpr
    // primary ::= identifierexpr
    // primary ::= parenthesisexpr
    fn parse_primary(&mut self) -> Box<dyn Expr> {
        if let Some(tok) = &self.cur_token {
            match tok {
                Token::Number => Box::new(self.parse_number_expr()),
                Token::Identifier => self.parse_identifier_expr(),
                Token::Character => {
                    if self.lexer.identifier_str == "(" {
                        self.parse_parenthesis_expr()
                    } else {
                        panic!("Unknown token '{}'", self.lexer.identifier_str);
                    }
                }
                _ => panic!("Unexpected token {:?}", tok),
            }
        } else {
            panic!("Expected token");
        }
    }

    // binoprhs ::= (('+'|'-'|'*'|'/') primary)*
    fn parse_binary_op_rhs(&mut self, expr_precedence: i8, lhs: Box<dyn Expr>) -> Box<dyn Expr> {
        // TODO left-right associativity
        let mut lhs = lhs;
        loop {
            let tok_precedence: i8 = self.get_op_precedence();
            if tok_precedence <= expr_precedence {
                return lhs;
            }

            // found bin op
            let bin_op_char = self.lexer.identifier_str.clone();
            self.read_token(); // eat operator

            let mut rhs = self.parse_primary();

            let next_precedence: i8 = self.get_op_precedence();
            if tok_precedence < next_precedence {
                rhs = self.parse_binary_op_rhs(tok_precedence, rhs);
            }
            lhs = Box::new(BinaryExprAst::new(bin_op_char, lhs, rhs));
        }
    }

    // expr ::= primary binoprhs
    fn parse_expr(&mut self) -> Box<dyn Expr> {
        let lhs: Box<dyn Expr> = self.parse_primary();
        self.parse_binary_op_rhs(0, lhs)
    }

    fn parse_top_level_expr(&mut self) -> Box<dyn Expr> {
        // TODO should define an anonymous function
        self.parse_expr()
    }

    fn log_verbose(&self, msg: String) {
        if USE_VERBOSE_LOGS {
            println!("{}", msg);
        }
    }

    fn read_token(&mut self) {
        self.cur_token = Some(self.lexer.get_token());
        self.log_verbose(format!(
            "Read a token {:?} {:?}",
            if self.lexer.identifier_str != "" {
                self.lexer.identifier_str.to_string()
            } else {
                self.lexer.num_val.to_string()
            },
            self.cur_token,
        ));
    }

    fn get_op_precedence(&self) -> i8 {
        if self.cur_token != Some(Token::Character) {
            return -1;
        }
        match self.lexer.identifier_str.as_str() {
            "+" => 10,
            "-" => 10,
            "*" => 20,
            "/" => 20,
            _ => -1,
        }
    }

    pub fn main_loop(&mut self) {
        self.read_token();
        loop {
            if let Some(tok) = &self.cur_token {
                let expr: Box<dyn Expr> = match tok {
                    Token::Eof => break,
                    Token::Number | Token::Character => self.parse_top_level_expr(),
                    _ => todo!(),
                };

                let mut treeprinter = TreePrinter::new();
                expr.print(&mut treeprinter, 0, 0);
                treeprinter.print_tree();
            }
        }
    }
}
