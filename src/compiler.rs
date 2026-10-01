use crate::scanner::{init_scanner, scan_token};

#[derive(Clone, Debug)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Error,
    Eof,
}

#[derive(Clone, Debug)]
pub struct Token {
    pub r#type: TokenType,
    pub start: *const char,
    pub length: usize,
    pub line: isize,
}

pub unsafe fn compile(source: &str) {
    unsafe {
        let source_chars: Vec<char> = source.chars().chain(std::iter::once('\0')).collect();
        init_scanner(source_chars.leak().as_ptr());
        let mut line = -1;
        loop {
            let token = scan_token();
            if token.line != line {
                print!("{:4} ", token.line);
                line = token.line;
            } else {
                print!("   | ")
            }
            let s: String = std::slice::from_raw_parts(token.start, token.length)
                .into_iter()
                .collect();
            println!("{:2} '{}'", token.r#type.clone() as usize, s);

            if matches!(token.r#type, TokenType::Eof) {
                break;
            }
        }
    }
}
