use rust_json_parser::JsonParser;
use rust_json_parser::JsonValue;
use rust_json_parser::Result;

fn parse_json(json: &str) -> Result<JsonValue> {
    let mut parser = JsonParser::new(json)?;
    let value = parser.parse()?;
    Ok(value)
}

fn main() {
    let json = r#"{
    "name": "Rust JSON Parser",
    "version": 1.0,
    "features": ["arrays", "objects", "nesting"],   
    "metadata": {
        "author": "You",
        "complete": true
    }
}"#;

    let value = parse_json(json).unwrap();
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
}
