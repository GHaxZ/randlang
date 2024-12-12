//  TODO: Fix every character being detected as "Unknown"

#[derive(Debug)]
pub enum Token {
    String(String),
    Integer(i32),
    Decimal(f32),
    Delimiter,       // Delimiter symbols such as space and newline
    True,            // true
    False,           // false
    Comment,         // #
    Add,             // +
    Subtract,        // -
    Multiply,        // *
    Divide,          // /
    Var,             // var
    Set,             // =
    Equals,          // ==
    ScopeStart,      // {
    ScopeEnd,        // }
    Unknown(String), // Non matching value, could be variable
}

impl Token {
    // Match a string to a respective Token
    pub fn from_str(token_string: &str) -> Self {
        match token_string.trim() {
            "#" => Self::Comment,
            "+" => Self::Add,
            "-" => Self::Subtract,
            "*" => Self::Multiply,
            "/" => Self::Divide,
            "var" => Self::Var,
            "=" => Self::Set,
            "==" => Self::Equals,
            "{" => Self::ScopeStart,
            "}" => Self::ScopeEnd,
            "true" => Self::True,
            "false" => Self::False,
            " " | "\n" | "\r" => Self::Delimiter,
            s => {
                if s.starts_with("\"") && s.ends_with("\"") {
                    let str = s
                        .strip_prefix("\"")
                        .unwrap_or(s)
                        .strip_suffix("\"")
                        .unwrap_or(s);
                    return Self::String(str.to_string());
                }

                if let Ok(int) = s.parse::<i32>() {
                    return Self::Integer(int);
                }

                if let Ok(dec) = s.parse::<f32>() {
                    return Self::Decimal(dec);
                }

                return Self::Unknown(s.to_string());
            }
        }
    }
}

pub struct Lexer {}

impl Lexer {
    pub fn tokenize(input: &str) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut current = String::new();

        for c in input.chars() {
            current.push(c);

            let token = Token::from_str(&current);
            match token {
                Token::Delimiter => {
                    if !current.trim().is_empty() {
                        tokens.push(Token::from_str(&current));
                    }
                    current.clear();
                }
                t => {
                    tokens.push(t);
                    current.clear();
                }
            }
        }

        if !current.is_empty() {
            tokens.push(Token::from_str(&current));
        }

        tokens
    }
}
