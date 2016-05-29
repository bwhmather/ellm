pub use self::Token::*;


#[derive(PartialEq, Clone, Debug)]
pub enum Token<'a> {
    TypeName(&'a str),
    VarName(&'a str),
    Operator(&'a str),

    String(&'a str),
    Number(&'a str),
    Comma,

    LBracket,
    RBracket,
    LParen,
    RParen,

    Indentation(usize),

    EOF,
}


type LexerError = &'static str;

type LexerResult<'a> = Result<Token<'a>, LexerError>;


#[derive(Clone, Debug)]
struct Lexer<'a> {
    input: &'a str,
    cursor: usize,
    row: usize,
    col: usize,
    first_token: bool,
}


impl<'a> Lexer<'a> {
    fn new(input : &'a str) -> Lexer<'a> {
        Lexer{
            input: input,
            cursor: 0,
            col: 0,
            row: 0,
            first_token: true,
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

        Ok(VarName(&self.input[token_start..self.cursor]))
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

        Ok(TypeName(&self.input[token_start..self.cursor]))
    }

    fn scan_operator(&mut self) -> LexerResult<'a> {
        // TODO operators can have more than one char
        let token_start = self.cursor;
        self.pop_char();

        Ok(Operator(&self.input[token_start..self.cursor]))
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

        Ok(Number(&self.input[token_start..self.cursor]))
    }

    pub fn next(&mut self) -> LexerResult<'a> {
        let mut newline = false;

        if self.first_token {
            newline = true;
            self.first_token = false;
        }

        loop {
            match self.peek_char() {
                Some('\n') => {
                    newline = true;
                    self.pop_char();
                }
                Some(' ') => { self.pop_char(); }
                _ => {
                    if newline {
                        return Ok(Indentation(self.col));
                    }
                    break;
                }
            }
        }

        match self.peek_char() {
            None => Ok(EOF),
            Some(ch) => match ch {
                'a'...'z' => { self.scan_varname() }
                'A'...'Z' => { self.scan_typename() }
                '0'...'9' => { self.scan_number() }

                '[' => { self.pop_char(); Ok(LBracket) }
                ']' => { self.pop_char(); Ok(RBracket) }
                '(' => { self.pop_char(); Ok(LParen) }
                ')' => { self.pop_char(); Ok(RParen) }
                '-' => {
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
    let program = "(Hello - world)";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(Indentation(0)));
    assert_eq!(lexer.next(), Ok(LParen));
    assert_eq!(lexer.next(), Ok(TypeName("Hello")));
    assert_eq!(lexer.next(), Ok(Operator("-")));
    assert_eq!(lexer.next(), Ok(VarName("world")));
    assert_eq!(lexer.next(), Ok(RParen));
    assert_eq!(lexer.next(), Ok(EOF));
    assert_eq!(lexer.next(), Ok(EOF));
}


#[test]
fn test_lexer_numbers() {
    let program = "1 234 -5 6";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(Indentation(0)));
    assert_eq!(lexer.next(), Ok(Number("1")));
    assert_eq!(lexer.next(), Ok(Number("234")));
    assert_eq!(lexer.next(), Ok(Number("-5")));
    assert_eq!(lexer.next(), Ok(Number("6")));
    assert_eq!(lexer.next(), Ok(EOF));
}


#[test]
fn test_lexer_indentation() {
    let program = "    4    \n  2\n0";
    let mut lexer = Lexer::new(program);

    assert_eq!(lexer.next(), Ok(Indentation(4)));
    assert_eq!(lexer.next(), Ok(Number("4")));
    assert_eq!(lexer.next(), Ok(Indentation(2)));
    assert_eq!(lexer.next(), Ok(Number("2")));
    assert_eq!(lexer.next(), Ok(Indentation(0)));
    assert_eq!(lexer.next(), Ok(Number("0")));
    assert_eq!(lexer.next(), Ok(EOF));
}
