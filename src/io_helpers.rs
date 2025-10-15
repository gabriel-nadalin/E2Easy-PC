use std::io::{self, Write};

pub fn request_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_string()
}