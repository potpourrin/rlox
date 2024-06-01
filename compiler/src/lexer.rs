use std::{
    collections::HashMap,
    default,
    error::Error,
    ops::{RangeBounds, RangeInclusive},
    str::Chars,
    usize,
};

#[derive(Debug, Clone)]
pub enum TokenType {
    // Single-character tokens.
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
    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    // Literals.
    Identifier,
    String(String),
    Number(f64),
    // kEYWORDS.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
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
    Eof,
}

macro_rules! IS_DIGIT {
    () => {
        '0'..='9'
    };
    ($char:expr) => {
        ('0'..='9').contains($char)
    };
}

macro_rules! IS_ALPHA {
    () => {
        'a'..='z' | 'A'..='Z' | '_'
    };
    ($char:expr) => {
        ('a'..='z').contains($char) || ('A'..='Z').contains($char) || '_'.eq($char)
    };
}

macro_rules! IS_ALPHANUMERIC {
    () => {
        IS_ALPHA!() | IS_DIGIT!()
    };
    ($char:expr) => {
        IS_ALPHA!($char) || IS_DIGIT!($char)
    };
}

macro_rules! match_lexeme {
    ($self:expr, $expected:expr, $then:expr, $otherwise:expr) => {{
        if $self.is_at_end() || $self.source.chars().nth($self.current).unwrap() != $expected {
            $self.add_token($otherwise);
        } else {
            $self.current += 1;
            $self.add_token($then);
        }
    }};
}

macro_rules! match_lexeme_peek {
    ($self:expr, $expected:expr) => {{
        if $self.is_at_end() || $self.source.chars().nth($self.current).unwrap() != $expected {
            false
        } else {
            true
        }
    }};
}

pub struct Lexer {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(source: String) -> Lexer {
        let mut keywords = HashMap::new();
        keywords.insert(String::from("and"), TokenType::And);
        keywords.insert(String::from("class"), TokenType::Class);
        keywords.insert(String::from("else"), TokenType::Else);
        keywords.insert(String::from("false"), TokenType::False);
        keywords.insert(String::from("for"), TokenType::For);
        keywords.insert(String::from("fun"), TokenType::Fun);
        keywords.insert(String::from("if"), TokenType::If);
        keywords.insert(String::from("nil"), TokenType::Nil);
        keywords.insert(String::from("or"), TokenType::Or);
        keywords.insert(String::from("print"), TokenType::Print);
        keywords.insert(String::from("return"), TokenType::Return);
        keywords.insert(String::from("super"), TokenType::Super);
        keywords.insert(String::from("this"), TokenType::This);
        keywords.insert(String::from("true"), TokenType::True);
        keywords.insert(String::from("var"), TokenType::Var);
        keywords.insert(String::from("while"), TokenType::While);

        return Self {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            keywords,
        };
    }

    fn scan_token(&mut self) -> Result<(), (usize, String)> {
        let c = self.advance().unwrap();

        let new_error = match c {
            '(' => Ok(self.add_token(TokenType::LeftParen)),
            ')' => Ok(self.add_token(TokenType::RightParen)),
            '{' => Ok(self.add_token(TokenType::LeftBrace)),
            '}' => Ok(self.add_token(TokenType::RightBrace)),
            ',' => Ok(self.add_token(TokenType::Comma)),
            '.' => Ok(self.add_token(TokenType::Dot)),
            '-' => Ok(self.add_token(TokenType::Minus)),
            '+' => Ok(self.add_token(TokenType::Plus)),
            ';' => Ok(self.add_token(TokenType::Semicolon)),
            '*' => Ok(self.add_token(TokenType::Star)),
            '!' => Ok(match_lexeme!(
                self,
                '=',
                TokenType::BangEqual,
                TokenType::Bang
            )),
            '=' => Ok(match_lexeme!(
                self,
                '=',
                TokenType::EqualEqual,
                TokenType::Equal
            )),
            '<' => Ok(match_lexeme!(
                self,
                '=',
                TokenType::LessEqual,
                TokenType::Less
            )),
            '>' => Ok(match_lexeme!(
                self,
                '=',
                TokenType::GreaterEqual,
                TokenType::Greater
            )),
            '/' => Ok({
                if match_lexeme_peek!(self, '/') {
                    while self.peek() != '\n' && self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }),
            ' ' | '\r' | '\t' => Ok(()),
            '\n' => Ok(self.line += 1),
            '"' => self.string(),
            IS_DIGIT!() => Ok(self.number()),
            IS_ALPHA!() => Ok(self.identifier()),
            _ => Err(String::from("Unexpected character")),
        };

        if let Err(new_error) = new_error {
            return Err((self.line, new_error));
        }

        return Ok(());
    }

    fn identifier(&mut self) {
        while IS_ALPHANUMERIC!(&self.peek()) {
            self.advance();
        }

        let text = self.source.get(self.start..self.current).unwrap();
        let token = if let Some(token) = self.keywords.get(text) {
            token.to_owned()
        } else {
            TokenType::Identifier
        };

        self.add_token(token);
    }

    fn number(&mut self) {
        while IS_DIGIT!(&self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && IS_DIGIT!(&self.peek_next()) {
            self.advance();

            while IS_DIGIT!(&self.peek()) {
                self.advance();
            }
        }

        let value: f64 = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .parse()
            .unwrap();

        self.add_token(TokenType::Number(value))
    }

    fn peek_next(&self) -> char {
        let next = self.current + 1;

        if next >= self.source.len() {
            return '\0';
        }

        return self.source.chars().nth(next).unwrap();
    }

    fn string(&mut self) -> Result<(), String> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            return Err(String::from("Unterminated string"));
        }

        self.advance();

        let value = self
            .source
            .get(self.start + 1..self.current - 1)
            .unwrap()
            .to_string();

        self.add_token(TokenType::String(value));

        return Ok(());
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        };

        return self.source.chars().nth(self.current).unwrap();
    }

    fn advance(&mut self) -> Option<char> {
        let get = self.source.chars().nth(self.current);

        self.current += 1;
        return get;
    }

    fn add_token(&mut self, token: TokenType) {
        let text = self
            .source
            .get(self.start..self.current)
            .unwrap()
            .to_string();

        self.tokens.push(Token::new(token, text, self.line));
    }
    pub fn scan_tokens(&mut self) -> Result<(), Vec<(usize, String)>> {
        let mut had_error = true;
        let mut errors = Vec::new();

        while !self.is_at_end() {
            self.start = self.current;

            if let Err(new_error) = self.scan_token() {
                errors.push(new_error);

                had_error = true;
            }
        }

        if had_error {
            return Err(errors);
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "".to_string(), self.line));

        return Ok(());
    }

    fn is_at_end(&self) -> bool {
        return self.current >= self.source.len();
    }
}

#[derive(Debug)]
pub struct Token {
    r#type: TokenType,
    lexeme: String,
    line: usize,
}

impl Token {
    pub fn new(r#type: TokenType, lexeme: String, line: usize) -> Self {
        Self {
            r#type,
            lexeme,
            line,
        }
    }
}
impl ToString for Token {
    fn to_string(&self) -> String {
        return format_args!("{:?} {:?} {}", self.r#type, self.lexeme, self.line).to_string();
    }
}
