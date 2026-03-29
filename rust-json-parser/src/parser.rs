// Week 2: Simple parser for primitive JSON values
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
}

// pub fn parse_json(input: &str) -> Result<JsonValue> {
//     let json_tokenizer = Tokenizer::new(input);
// let tokens = json_tokenizer.tokenize();
// // let tokens = Tokenizer.tokenize(input)?;
// if tokens.is_empty() {
//     let json_error = JsonError::UnexpectedEndOfInput {
//         expected: (String::from("JSON value")),
//         position: (0),
//     };
//     return Err(json_error);
// }

// match &tokens[0] {
//     Token::String(s) => Ok(JsonValue::String(s.clone())),
//     // Token::Number(n) => Ok(JsonValue::Number(*n)),
//     // Token::Boolean(b) => Ok(JsonValue::Boolean(*b)),
//     Token::Null => Ok(JsonValue::Null),
//     _ => Err(JsonError::UnexpectedToken {
//         expected: "valid JSON token".to_string(),
//         found: format!("{:?}", tokens[0]),
//         position: 0,
//     }),
// }
// }
//
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
}
