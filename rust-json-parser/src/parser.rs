// Week 2: Simple parser for primitive JSON values
use crate::error::JsonError;
use crate::tokenizer::{tokenize, Token};
use crate::value::JsonValue;

// Result type alias for convenience
type Result<T> = std::result::Result<T, JsonError>;

// TODO: Implement your parse_json function
// pub fn parse_json(input: &str) -> Result<JsonValue> {
//     // Your code goes here
//     // Hint:
//     // 1. Call tokenize(input)?  (? propagates errors)
//     // 2. Check if tokens is empty
//     // 3. Match on tokens[0] and convert to JsonValue
// }
pub fn parse_json(input: &str) -> Result<JsonValue> {
    let tokens = tokenize(input)?;
    if tokens.is_empty() {
        let json_error = JsonError::UnexpectedEndOfInput {
            expected: (String::from("JSON value")),
            position: (0),
        };
        Err(json_error)
    } else {
        let token = &tokens[0];
        let value = match token {
            Token::LeftBrace => {
                let left_brace: JsonValue = JsonValue::String('{'.to_string());
                left_brace
            }
            Token::RightBrace => {
                let right_brace: JsonValue = JsonValue::String('}'.to_string());
                right_brace
            }
            Token::LeftBracket => {
                let left_bracket: JsonValue = JsonValue::String('['.to_string());
                left_bracket
            }
            Token::RightBracket => {
                let right_bracket: JsonValue = JsonValue::String(']'.to_string());
                right_bracket
            }
            Token::Comma => {
                let comma: JsonValue = JsonValue::String(','.to_string());
                comma
            }
            Token::Colon => {
                let colon: JsonValue = JsonValue::String(':'.to_string());
                colon
            }
            Token::String(s) => {
                let string_value: JsonValue = JsonValue::String(s.to_string());
                string_value
            }
            Token::Number(n) => {
                let number_value: JsonValue = JsonValue::Number(*n);
                number_value
            }
            Token::Boolean(b) => {
                let bool_value: JsonValue = JsonValue::Boolean(*b);
                bool_value
            }
            Token::Null => JsonValue::Null,
        };
        Ok(value)
    }
}

// Copy these tests as-is:
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_string() {
        let result = parse_json(r#""hello world""#).unwrap();
        assert_eq!(result, JsonValue::String("hello world".to_string()));
    }

    #[test]
    fn test_parse_number() {
        let result = parse_json("42.5").unwrap();
        assert_eq!(result, JsonValue::Number(42.5));

        let result = parse_json("0").unwrap();
        assert_eq!(result, JsonValue::Number(0.0));

        let result = parse_json("-10").unwrap();
        assert_eq!(result, JsonValue::Number(-10.0));
    }

    #[test]
    fn test_parse_boolean() {
        let result = parse_json("true").unwrap();
        assert_eq!(result, JsonValue::Boolean(true));

        let result = parse_json("false").unwrap();
        assert_eq!(result, JsonValue::Boolean(false));
    }

    #[test]
    fn test_parse_null() {
        let result = parse_json("null").unwrap();
        assert_eq!(result, JsonValue::Null);
    }

    #[test]
    fn test_parse_error_empty() {
        let result = parse_json("");
        assert!(result.is_err());

        match result {
            Err(JsonError::UnexpectedEndOfInput { expected, position }) => {
                assert_eq!(expected, "JSON value");
                assert_eq!(position, 0);
            }
            _ => panic!("Expected UnexpectedEndOfInput error"),
        }
    }

    #[test]
    fn test_parse_error_invalid_token() {
        let result = parse_json("@");
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_with_whitespace() {
        let result = parse_json("  42  ").unwrap();
        assert_eq!(result, JsonValue::Number(42.0));

        let result = parse_json("\n\ttrue\n").unwrap();
        assert_eq!(result, JsonValue::Boolean(true));
    }

    #[test]
    fn test_result_pattern_matching() {
        let result = parse_json("42");

        match result {
            Ok(JsonValue::Number(n)) => assert_eq!(n, 42.0),
            _ => panic!("Expected successful number parse"),
        }

        let result = parse_json("@invalid@");

        match result {
            Err(JsonError::UnexpectedToken { .. }) => {} // Expected
            _ => panic!("Expected UnexpectedToken error"),
        }
    }
}
