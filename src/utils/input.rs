use std::io::{self, Write};

pub fn input(text: &str) -> String {
    let mut input = String::new();
    input.clear();
    print!("  {}", text);
    io::stdout().flush().expect("Failed to flush stdout");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string();
    let res = input.trim_end().len();
    input.truncate(res);
    return input;
}