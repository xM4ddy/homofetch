use serde::Deserialize;
use std::collections::HashMap;
use toml::from_str;
use std::fs;
use std::io;
use std::io::Write;

#[derive(Deserialize, Debug)]
struct Entry {
    #[allow(dead_code)]
    name: String,
    #[allow(dead_code)]
    flag: String,
    #[allow(dead_code)]
    flag_ascii: String,
}

fn main() {
    let toml_str = fs::read_to_string("src/data.toml").expect("Failed to read data.toml file");
    let entries_table: HashMap<String, Vec<Entry>> = from_str(&toml_str).unwrap();

    print!("Enter a Term: ");
    io::stdout().flush().unwrap();

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let data: &[Entry] = &entries_table[&(input_text).trim().to_lowercase()];

    //println!("{:?}", entries_table);
    println!("{:?}", data);
}
