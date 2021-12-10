use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse(line: &String) -> i32 {
    let open_symbols: Vec<String> = vec!["(", "[", "{", "<"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let close_symbols: Vec<String> = vec![")", "]", "}", ">"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let mut state = HashMap::new();
    let mut error: bool = false;
    for char in line.chars() {
        if open_symbols.contains(&char.to_string()) {
            let c = state.entry(char.to_string()).or_insert(0);
            *c += 1
        } else if close_symbols.contains(&char.to_string()) {
            let c = state.entry(char.to_string()).or_insert(0);
            if *c - 1 < 0 {
                error = true;
                break;
            }
        }
    }
    if error
    0
}

fn main() {
    let f = File::open("input_test").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut score = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        score += parse(&line);
    }
    println!("part 1: {}", score);
}
