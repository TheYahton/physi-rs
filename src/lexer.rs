#[derive(Debug)]
pub enum Token {
    End,
    NumLiteral(u32),
    Unit(String),
    Unknown(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
    LeftParen,
    RightParen,
    Equal,
}

pub trait Lexer {
    fn tokenize(&self) -> Vec<Token>;
}

impl Lexer for String {
    fn tokenize(&self) -> Vec<Token> {
        let mut tokens = Vec::new();
        let mut chars = self.chars().peekable();

        while let Some(&ch) = chars.peek() {
            // println!("{chars:?}");
            match ch {
                ch if ch.is_whitespace() => {
                    // Skip over any whitespace
                    chars.next();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    chars.next();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    chars.next();
                }
                '*' => {
                    tokens.push(Token::Asterisk);
                    chars.next();
                }
                '/' => {
                    tokens.push(Token::Slash);
                    chars.next();
                }
                '(' => {
                    tokens.push(Token::LeftParen);
                    chars.next();
                }
                ')' => {
                    tokens.push(Token::RightParen);
                    chars.next();
                }
                '=' => {
                    tokens.push(Token::Equal);
                    chars.next();
                }
                ch if ch.is_ascii_alphabetic() => {
                    tokens.push(Token::Unknown(ch.to_string()));
                    chars.next();
                }
                '[' => {
                    // Consume the '['
                    chars.next();
                    let mut unit_string = String::new();

                    // Collect all characters until the closing ']'
                    while let Some(&c) = chars.peek() {
                        if c == ']' {
                            break;
                        }
                        unit_string.push(c);
                        chars.next();
                    }

                    // Consume the closing ']' and panic if not found
                    match chars.next() {
                        Some(']') => tokens.push(Token::Unit(unit_string)),
                        other => panic!("Expected ']' but found {:?}", other),
                    }
                }
                '0'..='9' => {
                    let mut num_string = String::new();

                    // Collect all consecutive numeric characters
                    while let Some(&c) = chars.peek() {
                        if c.is_ascii_digit() {
                            num_string.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    // Parse the numeric literal and push the token
                    let n = num_string.parse().expect("Failed to parse number");
                    tokens.push(Token::NumLiteral(n));
                }
                unexpected => {
                    panic!("unexpected token {:?} encountered", unexpected);
                }
            }
        }

        tokens.push(Token::End);
        tokens
    }
}
