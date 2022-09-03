extern crate colorful;

use std::{fs::File, io::{BufReader, BufRead}};
use colorful::{Color, Colorful};
use crate::modules::{http, socks4, socks5};
use std::{thread};

pub fn read_file(path: &str, module: i8) {
    let file = File::open(path).expect("Failed to open file");
    let reader = BufReader::new(file);
    let threads: Vec<_> = reader.lines().map(|line| {
        let line = line.unwrap();
        thread::spawn(move || {
            if module == 1 {
                http::check(&line).map_err(|_e| println!("BAD - {}", line.gradient(Color::Red))).ok();
            } else if module == 2{
                socks4::check(&line).map_err(|_e| println!("BAD - {}", line.gradient(Color::Red))).ok();
            } else if module == 3{
                socks5::check(&line).map_err(|_e| println!("BAD - {}", line.gradient(Color::Red))).ok();
            }
        })
    }).collect();
    for t in threads {
        t.join().unwrap();
    }
}