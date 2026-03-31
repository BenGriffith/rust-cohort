use rust_json_parser::JsonParser;

fn main() {
    let input_string = r#"1 2 3"#;
    let mut parser = JsonParser::new(input_string).unwrap();
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
    println!("{:?}", parser.parse());
}
