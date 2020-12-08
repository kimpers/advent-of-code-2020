use std::fs::File;
use std::io::prelude::*;

pub fn read_file(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).unwrap();
    let mut contents = String::new();

    file.read_to_string(&mut contents).unwrap();
    let lines = contents
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();

    lines
}
