use crate::Result;
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

    pub fn tokenize(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while let Some(ch) = self.peek() {
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
                        match next_ch {
                            '"' => {
                                self.position += 1;
                                break;
                            }
                            '\\' => {
                                string_value = self.parse_escape_seq(string_value)?;
                            }
                            _ => {
                                if let Some(c) = self.advance() {
                                    string_value.push(c);
                                }
                            }
                        }
                        if self.is_at_end() {
                            return Err(JsonError::UnexpectedEndOfInput {
                                expected: '"'.to_string(),
                                position: self.position,
                            });
                        }
                    }
                    tokens.push(Token::String(string_value));
                }
                '0'..='9' | '-' => {
                    let num = self.parse_number()?;
                    tokens.push(Token::Number(num));
                }
                _ if ch.is_alphabetic() => {
                    let keyword = self.parse_keyword()?;
                    tokens.push(keyword);
                }
                _ if ch.is_whitespace() => {
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

    fn parse_escape_seq(&mut self, mut string_value: String) -> Result<String> {
        self.advance();
        match self.advance() {
            Some('"') => string_value.push('"'),
            Some('\\') => string_value.push('\\'),
            Some('/') => string_value.push('/'),
            Some('b') => string_value.push('\x08'),
            Some('f') => string_value.push('\x0C'),
            Some('n') => string_value.push('\n'),
            Some('r') => string_value.push('\r'),
            Some('t') => string_value.push('\t'),
            Some('u') => {
                let mut hex_string = String::new();

                while !self.is_at_end() && hex_string.len() <= 3 {
                    if let Some(c) = self.advance()
                        && c.is_ascii_hexdigit()
                    {
                        hex_string.push(c);
                    }
                }

                if hex_string.len() < 4 {
                    return Err(JsonError::InvalidUnicode {
                        sequence: hex_string,
                        position: self.position,
                    });
                }

                let code_point = u32::from_str_radix(&hex_string, 16);
                match code_point {
                    Ok(n) => {
                        let hex_digit = char::from_u32(n);
                        if let Some(n) = hex_digit {
                            string_value.push(n);
                        }
                    }
                    Err(_) => {
                        return Err(JsonError::InvalidUnicode {
                            sequence: hex_string,
                            position: self.position,
                        });
                    }
                }
            }
            other => {
                return Err(JsonError::InvalidEscape {
                    char: other.unwrap_or('?'),
                    position: self.position,
                });
            }
        }

        Ok(string_value)
    }

    fn parse_number(&mut self) -> Result<f64> {
        let mut string_value = String::new();
        while let Some(next_char) = self.peek() {
            let valid_char: bool =
                next_char.is_ascii_digit() || next_char == '.' || next_char == '-';
            match valid_char {
                true => {
                    if let Some(c) = self.advance() {
                        string_value.push(c);
                    }
                }
                _ => {
                    break;
                }
            }
        }

        match string_value.parse::<f64>() {
            Ok(n) => Ok(n),
            Err(_) => Err(JsonError::InvalidNumber {
                value: string_value,
                position: self.position,
            }),
        }
    }

    fn parse_keyword(&mut self) -> Result<Token> {
        let mut string_value = String::new();
        while let Some(next_ch) = self.peek() {
            match next_ch.is_alphabetic() {
                true => {
                    if let Some(c) = self.advance() {
                        string_value.push(c);
                    }
                }
                _ => {
                    break;
                }
            }
        }
        match string_value.as_str() {
            "null" => Ok(Token::Null),
            "true" => Ok(Token::Boolean(true)),
            "false" => Ok(Token::Boolean(false)),
            _ => Err(JsonError::UnexpectedToken {
                expected: r#""null", "true", or "false""#.to_string(),
                found: string_value,
                position: self.position,
            }),
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
        self.position >= self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::JsonError;

    // Result type alias for cleaner test signatures
    type Result<T> = std::result::Result<T, JsonError>;

    #[test]
    fn test_empty_braces() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new("{}");
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 2);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::RightBrace);
        Ok(())
    }

    #[test]
    fn test_simple_string() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new(r#""hello""#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello".to_string()));
        Ok(())
    }

    #[test]
    fn test_number() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new("42");
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(42.0));
        Ok(())
    }

    #[test]
    fn test_tokenize_string() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new(r#""hello world""#);
        let tokens = json_tokenizer.tokenize()?;

        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("hello world".to_string()));
        Ok(())
    }

    #[test]
    fn test_boolean_and_null() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new("true false null");
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0], Token::Boolean(true));
        assert_eq!(tokens[1], Token::Boolean(false));
        assert_eq!(tokens[2], Token::Null);
        Ok(())
    }

    #[test]
    fn test_simple_object() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new(r#"{"name": "Alice"}"#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBrace);
        assert_eq!(tokens[1], Token::String("name".to_string()));
        assert_eq!(tokens[2], Token::Colon);
        assert_eq!(tokens[3], Token::String("Alice".to_string()));
        assert_eq!(tokens[4], Token::RightBrace);
        Ok(())
    }

    #[test]
    fn test_multiple_values() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new(r#"{"age": 30, "active": true}"#);
        let tokens = json_tokenizer.tokenize()?;

        // Verify we have the right tokens
        assert!(tokens.contains(&Token::String("age".to_string())));
        assert!(tokens.contains(&Token::Number(30.0)));
        assert!(tokens.contains(&Token::Comma));
        assert!(tokens.contains(&Token::String("active".to_string())));
        assert!(tokens.contains(&Token::Boolean(true)));
        Ok(())
    }

    // String boundary tests - verify inner vs outer quote handling
    #[test]
    fn test_empty_string() -> Result<()> {
        // Outer boundary: adjacent quotes with no inner content
        let mut json_tokenizer = Tokenizer::new(r#""""#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_containing_json_special_chars() -> Result<()> {
        // Inner handling: JSON delimiters inside strings don't break tokenization
        let mut json_tokenizer = Tokenizer::new(r#""{key: value}""#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("{key: value}".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_keyword_like_content() -> Result<()> {
        // Inner handling: "true", "false", "null" inside strings stay as string content
        let mut json_tokenizer = Tokenizer::new(r#""not true or false""#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_number_like_content() -> Result<()> {
        // Inner handling: numeric content inside strings doesn't become Number tokens
        let mut json_tokenizer = Tokenizer::new(r#""phone: 555-1234""#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("phone: 555-1234".to_string()));
        Ok(())
    }

    // Number parsing tests
    #[test]
    fn test_negative_number() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new("-42");
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }

    #[test]
    fn test_decimal_number() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new("0.5");
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(0.5));
        Ok(())
    }

    #[test]
    fn test_leading_decimal_not_a_number() -> Result<()> {
        // .5 is invalid JSON - numbers must have leading digit (0.5 is valid)
        let mut json_tokenizer = Tokenizer::new(".5");
        let tokens = json_tokenizer.tokenize();
        // Should NOT be interpreted as 0.5
        assert!(tokens.is_err());
        Ok(())
    }

    #[test]
    fn test_array_numbers() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new("[1, 2, 3, -10]");
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 9);
        Ok(())
    }

    // Error position tests
    #[test]
    fn test_array_strings() -> Result<()> {
        let mut json_tokenizer = Tokenizer::new(r#"["hello", "world"]"#);
        let tokens = json_tokenizer.tokenize()?;
        assert_eq!(tokens.len(), 5);
        assert_eq!(tokens[0], Token::LeftBracket);
        assert_eq!(tokens[1], Token::String("hello".to_string()));
        assert!(tokens.contains(&Token::Comma));
        Ok(())
    }

    #[test]
    fn test_invalid_keyword_error_position_points_to_start() {
        let input = "   @yz";
        let mut json_tokenizer = Tokenizer::new(input);
        let result = json_tokenizer.tokenize();
        assert!(result.is_err());
        if let Err(JsonError::UnexpectedToken { position, .. }) = result {
            assert_eq!(
                position, 3,
                "error position should point to the start of 'xyz' (index 3), not past it"
            );
        } else {
            panic!("expected UnexpectedToken error");
        }
    }

    // === Struct Usage Tests ===
    #[test]
    fn test_tokenizer_struct_creation() {
        let mut tokenizer = Tokenizer::new(r#""hello""#);
        // Tokenizer should be created without error
        // Internal state is private, so we test via tokenize()
        assert!(tokenizer.tokenize().is_ok());
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

    // === Escape Sequence Tests ===

    #[test]
    fn test_escape_newline() {
        let mut tokenizer = Tokenizer::new(r#""hello\nworld""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("hello\nworld".to_string())]);
    }

    #[test]
    fn test_escape_tab() {
        let mut tokenizer = Tokenizer::new(r#""col1\tcol2""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("col1\tcol2".to_string())]);
    }

    #[test]
    fn test_escape_quote() {
        let mut tokenizer = Tokenizer::new(r#""say \"hello\"""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("say \"hello\"".to_string())]);
    }

    #[test]
    fn test_escape_backslash() {
        let mut tokenizer = Tokenizer::new(r#""path\\to\\file""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("path\\to\\file".to_string())]);
    }

    #[test]
    fn test_escape_forward_slash() {
        let mut tokenizer = Tokenizer::new(r#""a\/b""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("a/b".to_string())]);
    }

    #[test]
    fn test_escape_carriage_return() {
        let mut tokenizer = Tokenizer::new(r#""line\r\n""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("line\r\n".to_string())]);
    }

    #[test]
    fn test_escape_backspace_formfeed() {
        let mut tokenizer = Tokenizer::new(r#""\b\f""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("\u{0008}\u{000C}".to_string())]);
    }

    #[test]
    fn test_multiple_escapes() {
        let mut tokenizer = Tokenizer::new(r#""a\nb\tc\"""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("a\nb\tc\"".to_string())]);
    }

    // === Unicode Escape Tests ===

    #[test]
    fn test_unicode_escape_basic() {
        // \u0041 is 'A'
        let mut tokenizer = Tokenizer::new(r#""\u0041""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("A".to_string())]);
    }

    #[test]
    fn test_unicode_escape_multiple() {
        // \u0048\u0069 is "Hi"
        let mut tokenizer = Tokenizer::new(r#""\u0048\u0069""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("Hi".to_string())]);
    }

    #[test]
    fn test_unicode_escape_mixed() {
        // Mix of regular chars and unicode escapes
        let mut tokenizer = Tokenizer::new(r#""Hello \u0057orld""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("Hello World".to_string())]);
    }

    #[test]
    fn test_unicode_escape_lowercase() {
        // Lowercase hex digits should work too
        let mut tokenizer = Tokenizer::new(r#""\u004a""#);
        let tokens = tokenizer.tokenize().unwrap();
        assert_eq!(tokens, vec![Token::String("J".to_string())]);
    }

    // === Error Tests ===

    #[test]
    fn test_invalid_escape_sequence() {
        let mut tokenizer = Tokenizer::new(r#""\q""#);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(JsonError::InvalidEscape { .. })));
    }

    #[test]
    fn test_invalid_unicode_too_short() {
        let mut tokenizer = Tokenizer::new(r#""\u004""#);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_invalid_unicode_bad_hex() {
        let mut tokenizer = Tokenizer::new(r#""\u00GG""#);
        let result = tokenizer.tokenize();
        assert!(matches!(result, Err(JsonError::InvalidUnicode { .. })));
    }

    #[test]
    fn test_unterminated_string_with_escape() {
        let mut tokenizer = Tokenizer::new(r#""hello\n"#);
        let result = tokenizer.tokenize();
        assert!(result.is_err());
    }
}
