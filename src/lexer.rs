//  TODO: Simplify code (remove skip_whitespace, handle unexpected characters better)
//          Fix comments on same line as code (skip the line)
//          Try to remove regex dependency maybe

use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Token {
    // Literals
    String(String),
    Integer(i32),
    Decimal(f32),

    // Keywords
    Var,   // var
    True,  // true
    False, // false

    // Operators
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Assign,   // =
    Equals,   // ==

    // Delimiters
    ScopeStart, // {
    ScopeEnd,   // }
    Comment,    // #

    // Identifiers and whitespace
    Identifier(String),
    Whitespace,
}

fn identifier_regex() -> Regex {
    Regex::new(r"^[a-zA-Z_$][\w$]*$").unwrap()
}

#[derive(Debug)]
pub struct Lexer {
    input: Vec<char>,
    position: usize,
    line: usize,
    column: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
            line: 1,
            column: 1,
        }
    }

    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.position += 1;
        self.column += 1;

        if c == '\n' {
            self.line += 1;
            self.column = 1;
        }

        Some(c)
    }

    fn skip_whitespace(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }
            self.advance();
        }
        Token::Whitespace
    }

    fn read_number(&mut self, first_digit: char) -> Token {
        let mut value = String::from(first_digit);
        let mut has_decimal = false;

        while let Some(c) = self.peek() {
            if c == '.' && !has_decimal {
                has_decimal = true;
                value.push(c);
                self.advance();
            } else if c.is_digit(10) {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        if has_decimal {
            Token::Decimal(value.parse().unwrap())
        } else {
            Token::Integer(value.parse().unwrap())
        }
    }

    fn read_identifier(&mut self, first_char: char) -> Token {
        let mut value = String::from(first_char);

        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                value.push(c);
                self.advance();
            } else {
                break;
            }
        }

        match value.as_str() {
            "var" => Token::Var,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Identifier(value),
        }
    }

    fn read_string(&mut self) -> Token {
        let mut value = String::new();

        while let Some(c) = self.advance() {
            if c == '"' {
                break;
            }
            value.push(c);
        }

        Token::String(value)
    }

    pub fn next_token(&mut self) -> Option<Token> {
        let c = self.advance()?;

        let token = match c {
            c if c.is_whitespace() => self.skip_whitespace(),
            c if c.is_digit(10) => self.read_number(c),
            c if identifier_regex().is_match(c.to_string().as_str()) => self.read_identifier(c),
            '"' => self.read_string(),
            '+' => Token::Add,
            '-' => Token::Subtract,
            '*' => Token::Multiply,
            '/' => Token::Divide,
            '=' => {
                if let Some('=') = self.peek() {
                    self.advance();
                    Token::Equals
                } else {
                    Token::Assign
                }
            }
            '{' => Token::ScopeStart,
            '}' => Token::ScopeEnd,
            '#' => Token::Comment,

            _ => return None,
        };

        Some(token)
    }

    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut lexer = Self::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next_token() {
            if token != Token::Whitespace {
                tokens.push(token);
            }
        }

        tokens
    }
}
