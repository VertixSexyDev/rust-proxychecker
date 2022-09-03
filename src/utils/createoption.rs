extern crate colorful;

use colorful::{Color, Colorful};

pub fn createoption(num: i32, text: &str) {
    let num = num.to_string();
    print!("  [{}] - {}", num.gradient(Color::Green), text);
    println!();
}