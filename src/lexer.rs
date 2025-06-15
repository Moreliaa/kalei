use std::str::Chars;

#[derive(Debug, PartialEq)]
pub enum Token {
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

pub struct Lexer {
    pub identifier_str: String,
    pub num_val: f64,
    pub last_char: Option<char>,
}

impl Lexer {
    pub fn new() -> Lexer {
        Lexer {
            identifier_str: String::new(),
            num_val: 0.0,
            last_char: Some(' '),
        }
    }
    /**
     * Return the next token from standard input.
     */
    pub fn get_token(&mut self, chars: &mut Chars) -> Token {
        self.identifier_str = String::new();
        self.num_val = 0.0;

        while let Some(c) = self.last_char {
            if !c.is_whitespace() {
                break;
            }
            self.last_char = chars.next();
        }

        if self.last_char.is_none() {
            return Token::Eof;
        }

        if self.last_char.unwrap().is_ascii_alphabetic() {
            self.get_identifier(chars)
        } else if self.last_char.unwrap().is_numeric() {
            self.get_number(chars)
        } else if self.last_char.unwrap() == IDENT_CHAR_COMMENT {
            loop {
                self.last_char = chars.next();
                if self.last_char.is_none()
                    || self.last_char.unwrap() == '\n'
                    || self.last_char.unwrap() == '\r'
                {
                    break;
                }
            }
            self.get_token(chars)
        } else {
            self.identifier_str.push(self.last_char.unwrap());
            self.last_char = chars.next();
            Token::Character
        }
    }

    fn get_identifier(&mut self, chars: &mut Chars) -> Token {
        loop {
            self.identifier_str.push(self.last_char.unwrap());
            self.last_char = chars.next();
            if self.last_char.is_none() || !self.last_char.unwrap().is_ascii_alphanumeric() {
                break;
            }
        }
        match self.identifier_str.as_str() {
            IDENT_DEF => Token::Def,
            IDENT_EXTERN => Token::Extern,
            _ => Token::Identifier,
        }
    }

    fn get_number(&mut self, chars: &mut Chars) -> Token {
        let mut num_str = String::new();
        loop {
            num_str.push(self.last_char.unwrap());
            self.last_char = chars.next();
            if self.last_char.is_none()
                || (!self.last_char.unwrap().is_numeric() && self.last_char.unwrap() != '.')
            {
                break;
            }
        }
        self.num_val = match num_str.parse::<f64>() {
            Ok(val) => val,
            Err(error) => panic!("Failed to parse number token {num_str:?}: {error:?}"),
        };
        Token::Number
    }
}
