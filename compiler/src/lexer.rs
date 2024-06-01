use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
pub struct Token {
    t: TokenType,
    line: usize,
}

impl Token {
    pub fn new(t: TokenType, line: usize) -> Self {
        Self { t, line }
    }
}

#[derive(Debug)]
#[rustfmt::skip]
pub enum TokenType {
    None,
    Comment,
    // Single-character tokens.
    LeftParen, RightParen,
    LeftBrace, RightBrace,
    Comma, Dot, Semicolon, Slash, Star,
    Minus, Plus,     
    // One or two character tokens.
    Bang, BangEqual, 
    Equal,EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,
    // Literals.
    Identifier(String),
    String(String),
    Number(f64),
    // Keywords.
    If, Else, And, Or, False, True,
    Class, Super, This,
    Var, For,While,
    Fun, Print,
    Return,Nil,
    Eof,
}

#[derive(Debug)]
pub enum LexingErrorType {
    UnexpectedCharacter,
    UnterminatedString,
    NumberPrasingError,
}

#[derive(Debug)]
pub struct LexingError {
    t: LexingErrorType,
    line: usize,
}

impl LexingError {
    pub fn new(t: LexingErrorType, line: usize) -> Self {
        Self { t, line }
    }
}

#[inline]
fn match_next_lexeme(
    source: &mut Peekable<Chars>,
    expected: char,
    then: TokenType,
    otherwise: TokenType,
) -> TokenType {
    if source.next_if_eq(&expected).is_some() {
        source.next();
        then
    } else {
        otherwise
    }
}

fn which_keyword(identifier: &str) -> Option<TokenType> {
    let identifier = match identifier {
        "if" => TokenType::If,
        "else" => TokenType::Else,
        "and" => TokenType::And,
        "or" => TokenType::Or,
        "false" => TokenType::False,
        "true" => TokenType::True,
        "class" => TokenType::Class,
        "super" => TokenType::Super,
        "this" => TokenType::This,
        "var" => TokenType::Var,
        "for" => TokenType::For,
        "while" => TokenType::While,
        "fun" => TokenType::Fun,
        "print" => TokenType::Print,
        "return" => TokenType::Return,
        "nil" => TokenType::Nil,
        _ => return None,
    };

    Some(identifier)
}

#[allow(clippy::too_many_lines)]
fn scan_token(
    source: &mut Peekable<Chars>,
    line: &mut usize,
) -> Option<Result<(Token, usize), LexingError>> {
    let token = if let Some(char) = source.next() {
        match char {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => match_next_lexeme(source, '=', TokenType::BangEqual, TokenType::Bang),
            '=' => match_next_lexeme(source, '=', TokenType::EqualEqual, TokenType::Equal),
            '<' => match_next_lexeme(source, '=', TokenType::LessEqual, TokenType::Less),
            '>' => match_next_lexeme(source, '=', TokenType::GreaterEqual, TokenType::Greater),
            '/' => {
                let token = match_next_lexeme(source, '/', TokenType::Comment, TokenType::Slash);

                if let TokenType::Comment = token {
                    // consume until end of the line
                    while let Some(next_char) = source.peek() {
                        let next_char = *next_char;
                        source.next();

                        if next_char == '\n' {
                            break;
                        }
                    }

                    TokenType::Comment
                } else {
                    TokenType::Slash
                }
            }
            ' ' | '\r' | '\t' => TokenType::None,
            '\n' => {
                *line += 1;
                TokenType::None
            }
            '"' => {
                let mut found_termination = false;
                let mut string_value = String::new();
                while let Some(next_char) = source.peek() {
                    let next_char = *next_char;

                    if next_char == '"' {
                        found_termination = true;
                        source.next();
                        break;
                    }

                    string_value.push(next_char);

                    source.next();

                    if next_char == '\n' {
                        *line += 1;
                    }
                }

                if !found_termination {
                    return Some(Err(LexingError::new(
                        LexingErrorType::UnterminatedString,
                        *line,
                    )));
                }

                TokenType::String(string_value)
            }
            '0'..='9' => {
                let mut number_value = String::new();
                while let Some(next_char) = source.peek() {
                    let next_char = *next_char;
                    source.next();

                    if next_char.is_ascii_digit() {
                        number_value.push(next_char);
                    } else if next_char == '.' {
                        number_value.push(next_char);

                        while next_char.is_ascii_digit() {
                            number_value.push(next_char);
                        }

                        break;
                    }
                }

                if let Ok(number) = number_value.parse::<f64>() {
                    TokenType::Number(number)
                } else {
                    return Some(Err(LexingError::new(
                        LexingErrorType::NumberPrasingError,
                        *line,
                    )));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::from(char);

                while let Some(next_char) = source.peek() {
                    if next_char.is_alphanumeric() || *next_char == '_' {
                        identifier.push(*next_char);
                        source.next();
                    } else {
                        break;
                    }
                }

                if let Some(keyword) = which_keyword(&identifier) {
                    keyword
                } else {
                    TokenType::Identifier(identifier)
                }
            }
            _ => {
                return Some(Err(LexingError::new(
                    LexingErrorType::UnexpectedCharacter,
                    *line,
                )))
            }
        }
    } else {
        return None;
    };

    Some(Ok((Token::new(token, *line), *line)))
}

pub fn scan_tokens(source: &str) -> Result<Vec<Token>, Vec<LexingError>> {
    let mut source = source.chars().peekable();
    let mut line = 0;

    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    while let Some(lixing_result) = scan_token(&mut source, &mut line) {
        match lixing_result {
            Ok((token, new_line)) => {
                tokens.push(token);
                line = new_line;
            }
            Err(lexing_error) => errors.push(lexing_error),
        }
    }

    tokens.push(Token::new(TokenType::Eof, line));

    if !errors.is_empty() {
        return Err(errors);
    }

    Ok(tokens)
}
