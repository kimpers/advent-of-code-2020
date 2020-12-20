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

pub fn parse_input_to_character_matrix(input: &str) -> Vec<Vec<String>> {
    input
        .split("\n")
        .map(|row| row.trim())
        .filter(|row| row.len() > 0)
        .map(|line| line.chars().map(|c| c.to_string()).collect())
        .collect::<Vec<Vec<String>>>()
}

pub fn parse_input_to_string_vec(input: &str) -> Vec<String> {
    input
        .split("\n")
        .map(|row| row.trim())
        .filter(|row| row.len() > 0)
        .map(|r| r.to_string())
        .collect::<Vec<String>>()
}
