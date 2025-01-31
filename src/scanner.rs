use crate::token_type::{Object, Token, TokenType};
use std::iter::Peekable;
use std::str::Chars;
use std::{process::exit, str};

fn is_end(current: usize, src: &String) -> bool {
    current >= src.len()
}

pub fn scan_tokens(src: &String) -> Vec<Token> {
    //println!("scan_tokens: {src}");
    let mut tokens: Vec<Token> = Vec::new();
    let mut start: usize = 0;
    let mut current: usize = 0;
    let mut line: usize = 1;

    while !is_end(current, &src) {
        println!("Current: {current}, {}", src.len());
        start = current;
        scan_token(src, start, &mut current, &mut line, &mut tokens);
        println!("Current: {current}, {}", src.len());
    }

    tokens.push(Token {
        type_: TokenType::EOF,
        lexeme: String::from(""),
        line: line,
        literal: Object::Default,
    });
    tokens
}

fn scan_token(
    src: &String,
    start: usize,
    current: &mut usize,
    line: &mut usize,
    tokens: &mut Vec<Token>,
) {
    //println!("scan_token: {src}");
    let mut src_peek = src.chars().peekable();
    let c = advance_str(src, current);

    println!("scan_token: {c}");

    match c {
        '(' => add_token(TokenType::LEFT_PAREN, &src, start, *line, *current, tokens),
        ')' => add_token(TokenType::RIGHT_PAREN, &src, start, *line, *current, tokens),
        '{' => add_token(TokenType::LEFT_BRACE, &src, start, *line, *current, tokens),
        '}' => add_token(TokenType::RIGHT_BRACE, &src, start, *line, *current, tokens),
        ',' => add_token(TokenType::COMMA, &src, start, *line, *current, tokens),
        '.' => {
            add_token(TokenType::DOT, &src, start, *line, *current, tokens);
            println!("matched: {c}");
        },
        '-' => add_token(TokenType::MINUS, &src, start, *line, *current, tokens),
        '+' => add_token(TokenType::PLUS, &src, start, *line, *current, tokens),
        ';' => add_token(TokenType::SEMICOLON, &src, start, *line, *current, tokens),
        '*' => add_token(TokenType::STAR, &src, start, *line, *current, tokens),
        '!' => {
            match match_next('=', &src, current) {
                true => add_token(TokenType::BANG_EQUAL, &src, start, *line, *current, tokens),
                false => add_token(TokenType::BANG, &src, start, *line, *current, tokens)
            };
        },
        '=' => {
            println!("matched: {current}");
            match match_next('=', &src, current) {
                true => add_token(TokenType::EQUAL_EQUAL, &src, start, *line, *current, tokens),
                false => add_token(TokenType::EQUAL, &src, start, *line, *current, tokens)
            };
        },
        '<' => {
            match match_next('=', &src, current) {
                true => add_token(TokenType::LESS_EQUAL, &src, start, *line, *current, tokens),
                false => add_token(TokenType::LESS, &src, start, *line, *current, tokens)
            };
        },
        '>' => {
            match match_next('=', &src, current) {
                true => add_token(TokenType::GREATER_EQUAL, &src, start, *line, *current, tokens),
                false => add_token(TokenType::GREATER, &src, start, *line, *current, tokens)
            };
        },
        '/' => {
            println!("Slash!");
            match match_next('/', &src, current) {
                true => {
                    while *src_peek.peek().unwrap() != '\n' && !is_end(*current, src) {
                        advance_str(src, current);
                    }
                }
                false => {
                    add_token(TokenType::SLASH, src, start, *line, *current, tokens);
                }
            }
        },
        ' ' => (),
        '\r' => (),
        '\t' => (),
        '\n' => {
            *line += 1;
            println!("Newline!");
        },
        _ => {
            eprintln!("Unexpected character!");
            exit(65);
        }
    }
}

fn match_next(exp: char, src: &String, current: &mut usize) -> bool {
    if is_end(*current, src) {
        return false;
    }

    //println!("{exp} and {}", src.as_bytes()[*current] as char);

    if src.as_bytes()[*current] != exp as u8 {
        return false;
    }

    *current += 1;
    true
}

fn advance_str(src: &String, current: &mut usize) -> char {
    *current += 1;
    let c = src.as_bytes()[*current - (1 as usize)] as char;
    c
}

fn add_token(
    type_: TokenType,
    src: &String,
    start: usize,
    line: usize,
    current: usize,
    tokens: &mut Vec<Token>,
) {
    add_token_(type_, src, start, current, line, Object::Default, tokens);
}

fn add_token_(
    type_: TokenType,
    src: &String,
    start: usize,
    current: usize,
    line: usize,
    literal_: Object,
    tokens: &mut Vec<Token>,
) {
    let text = String::from(str::from_utf8(&src.as_bytes()[start..current]).unwrap());
    //println!("add_token_: {text}");
    let token_struct = Token {
        type_: type_,
        lexeme: text,
        line: line,
        literal: literal_,
    };
    tokens.push(token_struct);
}
