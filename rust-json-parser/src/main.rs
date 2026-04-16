use rust_json_parser::JsonParser;
use rust_json_parser::JsonValue;
use rust_json_parser::Result;

fn parse_json(json: &str) -> Result<JsonValue> {
    let mut parser = JsonParser::new(json)?;
    let value = parser.parse()?;
    Ok(value)
}

fn main() -> Result<()> {
    let json1 = r#"{
    "name": "Rust JSON Parser",
    "version": 1.0,
    "features": ["arrays", "objects", "nesting"],   
    "metadata": {
        "author": "You",
        "complete": true
    }
}"#;

    let json2: &str = r#"
{
    "users": [
        {"id": 1, "name": "Alice", "active": true},
        {"id": 2, "name": "Bob", "active": false}
    ],
    "metadata": {
        "version": "1.0",
        "generated": "2024-01-01"
    }
}"#;

    let value = parse_json(json1)?;

    if let Some(key) = value.get("name")
        && let Some(name) = key.as_str()
    {
        println!("name: {}", name);
    }

    if let Some(key) = value.get("features")
        && let Some(features) = key.as_array()
    {
        println!("features: {:?}", features)
    }

    if let Some(metadata) = value.get("metadata")
        && let Some(author) = metadata.get("author")
        && let Some(value) = author.as_str()
    {
        println!("author: {}", value);
    }

    // Serialize back to JSON
    println!("{}", value);

    let value2: JsonValue = parse_json(json2).unwrap();
    println!("{}", value2);
    Ok(())
}
