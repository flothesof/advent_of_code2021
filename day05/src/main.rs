use std::cmp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let s: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<i32> = s[0].split(",").map(|s| s.parse().unwrap()).collect();
        let end: Vec<i32> = s[1].split(",").map(|s| s.parse().unwrap()).collect();

        if start[0] == end[0] {
            // vertical line
            //println!("vertical {:?}, {:?}", start, end);
            let x = start[0];
            let ymin = cmp::min(start[1], end[1]);
            let ymax = cmp::max(start[1], end[1]) + 1;
            for y in ymin..ymax {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        } else if start[1] == end[1] {
            // horizontal line
            //println!("horizontal {:?}, {:?}", start, end);
            let y = start[1];
            for x in cmp::min(start[0], end[0])..cmp::max(start[0], end[0]) + 1 {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        }
    }
    let mut sum = 0;
    for val in counts.into_values() {
        if val > 1 {
            sum += 1;
        }
    }
    println!("part 1: {:?}", sum);
}

fn part2() {
    let mut counts: HashMap<(i32, i32), i32> = HashMap::new();
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let s: Vec<&str> = line.split(" -> ").collect();
        let start: Vec<i32> = s[0].split(",").map(|s| s.parse().unwrap()).collect();
        let end: Vec<i32> = s[1].split(",").map(|s| s.parse().unwrap()).collect();

        if start[0] == end[0] {
            // vertical line
            //println!("vertical {:?}, {:?}", start, end);
            let x = start[0];
            let ymin = cmp::min(start[1], end[1]);
            let ymax = cmp::max(start[1], end[1]) + 1;
            for y in ymin..ymax {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        } else if start[1] == end[1] {
            // horizontal line
            //println!("horizontal {:?}, {:?}", start, end);
            let y = start[1];
            for x in cmp::min(start[0], end[0])..cmp::max(start[0], end[0]) + 1 {
                let count = counts.entry((x, y)).or_insert(0);
                *count += 1;
            }
        } else {
            // diagonal
            let dx = end[0] - start[0];
            let dy = end[1] - start[1];
            //println!("{}, {}", dx, dy);
            if dx * dy > 0 {
                let xmin = cmp::min(start[0], end[0]);
                let ymin = cmp::min(start[1], end[1]);
                for i in 0..dx.abs() + 1 {
                    let x = xmin + i;
                    let y = ymin + i;
                    let count = counts.entry((x, y)).or_insert(0);
                    *count += 1;
                }
            } else {
                let xmin = cmp::min(start[0], end[0]);
                let ymin = cmp::max(start[1], end[1]);
                for i in 0..dx.abs() + 1 {
                    let x = xmin + i;
                    let y = ymin - i;
                    let count = counts.entry((x, y)).or_insert(0);
                    *count += 1;
                }
            }
        }
    }
    let mut sum = 0;
    for val in counts.into_values() {
        if val > 1 {
            sum += 1;
        }
    }
    println!("part 2: {:?}", sum);
}

fn main() {
    part1();
    part2();
}
