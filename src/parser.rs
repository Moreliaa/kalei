use crate::{ast::*, codegen::*, lexer::*, treeprinter::*};

const USE_VERBOSE_LOGS: bool = true;

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
        self.log_verbose(String::from("Parsed number expression"));
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
        self.log_verbose(String::from("Parsed parenthesis expression"));
        result
    }

    // variable references and function calls
    // identifierexpr ::= identifier
    // identifierexpr ::= identifier '(' expr* ')'
    fn parse_identifier_expr(&mut self) -> Box<dyn Expr> {
        let identifier = self.lexer.identifier_str.clone();
        self.read_token(); // eat identifier
        if self.lexer.identifier_str != "(" {
            self.log_verbose(String::from("Parsed identifier"));
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
        self.log_verbose(String::from("Parsed function call"));
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
                self.log_verbose(String::from("Parsed binary expression"));
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
        let lhs = self.parse_primary();
        self.parse_binary_op_rhs(0, lhs)
    }

    // prototype ::= identifier '(' identifier* ')'
    fn parse_prototype(&mut self) -> PrototypeAst {
        if self.cur_token != Some(Token::Identifier) {
            panic!("Expected identifier in prototype");
        }

        let name = self.lexer.identifier_str.clone();
        self.read_token(); // eat identifier
        if self.lexer.identifier_str != "(" {
            panic!("Expected '(' in prototype");
        }

        self.read_token(); // eat (

        let mut args: Vec<String> = vec![];
        while self.cur_token == Some(Token::Identifier) {
            args.push(self.lexer.identifier_str.clone());
            self.read_token(); // eat identifier
            if self.lexer.identifier_str == "," {
                self.read_token(); // eat ,
            }
        }

        if self.lexer.identifier_str != ")" {
            panic!(
                "Expected ')' in prototype, found {}",
                self.lexer.identifier_str
            );
        }

        self.read_token(); // eat )
        PrototypeAst::new(name, args)
    }

    // definition ::= 'def' prototype expr
    fn parse_def(&mut self) -> FunctionAst {
        self.read_token(); // eat def
        let proto = self.parse_prototype();
        let body = self.parse_expr();
        self.log_verbose(format!("Parsed function definition {}", proto.name));
        FunctionAst::new(proto, body)
    }

    // external ::= 'extern' prototype
    fn parse_extern(&mut self) -> PrototypeAst {
        self.read_token(); // eat extern
        self.log_verbose(String::from("Parsed external function definition"));
        self.parse_prototype()
    }

    // toplevelexpr ::= expr
    fn parse_top_level_expr(&mut self) -> FunctionAst {
        let expr = self.parse_expr();
        let proto = PrototypeAst::new(String::new(), vec![]);
        self.log_verbose(String::from("Parsed top-level expression"));
        FunctionAst::new(proto, expr)
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
            if !self.lexer.identifier_str.is_empty() {
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
                let function: Box<dyn Function> = match tok {
                    Token::Eof => break,
                    Token::Def => Box::new(self.parse_def()),
                    Token::Extern => Box::new(self.parse_extern()),
                    Token::Character => match self.lexer.identifier_str.as_str() {
                        ";" => {
                            self.read_token(); // eat ;
                            continue;
                        }
                        _ => Box::new(self.parse_top_level_expr()),
                    },
                    _ => Box::new(self.parse_top_level_expr()),
                };

                generate_code(function);

                // TODO treeprinter
                // let mut treeprinter = TreePrinter::new();
                // function.body.expr.print(&mut treeprinter, 0, 0);
                // treeprinter.print_tree();
            } else {
                panic!("Expected token");
            }
        }
    }
}
