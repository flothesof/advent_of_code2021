/*
- Cannot use tuple for counts!
- need to add .expect to get rid of error handling!
- there are two string types and we always need to convert back and forth, very hard for me
- functions need to be annotated for returns, else they return an empty tuple
- borrowing, note the &
    // This function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

*/

use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(fname: &String) -> std::vec::Vec<String> {
    let f = File::open(fname).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut lines = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        lines.push(line);
    }
    return lines;
}

fn make_counts(lines: &std::vec::Vec<String>) -> [std::vec::Vec<i32>; 2] {
    let mut counts_zero = Vec::new();
    let mut counts_one = Vec::new();
    for _i in 0..lines[0].len() {
        counts_zero.push(0);
        counts_one.push(0);
    }
    for line in lines {
        for i in 0..line.len() {
            let j = line.chars().nth(i).unwrap();
            if j.to_string() == "0" {
                counts_zero[i] += 1;
            } else if j.to_string() == "1" {
                counts_one[i] += 1;
            }
        }
    }
    return [counts_zero, counts_one];
}

fn part1() {
    let lines = read_lines(&"./input".to_string());
    // println!("{}, {}", lines.len(), lines[0].len());
    let [counts_zero, counts_one] = make_counts(&lines);
    let mut gamma = "".to_string();
    let mut epsilon = "".to_string();
    for i in 0..12 {
        if counts_zero[i] > counts_one[i] {
            gamma += "0";
            epsilon += "1";
        } else {
            gamma += "1";
            epsilon += "0";
        }
    }
    let int_gamma = isize::from_str_radix(&gamma, 2).unwrap();
    let int_epsilon = isize::from_str_radix(&epsilon, 2).unwrap();

    println!("part 1: {}", int_gamma * int_epsilon);
}

fn part2() {
    let fname = "./input".to_string();
    let mut lines = read_lines(&fname);
    let line_len = lines[0].len();
    for bit in 0..line_len {
        //println!("{}", lines.len());
        let [counts_zero, counts_one] = make_counts(&lines);
        // oxygen:???keep most common
        let keep;
        if counts_one[bit] > counts_zero[bit] {
            keep = "1";
        } else if counts_one[bit] < counts_zero[bit] {
            keep = "0";
        } else {
            keep = "1";
        }
        //println!("keep is {}", keep);
        let mut new_lines = Vec::new();
        for line in &lines {
            if line.as_bytes()[bit] == keep.as_bytes()[0] {
                new_lines.push(line.clone());
            }
        }
        lines = new_lines;
        //println!("numbers left {:?}", lines);
        if lines.len() == 1 {
            break;
        }
    }
    let oxygen = isize::from_str_radix(&lines[0], 2).unwrap();
    //println!("oxygen {:?}", oxygen);

    let mut lines = read_lines(&fname);
    let line_len = lines[0].len();
    for bit in 0..line_len {
        let [counts_zero, counts_one] = make_counts(&lines);
        let keep;
        if counts_one[bit] < counts_zero[bit] {
            keep = "1";
        } else if counts_one[bit] > counts_zero[bit] {
            keep = "0";
        } else {
            keep = "0";
        }
        let mut new_lines = Vec::new();
        for line in &lines {
            if line.as_bytes()[bit] == keep.as_bytes()[0] {
                new_lines.push(line.clone());
            }
        }
        lines = new_lines;
        if lines.len() == 1 {
            break;
        }
    }
    let co2 = isize::from_str_radix(&lines[0], 2).unwrap();
    //println!("oxygen {:?}", co2);
    println!("part 2: {:?}", oxygen * co2);
}

fn main() {
    part1();
    part2();
}
