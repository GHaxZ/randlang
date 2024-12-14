//  TODO:, returns Err if there errors occur during interpretation
//  Write tests for lexer (I don't want to write tests, but if this isn't the perfect use case for
//  tests, I don't know what is)

// Possible string tokens
#[derive(Debug)]
pub enum Token {
    // Literals
    String(String),
    Integer(i32),
    Decimal(f32),
    Boolean(bool),

    // Keywords
    Var,     // var
    Comment, // //

    // Operators
    Add,      // +
    Subtract, // -
    Multiply, // *
    Divide,   // /
    Assign,   // =
    Equals,   // ==

    // Scopes
    ScopeStart, // {
    ScopeEnd,   // }

    // Identifiers
    Identifier(String),

    // Other tokens
    Unknown(String),
    Whitespace,
}

// Lexer used for converting strings to code tokens
pub struct Lexer {
    content: Vec<char>,
    position: usize,
}

impl Lexer {
    // Create new lexer with specified content
    fn new(content: &str) -> Self {
        Self {
            content: content.chars().collect(),
            position: 0,
        }
    }

    // Get next character without advancing forward
    fn peek(&self) -> Option<char> {
        self.content.get(self.position).cloned()
    }

    // Get next character and advance forward
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;

        self.position += 1;

        Some(c)
    }

    // Skip the rest of the current line
    fn next_line(&mut self) {
        while let Some(c) = self.content.get(self.position) {
            self.position += 1;

            if *c == 0xA as char {
                break;
            }
        }
    }

    // Read a string token
    //  TODO: Figure out how to deal with unclosed strings
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

    // Read a number token
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

    // Read an identifier token
    fn read_identifier(&mut self, first_character: char) -> String {
        let mut identifier = String::from(first_character);

        while let Some(c) = self.peek() {
            if self.is_identifier_part(c) {
                identifier.push(c);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    // Returns corresponding token, if the specified string is a valid keyword
    fn check_keyword(&self, string: &str) -> Option<Token> {
        match string {
            "var" => Some(Token::Var),
            "true" => Some(Token::Boolean(true)),
            "false" => Some(Token::Boolean(false)),
            _ => None,
        }
    }

    // Checks if the specified character is a valid identifier start character
    fn is_identifier_start(&self, character: char) -> bool {
        character.is_alphabetic() || character == '_' || character == '$'
    }

    // Checks if the specified character is a valid identifier character
    fn is_identifier_part(&self, character: char) -> bool {
        character.is_alphanumeric() || character == '_'
    }

    // Get next token from current position, None no more characters are available
    //  TODO: Create check_symbol function similar to check_keyword for operators and delimiters
    fn next_token(&mut self) -> Option<Token> {
        let mut current = String::new();

        while let Some(c) = self.advance() {
            current.push(c);

            let token = match c {
                '+' => Some(Token::Add),
                '-' => Some(Token::Subtract),
                '*' => Some(Token::Multiply),
                '{' => Some(Token::ScopeStart),
                '}' => Some(Token::ScopeEnd),
                '=' => {
                    if let Some(next) = self.peek() {
                        if next == '=' {
                            self.advance();
                            Some(Token::Equals)
                        } else {
                            Some(Token::Assign)
                        }
                    } else {
                        Some(Token::Assign)
                    }
                }
                '/' => {
                    if let Some(next) = self.peek() {
                        if next == '/' {
                            self.advance();
                            self.next_line();
                            Some(Token::Comment)
                        } else {
                            Some(Token::Divide)
                        }
                    } else {
                        Some(Token::Divide)
                    }
                }

                '"' => Some(self.read_string()),

                c if c.is_whitespace() => Some(Token::Whitespace),

                c if c.is_digit(10) => {
                    return Some(self.read_number(c));
                }

                c if self.is_identifier_start(c) => {
                    let identifier = self.read_identifier(c);

                    if let Some(keyword_token) = self.check_keyword(&identifier) {
                        return Some(keyword_token);
                    }

                    return Some(Token::Identifier(identifier));
                }

                _ => Some(Token::Unknown(current.clone())),
            };

            return token;
        }

        None
    }

    // Turn a string of source code, into a Vec of Tokens
    pub fn tokenize(content: &str) -> Vec<Token> {
        let mut lexer = Self::new(content);
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(token) = lexer.next_token() {
            match token {
                Token::Whitespace => {}
                Token::Comment => {}
                t => tokens.push(t),
            }
        }

        tokens
    }
}
