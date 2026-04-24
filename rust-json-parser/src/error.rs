use std::fmt;

/// Errors that can occur during the lexical analysis or parsing of JSON data.
///
/// This enum categorizes all possible failure points, from malformed strings to
/// underlying I/O issues. Each variant provides the `position` in the input
/// string to facilitate debugging.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    /// Occurs when a token is found that does not match the expected JSON grammar.
    ///
    /// **Example:** Finding a comma where a colon was expected in an object.
    UnexpectedToken {
        /// A description of the expected token.
        expected: String,
        /// The actual token string found in the input.
        found: String,
        /// Byte offset where the error occurred.
        position: usize,
    },
    /// Occurs when the input stream ends prematurely.
    ///
    /// **Example:** An unclosed array `[1, 2`.
    UnexpectedEndOfInput { expected: String, position: usize },
    /// Occurs when a numeric literal is invalid.
    ///
    /// **Example:** `0123` (no leading zeros) or `1.2.3`.
    InvalidNumber { value: String, position: usize },
    /// Occurs when an invalid escape sequence is encountered within a string.
    ///
    /// Valid escapes are `"`, `\\`, `\/`, `\b`, `\f`, `\n`, `\r`, and `\t`.
    InvalidEscape { char: char, position: usize },
    /// Occurs when a `\uXXXX` escape sequence contains non-hexadecimal characters.
    InvalidUnicode { sequence: String, position: usize },
    /// A generic error for when a parser action is attempted at an out-of-bounds position.
    InvalidPosition { position: usize },
    /// A specific error for missing colons between keys and values in objects.
    ExpectedColon { position: usize },
    /// A wrapper for general Input/Output failures.
    IOError { message: String },
    /// Occurs when a specified JSON file cannot be located on the filesystem.
    FileNotFound { path: String },
}

impl fmt::Display for JsonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            JsonError::UnexpectedToken {
                expected,
                found,
                position,
            } => {
                write!(
                    f,
                    "Unexpected token at position {}: expected {}, found {}",
                    position, expected, found
                )
            }
            JsonError::UnexpectedEndOfInput { expected, position } => {
                write!(
                    f,
                    "Unexpected end of input at position {}: expected {}",
                    position, expected
                )
            }
            JsonError::InvalidNumber { value, position } => {
                write!(f, "Invalid number: {} at position {}:", value, position)
            }
            JsonError::InvalidEscape { char, position } => {
                write!(
                    f,
                    "Invalid escape character: {} at position {}:",
                    char, position
                )
            }
            JsonError::InvalidUnicode { sequence, position } => {
                write!(f, "Invalid unicode: {} at position {}:", sequence, position)
            }
            JsonError::InvalidPosition { position } => {
                write!(f, "Invalid position: {position}")
            }
            JsonError::ExpectedColon { position } => {
                write!(f, "Expected Colon at position: {}", position)
            }
            JsonError::IOError { message } => {
                write!(f, "{}", message)
            }
            JsonError::FileNotFound { path } => {
                write!(f, "File not found: {}", path)
            }
        }
    }
}

impl std::error::Error for JsonError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_escape_display() {
        let err = JsonError::InvalidEscape {
            char: 'q',
            position: 5,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("escape"));
        assert!(msg.contains("q"));
    }

    #[test]
    fn test_invalid_unicode_display() {
        let err = JsonError::InvalidUnicode {
            sequence: "00GG".to_string(),
            position: 3,
        };
        let msg = format!("{}", err);
        assert!(msg.contains("unicode") || msg.contains("Unicode"));
    }

    #[test]
    fn test_error_is_std_error() {
        let err = JsonError::InvalidEscape {
            char: 'x',
            position: 0,
        };
        let _: &dyn std::error::Error = &err; // Must implement Error trait
    }
}
