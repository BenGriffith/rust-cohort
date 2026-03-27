use rust_json_parser::tokenize;

fn main() {
    let input_string = r#"{"name": "Alice", "age": 30}"#;
    let tokens = tokenize(input_string);
    println!("{:?}", &tokens);
}
