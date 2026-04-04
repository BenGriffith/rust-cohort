use rust_json_parser::JsonParser;

fn main() {
    let input_string = "1 2 3 false @".to_string();
    let parser = JsonParser::new(&input_string);
    match parser {
        Ok(_) => println!("parser created without error"),
        Err(e) => println!("error: {:?}", e),
    }

    let input_string2 = "4 5 6 true".to_string();
    let input: Vec<&str> = input_string2.split_whitespace().collect();
    let mut parser2 = JsonParser::new(&input_string2).unwrap();
    for _ in input {
        println!("parsed value: {:?}", parser2.parse());
    }
}
