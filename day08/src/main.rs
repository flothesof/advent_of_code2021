use std::cmp;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let unique: Vec<i32> = vec![2, 3, 4, 7];
    let mut sum = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut line = line.split("|");
        line.next();
        let digits = line.next().unwrap().split(" ");
        for digit in digits {
            if unique.contains(&(digit.to_string().len() as i32)) {
                sum += 1;
            };
        }
    }
    println!("part 1: {}", sum);
}
