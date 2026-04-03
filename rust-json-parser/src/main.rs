use rust_json_parser::parse_json;

fn main() {
    let inputs: [String; 3] = [
        "42".to_string(),
        "false".to_string(),
        "@invalid".to_string(),
    ];
    for input in &inputs {
        let result = parse_json(input);
        match result {
            Ok(pass) => println!("parsed value: {:?}", pass),
            Err(fail) => println!("{}", fail),
        }
    }
}
