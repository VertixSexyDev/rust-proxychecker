extern crate colorful;

use colorful::{Color, Colorful};

pub fn logo() {
    println!("{}", r"  ____   ____             __  .__                ".gradient(Color::Green));
    println!("{}", r"  \   \ /   /____________/  |_|__|__  ______  ___".gradient(Color::Green));
    println!("{}", r"   \   Y   // __ \_  __ \   __\  \  \/  /\  \/  /".gradient(Color::Green));
    println!("{}", r"    \     /\  ___/|  | \/|  | |  |>    <  >    < ".gradient(Color::Green));
    println!("{}", r"     \___/  \___  >__|   |__| |__/__/\_ \/__/\_ \".gradient(Color::Green));
    println!("{}", r"                \/                     \/      \/".gradient(Color::Green));
}