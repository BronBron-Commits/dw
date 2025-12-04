#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Number(f32),
    StringLiteral(String),
    Comma,
    BeginBlock,
    EndBlock,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&ch) = chars.peek() {
        match ch {
            '{' => {
                tokens.push(Token::BeginBlock);
                chars.next();
            }
            '}' => {
                tokens.push(Token::EndBlock);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            '"' => {
                chars.next(); 
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2 == '"' {
                        break;
                    }
                    s.push(c2);
                    chars.next();
                }
                chars.next(); 
                tokens.push(Token::StringLiteral(s));
            }
            c if c.is_ascii_digit() || c == '-' || c == '.' => {
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_ascii_digit() || c2 == '.' || c2 == '-' {
                        s.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                if let Ok(n) = s.parse::<f32>() {
                    tokens.push(Token::Number(n));
                }
            }
            c if c.is_ascii_alphabetic() => {
                let mut s = String::new();
                while let Some(&c2) = chars.peek() {
                    if c2.is_alphanumeric() || c2 == '_' {
                        s.push(c2);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Ident(s));
            }
            _ => {
                chars.next();
            }
        }
    }

    tokens
}
