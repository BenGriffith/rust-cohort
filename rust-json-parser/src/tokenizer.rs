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

pub fn tokenize(input: &str) -> Result<Vec<Token>, JsonError> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some(&(pos, ch)) = chars.peek() {
        println!("{:?}", ch);
        match ch {
            '{' => {
                tokens.push(Token::LeftBrace);
                chars.next();
            }
            '}' => {
                tokens.push(Token::RightBrace);
                chars.next();
            }
            '[' => {
                tokens.push(Token::LeftBracket);
                chars.next();
            }
            ']' => {
                tokens.push(Token::RightBracket);
                chars.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                chars.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                chars.next();
            }
            '"' => {
                chars.next();
                let string_value: String = chars
                    .by_ref()
                    .take_while(|&(_, c)| c != '"')
                    .map(|(_, c)| c)
                    .collect();
                tokens.push(Token::String(string_value));
            }
            '0'..='9' | '-' => {
                let mut string_value = String::new();
                while let Some(&(_, next_char)) = chars.peek() {
                    if next_char.is_ascii_digit() || next_char == '.' || next_char == '-' {
                        string_value.push(next_char);
                        chars.next();
                    } else {
                        break;
                    }
                }

                match string_value.parse::<f64>() {
                    Ok(n) => tokens.push(Token::Number(n)),
                    Err(_) => {
                        return Err(JsonError::InvalidNumber {
                            value: string_value,
                            position: pos,
                        });
                    }
                }
            }
            _ if ch.is_alphabetic() => {
                let string_value: String = chars
                    .by_ref()
                    .take_while(|&(_, c)| c.is_alphabetic())
                    .map(|(_, c)| c)
                    .collect();

                match string_value.as_str() {
                    "null" => tokens.push(Token::Null),
                    "true" => {
                        let bool_string: bool = string_value.parse().unwrap_or_default();
                        tokens.push(Token::Boolean(bool_string));
                    }
                    "false" => {
                        let bool_string: bool = string_value.parse().unwrap_or_default();
                        tokens.push(Token::Boolean(bool_string));
                    }
                    _ => break,
                }
            }
            _ if ch.is_whitespace() => {
                println!("Skipped unknown char type: {}", ch);
                chars.next();
            }
            _ => {
                println!("{:?}", pos);
                return Err(JsonError::UnexpectedToken {
                    expected: "valid JSON token".to_string(),
                    found: ch.to_string(),
                    position: pos,
                });
            }
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::JsonError;

    // Result type alias for cleaner test signatures
    type Result<T> = std::result::Result<T, JsonError>;

    // String boundary tests - verify inner vs outer quote handling
    #[test]
    fn test_empty_string() -> Result<()> {
        // Outer boundary: adjacent quotes with no inner content
        let tokens = tokenize(r#""""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_containing_json_special_chars() -> Result<()> {
        // Inner handling: JSON delimiters inside strings don't break tokenization
        let tokens = tokenize(r#""{key: value}""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("{key: value}".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_keyword_like_content() -> Result<()> {
        // Inner handling: "true", "false", "null" inside strings stay as string content
        let tokens = tokenize(r#""not true or false""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("not true or false".to_string()));
        Ok(())
    }

    #[test]
    fn test_string_with_number_like_content() -> Result<()> {
        // Inner handling: numeric content inside strings doesn't become Number tokens
        let tokens = tokenize(r#""phone: 555-1234""#)?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::String("phone: 555-1234".to_string()));
        Ok(())
    }

    // Number parsing tests
    #[test]
    fn test_negative_number() -> Result<()> {
        let tokens = tokenize("-42")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(-42.0));
        Ok(())
    }

    #[test]
    fn test_decimal_number() -> Result<()> {
        let tokens = tokenize("0.5")?;
        assert_eq!(tokens.len(), 1);
        assert_eq!(tokens[0], Token::Number(0.5));
        Ok(())
    }

    #[test]
    fn test_leading_decimal_not_a_number() -> Result<()> {
        // .5 is invalid JSON - numbers must have leading digit (0.5 is valid)
        let tokens = tokenize(".5");
        // Should NOT be interpreted as 0.5
        assert!(tokens.is_err());
        Ok(())
    }

    // Error position tests

    #[test]
    fn test_invalid_keyword_error_position_points_to_start() {
        let input = "   @yz";
        let result = tokenize(input);
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
}
