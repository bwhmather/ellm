use std::str::CharRange;

pub use self::Token::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Token<'a> {
    TypeName(&'a str),
    VarName(&'a str),
    Number(&'a str),
    OpeningParenthesis,
    ClosingParenthesis,
    Comma,
    Operator,
    EOF,
}


pub type TokenizerResult<'a> = Result<Token<'a>, &'static str>;


pub struct Tokenizer<'a> {
    input: &'a str,
    cursor: usize,
}


impl<'a> Tokenizer<'a> {
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

    fn scan_varname(&mut self) -> TokenizerResult<'a> {
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

    fn scan_typename(&mut self) -> TokenizerResult<'a> {
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

    fn scan_number(&mut self) -> TokenizerResult<'a> {
        let re = regex!(r"^[0-9]+");

        match re.find(&self.input[self.cursor..]) {
            Some((token_start, token_end)) => {
                self.cursor = token_end;
                Ok(Number(&self.input[token_start..token_end]))
            }
            None => {
                Err("invalid number")
            }
        }
    }

    pub fn next(&mut self) -> TokenizerResult<'a> {
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
            '0'...'0' => {
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
                self.cursor = next;
                Err("Not implemented")
            }
            _ => {
                Err("")
            }
        }
    }
}


pub fn tokenize(input: &str) -> Tokenizer {
    Tokenizer{
        input: input,
        cursor: 0,
    }
}


#[test]
fn test_tokenize() {
    let program = "(hello world)";
    let mut tokenizer = tokenize(program);

    assert_eq!(tokenizer.next(), Ok(OpeningParenthesis));
    assert_eq!(tokenizer.next(), Ok(VarName("hello")));
    assert_eq!(tokenizer.next(), Ok(VarName("world")));
    assert_eq!(tokenizer.next(), Ok(ClosingParenthesis));
    assert_eq!(tokenizer.next(), Ok(EOF));
    assert_eq!(tokenizer.next(), Ok(EOF));
}
