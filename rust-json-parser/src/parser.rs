use crate::error::JsonError;
use crate::tokenizer::{Token, Tokenizer};
use crate::value::JsonValue;

// Result type alias for convenience
type Result<T> = std::result::Result<T, JsonError>;

pub struct JsonParser {
    tokens: Vec<Token>,
    position: usize,
    previous: usize,
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
            previous: 0,
        })
    }

    pub fn parse(&mut self) -> Result<JsonValue> {
        if self.is_at_end() {
            return Err(JsonError::InvalidPosition {
                position: self.position,
            });
        }

        match self.advance() {
            Some(Token::String(s)) => Ok(JsonValue::String(s)),
            Some(Token::Number(n)) => Ok(JsonValue::Number(n)),
            Some(Token::Boolean(b)) => Ok(JsonValue::Boolean(b)),
            Some(Token::Null) => Ok(JsonValue::Null),
            Some(Token::LeftBracket) => {
                let mut json_array: Vec<JsonValue> = vec![];
                while let Some(current_token) = self.advance() {
                    match current_token {
                        Token::String(s) => {
                            json_array.push(JsonValue::String(s));
                            self.advance();
                        }
                        Token::Number(n) => {
                            json_array.push(JsonValue::Number(n));
                            self.advance();
                        }
                        Token::Boolean(b) => {
                            json_array.push(JsonValue::Boolean(b));
                            self.advance();
                        }
                        Token::Null => {
                            json_array.push(JsonValue::Null);
                            self.advance();
                        }
                        Token::Comma => {
                            self.advance();
                        }
                        _ => {
                            break;
                        }
                    }
                }
                Ok(JsonValue::Array(json_array))
            }
            // Some(Token::LeftBracket) => Ok(JsonValue::String("here we go".to_string())),
            other => Err(JsonError::UnexpectedToken {
                expected: "valid JSON token".to_string(),
                found: format!("{:?}", other),
                position: self.previous,
            }),
        }
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position).cloned();
        self.position += 1;
        self.previous = self.position - 1;
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

    #[test]
    fn test_invalid_position() {
        let mut parser = JsonParser::new("1 2").unwrap();
        let json_value_one = parser.parse();
        let json_value_two = parser.parse();
        assert_eq!(json_value_one.unwrap(), JsonValue::Number(1.0));
        assert_eq!(json_value_two.unwrap(), JsonValue::Number(2.0));
        assert!(matches!(
            parser.parse(),
            Err(JsonError::InvalidPosition { .. })
        ));
    }

    mod array_tests {
        use super::*;

        fn parse_json(input: &str) -> Result<JsonValue> {
            let mut parser = JsonParser::new(input)?;
            parser.parse()
        }

        #[test]
        fn test_parse_empty_array() {
            let value = parse_json("[]").unwrap();
            assert_eq!(value, JsonValue::Array(vec![]));
        }

        #[test]
        fn test_parse_array_single() {
            let value = parse_json("[1]").unwrap();
            assert_eq!(value, JsonValue::Array(vec![JsonValue::Number(1.0)]));
        }

        #[test]
        fn test_parse_array_multiple() {
            let value = parse_json("[1, 2, 3]").unwrap();
            let expected = JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
                JsonValue::Number(3.0),
            ]);
            assert_eq!(value, expected);
        }

        #[test]
        fn test_parse_array_mixed_types() {
            let value = parse_json(r#"[1, "two", true, null]"#).unwrap();
            let expected = JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::String("two".to_string()),
                JsonValue::Boolean(true),
                JsonValue::Null,
            ]);
            assert_eq!(value, expected);
        }

        // #[test]
        // fn test_parse_nested_arrays() {
        //     let value = parse_json("[[1, 2], [3, 4]]").unwrap();
        //     let expected = JsonValue::Array(vec![
        //         JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]),
        //         JsonValue::Array(vec![JsonValue::Number(3.0), JsonValue::Number(4.0)]),
        //     ]);
        //     assert_eq!(value, expected);
        // }

        // #[test]
        // fn test_parse_deeply_nested() {
        //     let value = parse_json("[[[1]]]").unwrap();
        //     let expected = JsonValue::Array(vec![JsonValue::Array(vec![JsonValue::Array(vec![
        //         JsonValue::Number(1.0),
        //     ])])]);
        //     assert_eq!(value, expected);
        // }

        #[test]
        fn test_array_accessor() {
            let value = parse_json("[1, 2, 3]").unwrap();
            let arr = value.as_array().unwrap();
            assert_eq!(arr.len(), 3);
        }

        #[test]
        fn test_array_get_index() {
            let value = parse_json("[10, 20, 30]").unwrap();
            assert_eq!(value.get_index(1), Some(&JsonValue::Number(20.0)));
            assert_eq!(value.get_index(5), None);
        }
    }
}
