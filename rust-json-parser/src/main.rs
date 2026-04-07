use rust_json_parser::JsonParser;
use rust_json_parser::JsonValue;
use rust_json_parser::Result;

fn parse_json(json: &str) -> Result<JsonValue> {
    let mut parser = JsonParser::new(json)?;
    let value = parser.parse()?;
    Ok(value)
}

fn main() {
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

    let value = parse_json(json1).unwrap();
    let name = value.get("name".to_string()).unwrap().as_str().unwrap();
    let features = value
        .get("features".to_string())
        .unwrap()
        .as_array()
        .unwrap();
    let author = value
        .get("metadata".to_string())
        .unwrap()
        .get("author".to_string())
        .unwrap();
    println!("name: {}", name);
    println!("features: {:?}", features);
    println!("author: {}", author);

    // Serialize back to JSON
    println!("{}", value);

    let value2: JsonValue = parse_json(json2).unwrap();
    println!("{}", value2);
}
