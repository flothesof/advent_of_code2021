use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut crabs: Vec<i32> = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        crabs = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    }

    let xmin = crabs.iter().min().unwrap().clone();
    let xmax = crabs.iter().max().unwrap().clone();
    let mut cost: i64 = 100000000000000000;
    for target in xmin..xmax {
        let mut new_cost: i64 = 0;
        for crab in &crabs {
            new_cost += (target as i64 - *crab as i64).abs();
        }
        cost = cmp::min(cost, new_cost);
    }
    println!("part1: {}", cost);
}

fn part2() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut crabs: Vec<i32> = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        crabs = line.split(",").map(|s| s.parse::<i32>().unwrap()).collect();
    }
    let xmin = crabs.iter().min().unwrap().clone();
    let xmax = crabs.iter().max().unwrap().clone();
    let mut cost: i64 = 100000000000000000;
    for target in xmin..xmax {
        let mut new_cost: i64 = 0;
        for crab in &crabs {
            let steps = (target as i64 - *crab as i64).abs();
            new_cost += steps * (steps + 1) / 2;
        }
        cost = cmp::min(cost, new_cost);
    }
    println!("part2: {}", cost);
}

fn main() {
    part1();
    part2();
}
