use std::cmp::Ordering::{Less, Equal, Greater};

pub use self::Token::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Token<'a> {
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

    TypeName(&'a str),
    VarName(&'a str),
    Operator(&'a str),

    String(&'a str),
    Number(&'a str),

    EOF,
}


type LexerError = &'static str;

type LexerResult<'a> = Result<(Token<'a>, usize), LexerError>;


#[derive(Clone, Debug)]
struct Lexer<'a> {
    input: &'a str,
    token_start: usize,
    indent: usize,
    cursor: usize,
    row: usize,
    col: usize,
}


impl<'a> Lexer<'a> {
    fn new(input : &'a str) -> Lexer<'a> {
        Lexer{
            input: input,
            token_start: 0,
            indent: 0,
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

        let token = match &self.input[token_start..self.cursor] {
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
            _ => VarName(&self.input[token_start..self.cursor]),
        };

        Ok((token, self.indent))
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

        Ok((TypeName(&self.input[token_start..self.cursor]), self.indent))
    }

    fn scan_operator(&mut self) -> LexerResult<'a> {
        // TODO operators can have more than one char
        let token_start = self.cursor;
        self.pop_char();

        Ok((Operator(&self.input[token_start..self.cursor]), self.indent))
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

        Ok((Number(&self.input[token_start..self.cursor]), self.indent))
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

    pub fn next(&mut self) -> LexerResult<'a> {
        self.consume_whitespace();

        self.indent = self.col;

        match self.peek_char() {
            None => Ok((EOF, self.indent)),
            Some(ch) => match ch {
                'a'...'z' => {
                    self.scan_varname()
                }
                'A'...'Z' => {
                    self.scan_typename()
                }
                '0'...'9' => {
                    self.scan_number()
                }
                '[' => {
                    self.pop_char();
                    Ok((LBracket, self.indent))
                }
                ']' => {
                    self.pop_char();
                    Ok((RBracket, self.indent))
                }
                '(' => {
                    self.pop_char();
                    Ok((LParen, self.indent))
                }
                ')' => {
                    self.pop_char();
                    Ok((RParen, self.indent))
                }
                ',' => {
                    self.pop_char();
                    Ok((Comma, self.indent))
                }
                ':' => {
                    self.pop_char();
                    Ok((Colon, self.indent))
                }
                '_' => {
                    self.pop_char();
                    Ok((Underscore, self.indent))
                }
                '@' => {
                    self.pop_char();
                    Ok((At, self.indent))
                }
                '=' => {
                    self.pop_char();
                    Ok((Equals, self.indent))
                }
                '-' => {
                    match self.lookahead_char() {
                        Some('>') => {
                            self.pop_char(); self.pop_char();
                            Ok((RightArrow, self.indent))
                        }
                        Some(' ') | Some('\n') => {
                            let token_start = self.cursor;
                            self.pop_char();
                            Ok((Operator(
                                &self.input[token_start..self.cursor]
                            ), self.indent))
                        }
                        Some('0'...'9') => {
                            self.scan_number()
                        }
                        _ => {
                            Err("unhandled character following '-'")
                        }
                    }
                }
                _ => {
                    panic!("{:?}", self)
                }
            }
        }
    }
}


pub fn lex(input: &str) -> Result<Vec<(Token, usize)>, &'static str> {
    let mut lexer = Lexer::new(input);
    let mut output = Vec::new();

    loop {
        match lexer.next() {
            Err(error) => {
                return Err(error);
            }
            Ok(token @ (EOF, _)) => {
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

    assert_eq!(lexer.next(), Ok(((LParen, 0))));
    assert_eq!(lexer.next(), Ok(((TypeName("Hello"), 1))));
    assert_eq!(lexer.next(), Ok(((Operator("-"), 7))));
    assert_eq!(lexer.next(), Ok(((VarName("world"), 9))));
    assert_eq!(lexer.next(), Ok(((RParen, 14))));
    assert_eq!(lexer.next(), Ok(((EOF, 15))));
}


#[test]
fn test_lexer_numbers() {
    let program = "1 234 -5 6";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(((Number("1"), 0))));
    assert_eq!(lexer.next(), Ok(((Number("234"), 2))));
    assert_eq!(lexer.next(), Ok(((Number("-5"), 6))));
    assert_eq!(lexer.next(), Ok(((Number("6"), 9))));
    assert_eq!(lexer.next(), Ok(((EOF, 10))));
}


#[test]
fn test_lexer_indentation() {
    let program = "0\n  2\n  2\n    4\n      6\n  2\n";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(((Number("0"), 0))));
    assert_eq!(lexer.next(), Ok(((Number("2"), 2))));
    assert_eq!(lexer.next(), Ok(((Number("2"), 2))));
    assert_eq!(lexer.next(), Ok(((Number("4"), 4))));
    assert_eq!(lexer.next(), Ok(((Number("6"), 6))));
    assert_eq!(lexer.next(), Ok(((Number("2"), 2))));
    assert_eq!(lexer.next(), Ok(((EOF, 0))));
}


#[test]
fn test_python_style_list() {
    let program = "a = [\n  1,\n  2,\n]";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(((VarName("a"), 0))));
    assert_eq!(lexer.next(), Ok(((Equals, 2))));
    assert_eq!(lexer.next(), Ok(((LBracket, 4))));
    assert_eq!(lexer.next(), Ok(((Number("1"), 2))));
    assert_eq!(lexer.next(), Ok(((Comma, 3))));
    assert_eq!(lexer.next(), Ok(((Number("2"), 2))));
    assert_eq!(lexer.next(), Ok(((Comma, 3))));
    assert_eq!(lexer.next(), Ok(((RBracket, 0))));
    assert_eq!(lexer.next(), Ok(((EOF, 1))));
}


#[test]
fn test_haskell_style_list() {
    let program = "a =\n  [ 1\n  , 2\n  ]";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(((VarName("a"), 0))));
    assert_eq!(lexer.next(), Ok(((Equals, 2))));
    assert_eq!(lexer.next(), Ok(((LBracket, 2))));
    assert_eq!(lexer.next(), Ok(((Number("1"), 4))));
    assert_eq!(lexer.next(), Ok(((Comma, 2))));
    assert_eq!(lexer.next(), Ok(((Number("2"), 4))));
    assert_eq!(lexer.next(), Ok(((RBracket, 2))));
    assert_eq!(lexer.next(), Ok(((EOF, 3))));
}
