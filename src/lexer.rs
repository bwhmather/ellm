use std::str::CharRange;
use regex::Regex;

pub use self::Token::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Token<'a> {
    TypeName(&'a str),
    VarName(&'a str),
    Operator(&'a str),

    String(&'a str),
    Number(&'a str),
    Comma,

    OpeningParenthesis,
    ClosingParenthesis,
    Indentation(usize),

    EOF,
}


type LexerResult<'a> = Result<Token<'a>, &'static str>;


struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
}


impl<'a> Lexer<'a> {
    fn new(input : &'a str) -> Lexer<'a> {
        Lexer{
            input: input,
            cursor: 0,
        }
    }

    fn consume_whitespace(&mut self) {
        loop {
            if self.input.len() == self.cursor {
                return;    
            }

            if !self.input.is_char_boundary(self.cursor) {
                return;
            }

            let CharRange{ ch, next } = self.input.char_range_at(self.cursor);
            match ch {
                ' ' => { self.cursor = next; },
                _ => return,
            }
        }
    }

    fn scan_varname(&mut self) -> LexerResult<'a> {
        let token_start = self.cursor;

        loop {
            if self.input.len() == self.cursor {
                break;
            }

            if !self.input.is_char_boundary(self.cursor) {
                return Err("encountered invalid character");
            }

            let CharRange{ ch, next } = self.input.char_range_at(self.cursor); 

            match ch {
                'a'...'z' | 'A'...'Z' => {
                    self.cursor = next;
                }
                _ => break,
            }
        }
        Ok(VarName(&self.input[token_start..self.cursor]))
    }

    fn scan_typename(&mut self) -> LexerResult<'a> {
        let token_start = self.cursor;

        loop {
            if self.input.len() == self.cursor {
                break;
            }

            if !self.input.is_char_boundary(self.cursor) {
                return Err("encountered invalid character");
            }

            let CharRange{ ch, next } = self.input.char_range_at(self.cursor); 

            match ch {
                'a'...'z' | 'A'...'Z' => {
                    self.cursor = next;
                }
                _ => break,
            }
        }
        Ok(VarName(&self.input[token_start..self.cursor]))
    }

    fn scan_number(&mut self) -> LexerResult<'a> {
        let re = regex!(r"^-?[0-9]+");

        match re.find(&self.input[self.cursor..]) {
            Some((_, token_size)) => {
                let token_start = self.cursor;
                let token_end = token_start + token_size;
                self.cursor = token_end;
                Ok(Number(&self.input[token_start..token_end]))
            }
            None => {
                Err("invalid number")
            }
        }
    }

    pub fn next(&mut self) -> LexerResult<'a> {
        self.consume_whitespace();

        if self.input.len() == self.cursor {
            return Ok(EOF);
        }

        if !self.input.is_char_boundary(self.cursor) {
            return Err("encountered invalid character");
        }

        let CharRange{ ch, next } = self.input.char_range_at(self.cursor);

        match ch {
            'a'...'z' => {
                self.scan_varname()
            }
            'A'...'Z' => {
                self.scan_typename()
            }
            '0'...'9' => {
                self.scan_number()
            }
            '(' => {
                self.cursor = next;
                Ok(OpeningParenthesis)
            }
            ')' => {
                self.cursor = next;
                Ok(ClosingParenthesis)
            }
            '-' => {
                match self.input.char_at(next) {
                    '0'...'9' => {
                        self.scan_number()
                    }
                    _ => {
                        let token_start = self.cursor;
                        let token_end = next;
                        self.cursor = next;
                        Ok(Operator(&self.input[token_start..token_end]))
                    }
                }
            }
            _ => {
                Err("")
            }
        }
    }
}


pub fn lex(input: &str) -> Result<Vec<Token>, &'static str> {
    let mut lexer = Lexer::new(input);
    let mut output = Vec::new();

    loop {
        match lexer.next() {
            Err(error) => {
                return Err(error);
            }
            Ok(EOF) => {
                output.push(EOF);
                return Ok(output);
            }
            Ok(token) => {
                output.push(token);
            }
        }
    }
}


#[test]
fn test_lexer() {
    let program = "(hello - world)";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(OpeningParenthesis));
    assert_eq!(lexer.next(), Ok(VarName("hello")));
    assert_eq!(lexer.next(), Ok(Operator("-")));
    assert_eq!(lexer.next(), Ok(VarName("world")));
    assert_eq!(lexer.next(), Ok(ClosingParenthesis));
    assert_eq!(lexer.next(), Ok(EOF));
    assert_eq!(lexer.next(), Ok(EOF));
}

#[test]
fn test_lexer_numbers() {
    let program = "1 234 -5 6";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(Number("1")));
    assert_eq!(lexer.next(), Ok(Number("234")));
    assert_eq!(lexer.next(), Ok(Number("-5")));
    assert_eq!(lexer.next(), Ok(Number("6")));
    assert_eq!(lexer.next(), Ok(EOF));
}
