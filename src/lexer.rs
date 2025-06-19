use std::str::Chars;

#[derive(Debug, PartialEq, Clone)]
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

pub struct Lexer<'a> {
    pub identifier_str: String,
    pub num_val: f64,
    pub last_char: Option<char>,
    pub chars: Chars<'a>,
}

impl<'a> Lexer<'a> {
    pub fn new(chars: Chars<'a>) -> Lexer<'a> {
        Lexer {
            identifier_str: String::new(),
            num_val: 0.0,
            last_char: Some(' '),
            chars,
        }
    }

    pub fn get_token(&mut self) -> Token {
        self.identifier_str = String::new();
        self.num_val = 0.0;

        while let Some(c) = self.last_char {
            if !c.is_whitespace() {
                break;
            }
            self.last_char = self.chars.next();
        }

        if self.last_char.is_none() {
            return Token::Eof;
        }

        if self.last_char.unwrap().is_ascii_alphabetic() {
            self.get_identifier()
        } else if self.last_char.unwrap().is_numeric() {
            self.get_number()
        } else if self.last_char.unwrap() == IDENT_CHAR_COMMENT {
            loop {
                self.last_char = self.chars.next();
                if self.last_char.is_none()
                    || self.last_char.unwrap() == '\n'
                    || self.last_char.unwrap() == '\r'
                {
                    break;
                }
            }
            self.get_token()
        } else {
            self.identifier_str.push(self.last_char.unwrap());
            self.last_char = self.chars.next();
            Token::Character
        }
    }

    fn get_identifier(&mut self) -> Token {
        loop {
            self.identifier_str.push(self.last_char.unwrap());
            self.last_char = self.chars.next();
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

    fn get_number(&mut self) -> Token {
        let mut num_str = String::new();
        loop {
            num_str.push(self.last_char.unwrap());
            self.last_char = self.chars.next();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        let input = String::from("a ä+b 0.3 0.33#abc\ndef");
        let chars = input.chars();
        let mut lexer = Lexer::new(chars);
        assert_eq!(lexer.get_token(), Token::Identifier);
        assert_eq!(lexer.identifier_str, "a");

        assert_eq!(lexer.get_token(), Token::Character);
        assert_eq!(lexer.identifier_str, "ä");

        assert_eq!(lexer.get_token(), Token::Character);
        assert_eq!(lexer.identifier_str, "+");

        assert_eq!(lexer.get_token(), Token::Identifier);
        assert_eq!(lexer.identifier_str, "b");

        assert_eq!(lexer.get_token(), Token::Number);
        assert_eq!(lexer.num_val, 0.3);

        assert_eq!(lexer.get_token(), Token::Number);
        assert_eq!(lexer.num_val, 0.33);

        assert_eq!(lexer.get_token(), Token::Def);
        assert_eq!(lexer.identifier_str, "def");

        assert_eq!(lexer.get_token(), Token::Eof);
    }
}
