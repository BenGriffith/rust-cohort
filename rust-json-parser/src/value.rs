use std::collections::HashMap;
use std::fmt;

/// This enum is the core AST (Abstract Syntax Tree) for the parser, providing a
/// type-safe way to represent and manipulate JSON data structures into Rust.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue {
    /// Represents a JSON `null` literal.
    Null,
    /// Represents a JSON boolean: `true` or `false`.
    Boolean(bool),
    /// Represents a JSON number. Internally stored as `f64`.
    Number(f64),
    /// Represents a JSON string.
    String(String),
    /// Represents a JSON array (an ordered list of values).
    Array(Vec<JsonValue>),
    /// Represents a JSON object (a collection of key/value pairs).
    Object(HashMap<String, JsonValue>),
}

impl JsonValue {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn as_str(&self) -> Option<&str> {
        match self {
            JsonValue::String(s) => Some(s.as_str()),
            _ => None,
        }
    }

    pub fn as_f64(&self) -> Option<f64> {
        match self {
            JsonValue::Number(n) => Some(*n),
            _ => None,
        }
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    // returns the inner vec if this is an array
    pub fn as_array(&self) -> Option<&Vec<JsonValue>> {
        match self {
            JsonValue::Array(a) => Some(a),
            _ => None,
        }
    }

    pub fn as_object(&self) -> Option<&HashMap<String, JsonValue>> {
        match self {
            JsonValue::Object(o) => Some(o),
            _ => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<&JsonValue> {
        match self {
            JsonValue::Object(o) => o.get(key),
            _ => None,
        }
    }

    pub fn get_index(&self, index: usize) -> Option<&JsonValue> {
        match self {
            JsonValue::Array(a) => a.get(index),
            _ => None,
        }
    }

    pub fn escape_string(s: &str) -> String {
        let mut escaped = String::from("\"");
        for c in s.chars() {
            match c {
                '"' => escaped.push_str("\\\""),
                '\\' => escaped.push_str("\\\\"),
                '\x08' => escaped.push_str("\\b"),
                '\x0c' => escaped.push_str("\\f"),
                '\n' => escaped.push_str("\\n"),
                '\r' => escaped.push_str("\\r"),
                '\t' => escaped.push_str("\\t"),
                _ => escaped.push(c),
            }
        }
        escaped.push('"');
        escaped
    }

    pub fn pretty_print(&self, indent: usize) -> String {
        self.pretty_print_recursive(0, indent)
    }

    fn pretty_print_recursive(&self, depth: usize, indent: usize) -> String {
        let current_indent = " ".repeat(depth * indent);
        let next_indent = " ".repeat((depth + 1) * indent);

        match self {
            JsonValue::String(s) => JsonValue::escape_string(s),
            JsonValue::Number(n) => n.to_string(),
            JsonValue::Boolean(b) => b.to_string(),
            JsonValue::Null => "null".to_string(),
            JsonValue::Array(arr) => {
                let mut print_string = String::from("[\n");
                for (i, val) in arr.iter().enumerate() {
                    print_string.push_str(&next_indent);
                    print_string.push_str(&val.pretty_print_recursive(depth + 1, indent));
                    if i < arr.len() - 1 {
                        print_string.push(',');
                    }
                    print_string.push('\n');
                }
                print_string.push_str(&current_indent);
                print_string.push(']');
                print_string
            }
            JsonValue::Object(obj) => {
                let mut print_string = String::from("{\n");
                for (i, (key, value)) in obj.iter().enumerate() {
                    print_string.push_str(&next_indent);
                    print_string.push_str(&format!("\"{}\": ", key));
                    print_string.push_str(&value.pretty_print_recursive(depth + 1, indent));
                    if i < obj.len() - 1 {
                        print_string.push(',');
                    }
                    print_string.push('\n');
                }
                print_string.push_str(&current_indent);
                print_string.push('}');
                print_string
            }
        }
    }
}

impl fmt::Display for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonValue::Null => {
                write!(f, "null")
            }
            JsonValue::Boolean(b) => {
                write!(f, "{}", b)
            }
            JsonValue::Number(n) => {
                write!(f, "{}", n)
            }
            JsonValue::String(s) => {
                write!(f, "{}", JsonValue::escape_string(s))
            }
            JsonValue::Array(v) => {
                write!(f, "[")?;
                for (i, value) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ",")?;
                    }
                    write!(f, "{}", value)?;
                }
                write!(f, "]")
            }
            JsonValue::Object(hm) => {
                write!(f, "{{")?;
                for (i, item) in hm.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}: {}", JsonValue::escape_string(item.0), item.1)?;
                }
                write!(f, "}}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_json_value_creation() {
        let null_val = JsonValue::Null;
        let bool_val = JsonValue::Boolean(true);
        let num_val = JsonValue::Number(42.5);
        let str_val = JsonValue::String("hello".to_string());

        assert!(null_val.is_null());
        assert_eq!(bool_val.as_bool(), Some(true));
        assert_eq!(num_val.as_f64(), Some(42.5));
        assert_eq!(str_val.as_str(), Some("hello"));
    }

    #[test]
    fn test_json_value_accessors() {
        let value = JsonValue::String("test".to_string());
        assert_eq!(value.as_str(), Some("test"));
        assert_eq!(value.as_f64(), None);
        assert_eq!(value.as_bool(), None);
        assert!(!value.is_null());

        let value = JsonValue::Number(42.0);
        assert_eq!(value.as_f64(), Some(42.0));
        assert_eq!(value.as_str(), None);

        let value = JsonValue::Boolean(true);
        assert_eq!(value.as_bool(), Some(true));

        let value = JsonValue::Null;
        assert!(value.is_null());
    }

    #[test]
    fn test_json_value_equality() {
        assert_eq!(JsonValue::Null, JsonValue::Null);
        assert_eq!(JsonValue::Boolean(true), JsonValue::Boolean(true));
        assert_eq!(JsonValue::Number(42.0), JsonValue::Number(42.0));
        assert_eq!(
            JsonValue::String("test".to_string()),
            JsonValue::String("test".to_string())
        );

        assert_ne!(JsonValue::Null, JsonValue::Boolean(false));
        assert_ne!(JsonValue::Number(1.0), JsonValue::Number(2.0));
    }

    mod display_tests {
        use super::*;
        use crate::error::JsonError;
        use crate::parser::JsonParser;

        fn parse_json(input: &str) -> Result<JsonValue, JsonError> {
            let mut parser = JsonParser::new(input)?;
            parser.parse()
        }

        #[test]
        fn test_display_primitives() {
            assert_eq!(JsonValue::Null.to_string(), "null");
            assert_eq!(JsonValue::Boolean(true).to_string(), "true");
            assert_eq!(JsonValue::Boolean(false).to_string(), "false");
            assert_eq!(JsonValue::Number(42.0).to_string(), "42");
            assert_eq!(JsonValue::Number(3.14).to_string(), "3.14");
            assert_eq!(
                JsonValue::String("hello".to_string()).to_string(),
                "\"hello\""
            );
        }

        #[test]
        fn test_display_array() {
            let value = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Number(2.0)]);
            assert_eq!(value.to_string(), "[1,2]");
        }

        #[test]
        fn test_display_empty_containers() {
            assert_eq!(JsonValue::Array(vec![]).to_string(), "[]");
            assert_eq!(JsonValue::Object(HashMap::new()).to_string(), "{}");
        }

        #[test]
        fn test_display_escape_string() {
            let value = JsonValue::String("hello\nworld".to_string());
            assert_eq!(value.to_string(), "\"hello\\nworld\"");
        }

        #[test]
        fn test_display_escape_quotes() {
            let value = JsonValue::String("say \"hi\"".to_string());
            assert_eq!(value.to_string(), "\"say \\\"hi\\\"\"");
        }

        #[test]
        fn test_display_nested() {
            let value = parse_json(r#"{"arr": [1, 2]}"#).unwrap();
            let output = value.to_string();
            // Object key order may vary, so check components
            assert!(output.contains("\"arr\""));
            assert!(output.contains("[1,2]"));
        }

        #[test]
        fn test_display_nested_array() {
            let value = JsonValue::Array(vec![JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::Number(2.0),
            ])]);
            assert_eq!(value.to_string(), "[[1,2]]");
        }
    }
}
