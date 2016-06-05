use std::cmp::Ordering::{Less, Equal, Greater};

pub use self::Token::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    TypeName,
    VarName,
    Operator,

    String,
    Number,
    Comma,

    LBracket,
    RBracket,
    LParen,
    RParen,

    EOF,
}


type LexerError = &'static str;

type LexerResult<'a> = Result<(usize, Token, &'a str), LexerError>;


#[derive(Clone, Debug)]
struct Lexer<'a> {
    input: &'a str,
    token_start: usize,
    indentation: usize,
    cursor: usize,
    row: usize,
    col: usize,
}


impl<'a> Lexer<'a> {
    fn new(input : &'a str) -> Lexer<'a> {
        Lexer{
            input: input,
            token_start: 0,
            indentation: 0,
            cursor: 0,
            col: 0,
            row: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        if self.input.len() == self.cursor {
            None
        } else {
            self.input[self.cursor..].chars().next()
        }
    }

    fn lookahead_char(&self) -> Option<char> {
        if self.input.len() <= self.cursor {
            return None;
        }

        let mut chars = self.input[self.cursor..].chars();
        chars.next();
        chars.next()
    }

    fn pop_char(&mut self) {
        if self.input.len() <= self.cursor {
            return;
        }

        let ch = self.input[self.cursor..].chars().next().unwrap();

        if ch == '\n' {
            self.row += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }

        self.cursor += ch.len_utf8();
    }

    fn scan_varname(&mut self) -> LexerResult<'a> {
        let token_start = self.cursor;

        loop {
            match self.peek_char() {
                Some(ch) => {
                    match ch {
                        'a'...'z' | 'A'...'Z' => { self.pop_char(); }
                        _ => break,
                    }
                }
                None => { break },
            }
        }

        self.end_token(VarName)
    }

    fn scan_typename(&mut self) -> LexerResult<'a> {
        let token_start = self.cursor;

        loop {
            match self.peek_char() {
                Some(ch) => {
                    match ch {
                        'a'...'z' | 'A'...'Z' => { self.pop_char(); }
                        _ => break,
                    }
                }
                None => break,
            }
        }

        self.end_token(TypeName)
    }

    fn scan_operator(&mut self) -> LexerResult<'a> {
        // TODO operators can have more than one char
        let token_start = self.cursor;
        self.pop_char();

        self.end_token(Operator)
    }

    fn scan_number(&mut self) -> LexerResult<'a> {
        let token_start = self.cursor;

        match self.peek_char() {
            Some('-') => self.pop_char(),
            _ => (),
        }

        loop {
            match self.peek_char() {
                Some(ch) => {
                    match ch {
                        '0'...'9' => {
                            self.pop_char();
                        }
                        _ => break,
                    }
                }
                None => break,
            }
        }

        self.end_token(Number)
    }

    fn consume_whitespace(&mut self) {
        loop {
            match self.peek_char() {
                Some(' ') | Some('\n') => {
                    self.pop_char();
                }
                _ => {
                    break;
                }
            }
        }
    }

    fn start_token(&mut self) {
        self.token_start = self.cursor;
        self.indentation = self.col;
    }

    fn end_token(&self, token: Token) -> LexerResult<'a> {
        Ok((
            self.indentation,
            token,
            &self.input[self.token_start..self.cursor]
        ))
    }

    pub fn next(&mut self) -> LexerResult<'a> {
        self.consume_whitespace();
        self.start_token();

        match self.peek_char() {
            None => self.end_token(EOF),
            Some(ch) => match ch {
                'a'...'z' => { self.scan_varname() }
                'A'...'Z' => { self.scan_typename() }
                '0'...'9' => { self.scan_number() }

                '[' => { self.pop_char(); self.end_token(LBracket) }
                ']' => { self.pop_char(); self.end_token(RBracket) }
                '(' => { self.pop_char(); self.end_token(LParen) }
                ')' => { self.pop_char(); self.end_token(RParen) }
                ',' => { self.pop_char(); self.end_token(Comma) }
                '-' | '=' => {
                    match self.lookahead_char() {
                        // Some('0'...'9') => { self.scan_number() }
                        Some(ch) => {
                            match ch {
                                '0'...'9' => { self.scan_number() }
                                _ => { self.scan_operator() }
                            }
                        }

                        _ => { self.scan_operator() }
                    }
                }
                _ => {
                    panic!("{:?}", self)
                }
            }
        }
    }
}


pub fn lex(input: &str) -> Result<Vec<(usize, Token, &str)>, &'static str> {
    let mut lexer = Lexer::new(input);
    let mut output = Vec::new();

    loop {
        match lexer.next() {
            Err(error) => {
                return Err(error);
            }
            Ok(token @ (_, EOF, _)) => {
                output.push(token);
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
    let program = "(Hello - world)";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok((0, LParen, "(")));
    assert_eq!(lexer.next(), Ok((1, TypeName, "Hello")));
    assert_eq!(lexer.next(), Ok((7, Operator, "-")));
    assert_eq!(lexer.next(), Ok((9, VarName, "world")));
    assert_eq!(lexer.next(), Ok((14, RParen, ")")));
    assert_eq!(lexer.next(), Ok((15, EOF, "")));
}


#[test]
fn test_lexer_numbers() {
    let program = "1 234 -5 6";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok((0, Number, "1")));
    assert_eq!(lexer.next(), Ok((2, Number, "234")));
    assert_eq!(lexer.next(), Ok((6, Number, "-5")));
    assert_eq!(lexer.next(), Ok((9, Number, "6")));
    assert_eq!(lexer.next(), Ok((10, EOF, "")));
}


#[test]
fn test_lexer_indentation() {
    let program = "0\n  2\n  2\n    4\n      6\n  2\n";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok((0, Number, "0")));
    assert_eq!(lexer.next(), Ok((2, Number, "2")));
    assert_eq!(lexer.next(), Ok((2, Number, "2")));
    assert_eq!(lexer.next(), Ok((4, Number, "4")));
    assert_eq!(lexer.next(), Ok((6, Number, "6")));
    assert_eq!(lexer.next(), Ok((2, Number, "2")));
    assert_eq!(lexer.next(), Ok((0, EOF, "")));
}


#[test]
fn test_python_style_list() {
    let program = "a = [\n  1,\n  2,\n]";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok((0, VarName, "a")));
    assert_eq!(lexer.next(), Ok((2, Operator, "=")));
    assert_eq!(lexer.next(), Ok((4, LBracket, "[")));
    assert_eq!(lexer.next(), Ok((2, Number, "1")));
    assert_eq!(lexer.next(), Ok((3, Comma, ",")));
    assert_eq!(lexer.next(), Ok((2, Number, "2")));
    assert_eq!(lexer.next(), Ok((3, Comma, ",")));
    assert_eq!(lexer.next(), Ok((0, RBracket, "]")));
    assert_eq!(lexer.next(), Ok((1, EOF, "")));
}


#[test]
fn test_haskell_style_list() {
    let program = "a =\n  [ 1\n  , 2\n  ]";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok((0, VarName, "a")));
    assert_eq!(lexer.next(), Ok((2, Operator, "=")));
    assert_eq!(lexer.next(), Ok((2, LBracket, "[")));
    assert_eq!(lexer.next(), Ok((4, Number, "1")));
    assert_eq!(lexer.next(), Ok((2, Comma, ",")));
    assert_eq!(lexer.next(), Ok((4, Number, "2")));
    assert_eq!(lexer.next(), Ok((2, RBracket, "]")));
    assert_eq!(lexer.next(), Ok((3, EOF, "")));
}
