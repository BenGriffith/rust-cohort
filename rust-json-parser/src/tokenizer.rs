use crate::error::JsonError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
}

pub struct Tokenizer {
    input: Vec<char>,
    position: usize,
}

impl Tokenizer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    fn advance(&mut self) -> Option<char> {
        // move forward, return previous char
        let current_char = self.peek();
        self.position += 1;
        current_char
    }

    fn peek(&self) -> Option<char> {
        // look at current char without advancing
        self.input.get(self.position).copied()
    }

    fn is_at_end(&self) -> bool {
        // check if we've consumed all the input
        self.input.is_empty()
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, JsonError> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
            println!("position: {}, ch: {}", self.position, ch);
            match ch {
                '{' => {
                    tokens.push(Token::LeftBrace);
                    self.position += 1;
                }
                '}' => {
                    tokens.push(Token::RightBrace);
                    self.position += 1;
                }
                '[' => {
                    tokens.push(Token::LeftBracket);
                    self.position += 1;
                }
                ']' => {
                    tokens.push(Token::RightBracket);
                    self.position += 1;
                }
                ',' => {
                    tokens.push(Token::Comma);
                    self.position += 1;
                }
                ':' => {
                    tokens.push(Token::Colon);
                    self.position += 1;
                }
                '"' => {
                    self.position += 1;
                    let mut string_value = String::new();
                    while let Some(next_ch) = self.peek() {
                        if next_ch == '"' {
                            self.position += 1;
                            break;
                        } else {
                            string_value.push(self.advance().unwrap()); // FIX THIS UNWRAP
                        }
                    }
                    tokens.push(Token::String(string_value));
                }
                '0'..='9' | '-' => {
                    let mut string_value = String::new();
                    while let Some(next_char) = self.peek() {
                        if next_char.is_ascii_digit() || next_char == '.' || next_char == '-' {
                            string_value.push(self.advance().unwrap()); // FIX THIS UNWRAP
                        } else {
                            break;
                        }
                    }

                    match string_value.parse::<f64>() {
                        Ok(n) => tokens.push(Token::Number(n)),
                        Err(_) => {
                            return Err(JsonError::InvalidNumber {
                                value: string_value,
                                position: self.position,
                            });
                        }
                    }
                }
                _ if ch.is_alphabetic() => {
                    let mut string_value = String::new();
                    while let Some(next_ch) = self.peek() {
                        if next_ch.is_alphabetic() {
                            string_value.push(self.advance().unwrap());
                        } else {
                            break;
                        }
                    }

                    match string_value.as_str() {
                        "null" => tokens.push(Token::Null),
                        "true" => {
                            let bool_string: bool = string_value.parse().unwrap_or_default();
                            tokens.push(Token::Boolean(bool_string));
                        }
                        "false" => {
                            let bool_string: bool = string_value.parse().unwrap_or_default();
                            tokens.push(Token::Boolean(bool_string));
                        }
                        _ => break,
                    }
                }
                _ if ch.is_whitespace() => {
                    println!("Skipped unknown char type: {}", ch);
                    self.position += 1;
                }
                _ => {
                    return Err(JsonError::UnexpectedToken {
                        expected: "valid JSON token".to_string(),
                        found: ch.to_string(),
                        position: self.position,
                    });
                }
            }
        }
        Ok(tokens)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Struct Usage Tests ===

    #[test]
    fn test_tokenizer_struct_creation() {
        let tokenizer = Tokenizer::new(r#""hello""#);
        // Tokenizer should be created without error
        // Internal state is private, so we test via tokenize()
    }

    #[test]
    fn test_tokenizer_multiple_tokens() {
        // Tests that a single tokenize() call handles multiple tokens
        // Note: Unlike Python iterators, calling tokenize() again on the same
        // instance would return empty - the input has been consumed.
        // Create a new Tokenizer instance if you need to parse new input.
        let mut tokenizer = Tokenizer::new("123 456");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens.len(), 2);
    }

    // === Basic Token Tests (from Week 1 - ensure they still pass) ===

    #[test]
    fn test_tokenize_number() {
        let mut tokenizer = Tokenizer::new("42");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(42.0)]);
    }

    #[test]
    fn test_tokenize_negative_number() {
        let mut tokenizer = Tokenizer::new("-3.14");
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::Number(-3.14)]);
    }

    #[test]
    fn test_tokenize_literals() {
        let mut t1 = Tokenizer::new("true");
        assert_eq!(t1.tokenize().unwrap(), vec![Token::Boolean(true)]);

        let mut t2 = Tokenizer::new("false");
        assert_eq!(t2.tokenize().unwrap(), vec![Token::Boolean(false)]);

        let mut t3 = Tokenizer::new("null");
        assert_eq!(t3.tokenize().unwrap(), vec![Token::Null]);
    }

    #[test]
    fn test_tokenize_simple_string() {
        let mut tokenizer = Tokenizer::new(r#""hello""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("hello".to_string())]);
    }
}
