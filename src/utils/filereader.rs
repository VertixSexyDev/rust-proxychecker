extern crate colorful;

use std::{fs::File, io::{BufReader, BufRead}, path::Path};
use colorful::{Color, Colorful};
use crate::modules::{http, socks4, socks5};
use std::{thread};

fn get_file_path(path: &Path) -> impl BufRead {
    BufReader::new(File::open(path).unwrap())
}

pub fn read_file(path: &str, module: i8) {
    let reader = get_file_path(Path::new(path));
    let total = get_file_path(Path::new(path)).lines().count();
    println!("{} {}", "Total proxies: ".gradient(Color::Green), total);
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