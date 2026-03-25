use rust_json_parser::tokenizer;

fn main() {
    let input_string = r#"{"name": "Alice", "age": 30}"#;
    let tokens = tokenizer::tokenize(input_string);
    println!("{:?}", &tokens);
}
