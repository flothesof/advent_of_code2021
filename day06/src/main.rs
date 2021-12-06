use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut fishs = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let ints: Vec<i32> = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
        for fish in ints {
            fishs.push((fish, 6, 1));
        }
    }
    println!("{:?}", fishs);
    for day in 0..80 {
        let mut existing = Vec::new();
        let mut created = Vec::new();
        for t in &fishs {
            if t.0 == 0 {
                // reset loop for old fish
                existing.push((6, 6, t.2 + 1));
                // add child
                created.push((8, 8, 1)); // new fish
            } else {
                existing.push((t.0 - 1, t.1, t.2));
            }
        }
        fishs = existing;
        for item in created {
            fishs.push(item);
        }
        println!("day: {}, len: {}", day, fishs.len());
    }
}
