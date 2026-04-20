//! # JSON Parser
//!
//! A lightweight, hand-rolled JSON parser and tokenizer implemented in Rust.
//! This crate provides a full pipeline for transforming raw JSON strings into a
//! structured 'JsonValue' Abstract Syntax Tree (AST).
//!
//! ## Core Components
//!
//! * **Tokenizer**: Lexes raw input strings into a stream of functional JSON tokens.
//! * **Parser**: Consumes tokens and builds the corresponding `JsonValue` tree,
//!   handling nested objects and arrays.
//! * **Value**: Defines the `JsonValue` enum representing all valid JSON types.
//! * **Error**: Custom error types for robust error handling during lexing and parsing.
//!
//! ## Python Integration
//!
//! This crate includes optional Python bindings via PyO3. When the `python`
//! feature is enabled, the parser can be used directly within a Python environment.
//!
//! ## Example
//!
//! ```rust
//! use rust_json_parser::{JsonParser, JsonValue};
//!
//! let mut parser = JsonParser::new("\"hello\"").unwrap();
//! let value = parser.parse().unwrap();
//! assert_eq!(value, JsonValue::String("hello".to_string()));
//! ```

mod error;
mod parser;
mod tokenizer;
mod value;

pub use error::JsonError;
pub use parser::JsonParser;
pub use tokenizer::{Token, Tokenizer};
pub use value::JsonValue;

pub type Result<T> = std::result::Result<T, JsonError>;

#[cfg(feature = "python")]
mod python_bindings;

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
