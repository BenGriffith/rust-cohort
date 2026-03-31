use crate::error::JsonError;
use crate::tokenizer::{Token, Tokenizer};
use crate::value::JsonValue;

// Result type alias for convenience
type Result<T> = std::result::Result<T, JsonError>;

pub struct JsonParser {
    tokens: Vec<Token>,
    position: usize,
}

impl JsonParser {
    pub fn new(input: &str) -> Result<Self> {
        let mut json_tokenizer = Tokenizer::new(input);
        let json_tokens = json_tokenizer.tokenize()?;
        if json_tokens.is_empty() {
            return Err(JsonError::UnexpectedEndOfInput {
                expected: "JSON value".to_string(),
                position: 0,
            });
        }
        Ok(Self {
            tokens: json_tokens,
            position: 0,
        })
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        if self.is_at_end() {
            return Err(JsonError::UnexpectedEndOfInput {
                expected: "test".to_string(),
                position: self.position,
            });
        }

        match self.advance().unwrap() {
            Token::String(s) => Ok(JsonValue::String(s.clone())),
            Token::Number(n) => Ok(JsonValue::Number(n)),
            Token::Boolean(b) => Ok(JsonValue::Boolean(b)),
            Token::Null => Ok(JsonValue::Null),
            _ => Err(JsonError::UnexpectedToken {
                expected: "valid JSON token".to_string(),
                found: format!("{:?}", self.tokens[self.position]),
                position: self.position,
            }),
        }
    }

    // switch these around so that this advance looks similar to tokenizer.advance()
    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        token
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.tokens.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // === Struct Usage Tests ===

    #[test]
    fn test_parser_creation() {
        let parser = JsonParser::new("42");
        assert!(parser.is_ok());
    }

    #[test]
    fn test_parser_creation_tokenize_error() {
        let parser = JsonParser::new(r#""\q""#); // Invalid escape
        assert!(parser.is_err());
    }

    #[test]
    fn test_parse_number() {
        let mut parser = JsonParser::new("42").unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Number(42.0));
    }

    #[test]
    fn test_parse_negative_number() {
        let mut parser = JsonParser::new("-3.14").unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Number(-3.14));
    }

    #[test]
    fn test_parse_boolean_true() {
        let mut parser = JsonParser::new("true").unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Boolean(true));
    }

    #[test]
    fn test_parse_boolean_false() {
        let mut parser = JsonParser::new("false").unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_null() {
        let mut parser = JsonParser::new("null").unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::Null);
    }

    #[test]
    fn test_parse_simple_string() {
        let mut parser = JsonParser::new(r#""hello""#).unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::String("hello".to_string()));
    }

    // === Escape Sequence Integration Tests ===

    #[test]
    fn test_parse_string_with_newline() {
        let mut parser = JsonParser::new(r#""hello\nworld""#).unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::String("hello\nworld".to_string()));
    }

    #[test]
    fn test_parse_string_with_tab() {
        let mut parser = JsonParser::new(r#""col1\tcol2""#).unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::String("col1\tcol2".to_string()));
    }

    #[test]
    fn test_parse_string_with_quotes() {
        let mut parser = JsonParser::new(r#""say \"hi\"""#).unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::String("say \"hi\"".to_string()));
    }

    #[test]
    fn test_parse_string_with_unicode() {
        let mut parser = JsonParser::new(r#""\u0048\u0065\u006c\u006c\u006f""#).unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(value, JsonValue::String("Hello".to_string()));
    }

    #[test]
    fn test_parse_complex_escapes() {
        let mut parser = JsonParser::new(r#""line1\nline2\t\"quoted\"\u0021""#).unwrap();
        let value = parser.parse().unwrap();
        assert_eq!(
            value,
            JsonValue::String("line1\nline2\t\"quoted\"!".to_string())
        );
    }

    // === Error Tests ===

    #[test]
    fn test_parse_empty_input() {
        let parser = JsonParser::new("");
        // Could fail at tokenization (no tokens) or parsing (empty token list)
        // Either is acceptable - just verify it's an error
        assert!(parser.is_err() || parser.unwrap().parse().is_err());
    }

    #[test]
    fn test_parse_whitespace_only() {
        let parser = JsonParser::new("   ");
        assert!(parser.is_err() || parser.unwrap().parse().is_err());
    }
}
