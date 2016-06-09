use std::cmp::Ordering::{Less, Equal, Greater};

pub use self::Token::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Token {
    LBracket,
    RBracket,
    LParen,
    RParen,

    Colon,
    RightArrow,
    Underscore,
    At,
    Equals,
    Comma,

    Type,
    Alias,
    Port,
    Module,
    Exposing,
    Where,
    Let,
    In,
    Case,
    Of,
    If,
    Then,
    Else,

    TypeName,
    VarName,
    Operator,

    String,
    Number,

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

        let token = match &self.input[self.token_start..self.cursor] {
            "type" => Type,
            "alias" => Alias,
            "port" => Port,
            "module" => Module,
            "exposing" => Exposing,
            "where" => Where,
            "let" => Let,
            "in" => In,
            "case" => Case,
            "of" => Of,
            "if" => If,
            "then" => Then,
            "else" => Else,
            _ => VarName,
        };

        self.end_token(token)
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

                '[' => { self.pop_char(); Ok(LBracket, self.indentation) }
                ']' => { self.pop_char(); Ok(RBracket, self.indentation) }
                '(' => { self.pop_char(); Ok(LParen, self.indentation) }
                ')' => { self.pop_char(); Ok(RParen, self.indentation) }
                ',' => { self.pop_char(); Ok(Comma, self.indentation) }
                ':' => { self.pop_char(); Ok(Colon, self.indentation) }
                '_' => { self.pop_char(); Ok(Underscore, self.indentation) }
                '@' => { self.pop_char(); Ok(At, self.indentation) }
                '=' => { self.pop_char(); Ok(Equals, self.indentation) }
                '-' => {
                    match self.lookahead_char() {
                        // Some('0'...'9') => { self.scan_number() }
                        Some(ch) => {
                            match ch {
                                '>' => {
                                    self.pop_char(); self.pop_char();
                                    self.end_token(RightArrow)
                                }
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
    assert_eq!(lexer.next(), Ok((2, Equals, "=")));
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
    assert_eq!(lexer.next(), Ok((2, Equals, "=")));
    assert_eq!(lexer.next(), Ok((2, LBracket, "[")));
    assert_eq!(lexer.next(), Ok((4, Number, "1")));
    assert_eq!(lexer.next(), Ok((2, Comma, ",")));
    assert_eq!(lexer.next(), Ok((4, Number, "2")));
    assert_eq!(lexer.next(), Ok((2, RBracket, "]")));
    assert_eq!(lexer.next(), Ok((3, EOF, "")));
}
