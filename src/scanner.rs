use crate::compiler::{Token, TokenType};
use std::ptr::null;

pub struct Scanner {
    start: *const char,
    current: *const char,
    line: isize,
}

static mut SCANNER: Scanner = Scanner {
    start: null(),
    current: null(),
    line: 0,
};

pub unsafe fn init_scanner(source: *const char) {
    unsafe {
        SCANNER.start = source;
        SCANNER.current = source;
        SCANNER.line = 1;
    }
}

fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

unsafe fn is_at_end() -> bool {
    unsafe { *SCANNER.current == '\0' }
}

unsafe fn advance() -> char {
    unsafe {
        SCANNER.current = SCANNER.current.add(1);
        *SCANNER.current.sub(1)
    }
}

unsafe fn peek() -> char {
    unsafe { *SCANNER.current }
}

unsafe fn peek_next() -> char {
    unsafe {
        if is_at_end() {
            '\0'
        } else {
            *SCANNER.current.add(1)
        }
    }
}

unsafe fn r#match(expected: char) -> bool {
    unsafe {
        if is_at_end() {
            return false;
        }
        if *SCANNER.current != expected {
            return false;
        }
        SCANNER.current = SCANNER.current.add(1);
        true
    }
}

unsafe fn make_token(r#type: TokenType) -> Token {
    unsafe {
        Token {
            r#type,
            start: SCANNER.start,
            length: SCANNER.current.offset_from_unsigned(SCANNER.start),
            line: SCANNER.line,
        }
    }
}

unsafe fn error_token(message: &str) -> Token {
    let msg: Vec<char> = message.chars().collect();
    let length = msg.len();
    unsafe {
        Token {
            r#type: TokenType::Error,
            start: msg.leak().as_ptr(),
            length,
            line: SCANNER.line,
        }
    }
}

unsafe fn skip_whitespace() {
    unsafe {
        let mut c;
        loop {
            c = peek();
            match c {
                ' ' | '\r' | '\t' => {
                    advance();
                }
                '\n' => {
                    SCANNER.line += 1;
                    advance();
                }
                '/' => {
                    if peek_next() == '/' {
                        // A comment goes until the end of the line
                        while peek() != '\n' && !is_at_end() {
                            advance();
                        }
                    } else {
                        break;
                    }
                }
                _ => {
                    break;
                }
            }
        }
    }
}

fn check_keyword(start: usize, length: usize, rest: &str, r#type: TokenType) -> TokenType {
    unsafe {
        if (SCANNER.current.offset_from_unsigned(SCANNER.start) == start + length)
            && (std::slice::from_raw_parts(SCANNER.start.add(start), length)
                .iter()
                .map(|&c| c)
                .eq(rest.chars()))
        {
            r#type
        } else {
            TokenType::Identifier
        }
    }
}

unsafe fn identifier_type() -> TokenType {
    unsafe {
        match { *SCANNER.start } {
            'a' => check_keyword(1, 2, "nd", TokenType::And),
            'c' => check_keyword(1, 4, "lass", TokenType::Class),
            'e' => check_keyword(1, 3, "lse", TokenType::Else),
            'f' => {
                if SCANNER.current.offset_from(SCANNER.start) > 1 {
                    match *SCANNER.start.add(1) {
                        'a' => check_keyword(2, 3, "lse", TokenType::False),
                        'o' => check_keyword(2, 1, "r", TokenType::For),
                        'u' => check_keyword(2, 1, "n", TokenType::Fun),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'i' => check_keyword(1, 1, "f", TokenType::If),
            'n' => check_keyword(1, 2, "il", TokenType::Nil),
            'o' => check_keyword(1, 1, "r", TokenType::Or),
            'p' => check_keyword(1, 4, "rint", TokenType::Print),
            'r' => check_keyword(1, 5, "eturn", TokenType::Return),
            's' => check_keyword(1, 4, "uper", TokenType::Super),
            't' => {
                if SCANNER.current.offset_from(SCANNER.start) > 1 {
                    match *SCANNER.start.add(1) {
                        'h' => check_keyword(2, 2, "is", TokenType::This),
                        'r' => check_keyword(2, 2, "ue", TokenType::True),
                        _ => TokenType::Identifier,
                    }
                } else {
                    TokenType::Identifier
                }
            }
            'v' => check_keyword(1, 2, "ar", TokenType::Var),
            'w' => check_keyword(1, 4, "hile", TokenType::While),
            _ => TokenType::Identifier,
        }
    }
}

unsafe fn identifier() -> Token {
    unsafe {
        while is_alpha(peek()) || is_digit(peek()) {
            advance();
        }
        make_token(identifier_type())
    }
}

unsafe fn number() -> Token {
    unsafe {
        while is_digit(peek()) {
            advance();
        }
        // Look for a fractional part
        if peek() == '.' && is_digit(peek_next()) {
            // Consume the "."
            advance();

            while is_digit(peek()) {
                advance();
            }
        }
        make_token(TokenType::Number)
    }
}

unsafe fn string() -> Token {
    unsafe {
        while peek() != '"' && !is_at_end() {
            if peek() == '\n' {
                SCANNER.line += 1
            };
            advance();
        }
        if is_at_end() {
            return error_token("Unterminated string.");
        };
        // The closing quote
        advance();
        make_token(TokenType::String)
    }
}

pub unsafe fn scan_token() -> Token {
    unsafe {
        skip_whitespace();
        SCANNER.start = SCANNER.current;

        if is_at_end() {
            return make_token(TokenType::Eof);
        }

        let c = advance();
        if is_alpha(c) {
            return identifier();
        }
        if is_digit(c) {
            return number();
        }

        match c {
            '(' => make_token(TokenType::LeftParen),
            ')' => make_token(TokenType::RightParen),
            '{' => make_token(TokenType::LeftBrace),
            '}' => make_token(TokenType::RightBrace),
            ';' => make_token(TokenType::Semicolon),
            ',' => make_token(TokenType::Comma),
            '.' => make_token(TokenType::Dot),
            '-' => make_token(TokenType::Minus),
            '+' => make_token(TokenType::Plus),
            '/' => make_token(TokenType::Slash),
            '*' => make_token(TokenType::Star),
            '!' => make_token(if r#match('=') {
                TokenType::BangEqual
            } else {
                TokenType::Bang
            }),
            '=' => make_token(if r#match('=') {
                TokenType::EqualEqual
            } else {
                TokenType::Equal
            }),
            '<' => make_token(if r#match('=') {
                TokenType::LessEqual
            } else {
                TokenType::Less
            }),
            '>' => make_token(if r#match('=') {
                TokenType::GreaterEqual
            } else {
                TokenType::Greater
            }),
            '"' => string(),
            _ => error_token("Unexpected character."),
        }
    }
}
