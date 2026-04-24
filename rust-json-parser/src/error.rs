use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    UnexpectedToken {
        expected: String,
        found: String,
        position: usize,
    },
    UnexpectedEndOfInput {
        expected: String,
        position: usize,
    },
    InvalidNumber {
        value: String,
        position: usize,
    },
    InvalidEscape {
        char: char,
        position: usize,
    },
    InvalidUnicode {
        sequence: String,
        position: usize,
    },
    InvalidPosition {
        position: usize,
    },
    ExpectedColon {
        position: usize,
    },
    IOError {
        message: String,
    },
    FileNotFound {
        path: String,
    },
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
