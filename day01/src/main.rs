use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;


fn main(){
    part1();
    part2();
}

fn part2() {
    let mut a: Option<i32> = None;
    let mut b: Option<i32> = None;
    let mut c: Option<i32> = None;
    let mut prev:i32;
    let mut count: i32 = 0;
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if a == None {
                    a = Some(ip.parse::<i32>().unwrap());
                } else if b == None {
                    b = Some(ip.parse::<i32>().unwrap());
                } else if c == None {
                    c = Some(ip.parse::<i32>().unwrap());
                } else {
                    prev = a.unwrap() + b.unwrap() + c.unwrap();
                    a = b;
                    b = c;
                    c = Some(ip.parse::<i32>().unwrap());
                    if a.unwrap() + b.unwrap() + c.unwrap() > prev {
                        count += 1;
                    }
                }
            }
        }
    }
    println!("part 2: counted {} increases", count);
}

fn part1() {
    let mut prev: Option<i32> = None;
    let mut next: Option<i32> = None;
    let mut count: i32 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if prev == None {
                    prev = Some(ip.parse::<i32>().unwrap());
                    next = prev;
                } else {
                    prev = next;
                    next = Some(ip.parse::<i32>().unwrap());
                    if next > prev {
                        // println!("prev {:?}, next {:?}", prev.unwrap(), next.unwrap());
                        count += 1;
                    }
                }
            }
        }
    }
    println!("part 1: counted {} increases", count);
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
