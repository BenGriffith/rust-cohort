mod error;
mod parser;
mod tokenizer;
mod value;

pub use error::JsonError;
pub use parser::JsonParser;
pub use tokenizer::{Token, Tokenizer};
pub use value::JsonValue;

pub type Result<T> = std::result::Result<T, JsonError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        // Test the full parsing pipeline
        let mut parser = JsonParser::new("42").unwrap();
        assert_eq!(parser.parse().unwrap(), JsonValue::Number(42.0));

        parser = JsonParser::new("true").unwrap();
        assert_eq!(parser.parse().unwrap(), JsonValue::Boolean(true));

        parser = JsonParser::new("null").unwrap();
        assert_eq!(parser.parse().unwrap(), JsonValue::Null);

        parser = JsonParser::new(r#""hello""#).unwrap();
        assert_eq!(
            parser.parse().unwrap(),
            JsonValue::String("hello".to_string())
        );
    }
}
