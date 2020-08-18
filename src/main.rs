use json_rs::lexer;
use std::env;
use std::process::exit;

fn main() {
    match env::args().into_iter().nth(1) {
        Some(raw) => {
            let mut lexer = lexer::Lexer::new(raw.to_string());

            while !lexer.is_eof() {
                match lexer.next_token() {
                    Ok(tok) => println!("{:?}", tok),
                    Err(e) => exit(1),
                }
            }
        }
        None => {
            eprintln!("cmd <args>");
            exit(1);
        }
    }
}
