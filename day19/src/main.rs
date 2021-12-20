use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input_test").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut line_iter = f.lines();
    let mut scanner_points = Vec::new();
    while true {
        let mut next_line = line_iter.next();
        if next_line.is_none() {
            break;
        } else {
            let mut line = next_line.unwrap().expect("Unable to read line");
            println!("{:?}", line);
            if line.starts_with("--- scanner") {
                let mut points = Vec::new();
                while true {
                    next_line = line_iter.next();
                    if next_line.is_none() {
                        break;
                    } else {
                        line = next_line.unwrap().expect("Unable to read line");
                        println!("{:?}", line);
                        if line.len() == 0 {
                            scanner_points.push(points);
                            break;
                        } else {
                            let point: Vec<i32> =
                                line.split(",").map(|s| s.parse().unwrap()).collect();
                            points.push(point);
                        }
                    }
                }
            }
        }
    }
    println!("{}", scanner_points.len());
}
