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
    let scores = HashMap::from([(")", 3), ("]", 57), ("}", 1197), (">", 25137)]);
    let mut stack = Vec::new();
    for char in line.chars() {
        if open_symbols.contains(&char.to_string()) {
            stack.push(char.to_string().clone())
        } else if close_symbols.contains(&char.to_string()) {
            let pos = close_symbols
                .iter()
                .position(|s| s == &char.to_string())
                .unwrap();
            let open = open_symbols[pos].clone();
            let last = stack[stack.len() - 1].clone();
            if open != *last {
                let s = char.to_string();
                let s_slice: &str = &*s;
                return *scores.get(s_slice).unwrap();
            } else {
                stack.pop();
            }
        }
    }
    0
}

fn part1() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut score = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        score += parse(&line);
    }
    println!("part 1: {}", score);
}

fn parse_part2(line: &String) -> i64 {
    let open_symbols: Vec<String> = vec!["(", "[", "{", "<"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let close_symbols: Vec<String> = vec![")", "]", "}", ">"]
        .iter()
        .map(|s| s.to_string())
        .collect();
    let scores = HashMap::from([("(", 1), ("[", 2), ("{", 3), ("<", 4)]);
    let mut stack = Vec::new();
    for char in line.chars() {
        if open_symbols.contains(&char.to_string()) {
            stack.push(char.to_string().clone())
        } else if close_symbols.contains(&char.to_string()) {
            let pos = close_symbols
                .iter()
                .position(|s| s == &char.to_string())
                .unwrap();
            let open = open_symbols[pos].clone();
            let last = stack[stack.len() - 1].clone();
            if open != *last {
                return 0;
            } else {
                stack.pop();
            }
        }
    }
    // unwrap stack
    let mut score: i64 = 0;
    while stack.len() > 0 {
        let item = stack.pop();
        score *= 5;
        let s_slice: &str = &item.unwrap();
        score += scores[s_slice];
    }
    score
}

fn part2() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut scores = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let subscore = parse_part2(&line);
        if subscore > 0 {
            scores.push(subscore);
        }
    }
    scores.sort();
    println!("part 2: {}", scores[(scores.len() - 1) / 2]);
}

fn main() {
    part1();
    part2();
}
