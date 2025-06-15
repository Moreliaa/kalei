use std::io::prelude::*;
use std::io::stdin;
use std::str::Chars;

fn main() {
    println!("Hello, world!");
    let mut stdin = stdin();
    let mut buffer = String::new();
    stdin.read_to_string(&mut buffer).unwrap();
    let mut chars = buffer.chars();
    let mut lexer = Lexer::new();
    loop {
        let tok = lexer.get_token(&mut chars);
        println!("{tok:?}");
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
}

impl Lexer {
    fn new() -> Lexer {
        Lexer {
            identifier_str: String::new(),
            num_val: 0.0,
        }
    }
    /**
     * Return the next token from standard input.
     */
    fn get_token(&mut self, chars: &mut Chars) -> Token {
        self.identifier_str = String::new();
        self.num_val = 0.0;
        let mut last_char: Option<char> = Some(' ');

        while last_char == Some(' ') {
            last_char = chars.next();
        }

        if last_char == None {
            return Token::Eof;
        }

        if last_char.unwrap().is_alphabetic() {
            loop {
                self.identifier_str.push(last_char.unwrap());
                last_char = chars.next();
                if last_char == None || !last_char.unwrap().is_alphanumeric() {
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
        } else if last_char.unwrap().is_numeric() {
            let mut num_str = String::new();
            let mut seen_dot = false;
            loop {
                num_str.push(last_char.unwrap());
                last_char = chars.next();
                if last_char == None || !last_char.unwrap().is_numeric() {
                    if last_char.unwrap() == '.' {
                        if seen_dot {
                            break;
                        } else {
                            seen_dot = true;
                        }
                    } else {
                        break;
                    }
                }
            }
            self.num_val = match num_str.parse::<f64>() {
                Ok(val) => val,
                Err(error) => panic!("Failed to parse number token '{num_str:?}': {error:?}"),
            };
            return Token::Number;
        } else {
            if last_char.unwrap() == IDENT_CHAR_COMMENT {
                // Comment until end of line
                loop {
                    last_char = chars.next();
                    if last_char == None || last_char.unwrap() == '\n' || last_char.unwrap() == '\r'
                    {
                        break;
                    }
                }
                return self.get_token(chars);
            } else {
                return Token::Character;
            }
        }
    }
}
