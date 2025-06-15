use std::io::prelude::*;
use std::io::stdin;
use std::str::Chars;

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

// Lexer
#[derive(Debug)]
enum Token {
    Eof,
    Def,
    Extern,
    Identifier,
    Number,
    Character,
}

const IDENT_DEF: &str = "def";
const IDENT_EXTERN: &str = "extern";
const IDENT_CHAR_COMMENT: char = '#';

// note: we want this to read the next character from stdin!
struct Lexer {
    identifier_str: String,
    num_val: f64,
    last_char: Option<char>,
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            identifier_str: String::new(),
            num_val: 0.0,
            last_char: Some(' '),
        }
    }
    /**
     * Return the next token from standard input.
     */
    fn get_token(&mut self, chars: &mut Chars) -> Token {
        self.identifier_str = String::new();
        self.num_val = 0.0;

        while let Some(c) = self.last_char {
            if !c.is_whitespace() {
                break;
            }
            self.last_char = chars.next();
        }

        if self.last_char == None {
            return Token::Eof;
        }

        if self.last_char.unwrap().is_alphabetic() {
            loop {
                self.identifier_str.push(self.last_char.unwrap());
                self.last_char = chars.next();
                if self.last_char == None || !self.last_char.unwrap().is_alphanumeric() {
                    break;
                }
            }
            if self.identifier_str == IDENT_DEF {
                return Token::Def;
            }
            if self.identifier_str == IDENT_EXTERN {
                return Token::Extern;
            }
            return Token::Identifier;
        } else if self.last_char.unwrap().is_numeric() {
            let mut num_str = String::new();
            loop {
                num_str.push(self.last_char.unwrap());
                self.last_char = chars.next();
                if self.last_char == None || !self.last_char.unwrap().is_numeric() {
                    if self.last_char.unwrap() != '.' {
                        break;
                    }
                }
            }
            self.num_val = match num_str.parse::<f64>() {
                Ok(val) => val,
                Err(error) => panic!("Failed to parse number token {num_str:?}: {error:?}"),
            };
            return Token::Number;
        } else {
            if self.last_char.unwrap() == IDENT_CHAR_COMMENT {
                // Comment until end of line
                loop {
                    self.last_char = chars.next();
                    if self.last_char == None
                        || self.last_char.unwrap() == '\n'
                        || self.last_char.unwrap() == '\r'
                    {
                        break;
                    }
                }
                return self.get_token(chars);
            } else {
                self.identifier_str.push(self.last_char.unwrap());
                self.last_char = chars.next();
                return Token::Character;
            }
        }
    }
}
