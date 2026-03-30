use rust_json_parser::Tokenizer;

fn main() {
    let input_string = r#""hello\n"t"#;
    let mut tokens = Tokenizer::new(input_string);
    println!("{:?}", tokens.tokenize());
}
