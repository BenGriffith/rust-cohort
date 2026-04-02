use rust_json_parser::JsonParser;

fn main() {
    let input_string = "1 2 3 false".to_string();
    let input: Vec<&str> = input_string.split_whitespace().collect();

    let mut parser = JsonParser::new(&input_string).unwrap();
    for _ in input {
        println!("parsed value: {:?}", parser.parse());
    }
}
