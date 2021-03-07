use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() {
    let f = File::open("cargo.toml").unwrap();

    let reader = BufReader::new(f);

    for line_ in reader.lines() {
        let line = line_.unwrap();
        println!("{} ({} bytes long)", line, line.len());
    }
}
