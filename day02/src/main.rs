/*
Notes:
- reading a string can be simple actually, and doesn’t need to use buffers
https://stackoverflow.com/questions/31192956/whats-the-de-facto-way-of-reading-and-writing-files-in-rust-1-x
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    part1();
    part2();
}

fn part2() {
    let f = File::open("./input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let vec = line.split(" ").collect::<Vec<&str>>();
        let val = vec[1].parse::<i32>().unwrap();
        if vec[0] == "forward" {
            horizontal += val;
            depth += aim * val;
        } else if vec[0] == "up" {
            aim -= val;
        } else if vec[0] == "down" {
            aim += val;
        }
    }

    println!(
        "part 2: horizontal {}, depth {}, score {}",
        horizontal,
        depth,
        horizontal * depth
    );
}

fn part1() {
    let f = File::open("./input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut depth = 0;
    let mut horizontal = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let vec = line.split(" ").collect::<Vec<&str>>();
        // println!("{}", vec[0]);
        let val = vec[1].parse::<i32>().unwrap();
        if vec[0] == "forward" {
            horizontal += val;
        } else if vec[0] == "up" {
            depth -= val;
        } else if vec[0] == "down" {
            depth += val;
        }
    }
    println!(
        "part 1: horizontal {}, depth {}, score {}",
        horizontal,
        depth,
        horizontal * depth
    );
}
