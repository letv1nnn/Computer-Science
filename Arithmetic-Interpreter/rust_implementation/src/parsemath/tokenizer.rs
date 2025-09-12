use std::str::Chars;
use std::iter::Peekable;

use crate::parsemath::token::Token;


pub struct Tokenizer<'a> {
    expr: Peekable<Chars<'a>>
}

impl<'a> Tokenizer<'a> {
    pub fn new(expr: &'a str) -> Self {
        Self { expr: expr.chars().peekable() }
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let next_char = self.expr.next();

        match next_char {
            Some('0'..='9') => {
                let mut number = next_char?.to_string();
                
                while let Some(next_char) = self.expr.peek() {
                    if next_char.is_numeric() || next_char == &'.' {
                        number.push(self.expr.next()?);
                    } else if next_char == &'(' {
                        return None;
                    } else {
                        break;
                    }
                }

                Some(Token::Num(number.parse::<f64>().unwrap()))
            },
            Some('+') => Some(Token::Add),
            Some('-') => Some(Token::Subtract),
            Some('/') => Some(Token::Divide),
            Some('*') => Some(Token::Multiply),
            Some('^') => Some(Token::Caret),
            Some('(') => Some(Token::LeftParen),
            Some(')') => Some(Token::RightParen),            
            None => Some(Token::EOF),
            Some(_) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_positive_integer() {
        let mut tokenizer = Tokenizer::new("34");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(34.0));
    }

    #[test]
    fn test_float_number() {
        let mut tokenizer = Tokenizer::new("123.456");
        assert_eq!(tokenizer.next().unwrap(), Token::Num(123.456));
    }
}
