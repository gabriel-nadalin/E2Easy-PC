use std::io::{self, Write, BufReader};
use std::fs::File;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// Writes a serializable object to a JSON file.
pub fn write_json_to_file<T: Serialize>(value: &T, path: &str) -> std::io::Result<()> {
    let json = serde_json::to_vec_pretty(value)?;
    let mut file = File::create(path)?;
    file.write_all(&json)?;
    Ok(())
}

pub fn read_json<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let value = serde_json::from_reader(reader)?;
    Ok(value)
}

pub fn request_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    input.trim().to_string()
}