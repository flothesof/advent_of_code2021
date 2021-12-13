use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[allow(while_true)]
fn main() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut dots = HashSet::new();
    let mut line_iter = f.lines();
    while true {
        let line = line_iter.next().unwrap().expect("unable to read line");
        if line.len() == 0 {
            break;
        }
        let coords: Vec<i32> = line.split(",").map(|s| s.parse().unwrap()).collect();
        dots.insert((coords[0], coords[1]));
    }
    let mut part1 = false;
    for line in line_iter {
        let line = line.expect("unable to read line");
        if line.starts_with("fold along y=") {
            let mut iter = line.split("fold along y=");
            iter.next();
            let coord = iter.next().unwrap().parse::<i32>().unwrap();
            let mut new_dots = HashSet::new();
            for dot in dots {
                if dot.1 > coord {
                    let delta = dot.1 - coord;
                    new_dots.insert((dot.0, dot.1 - 2 * delta));
                } else {
                    new_dots.insert(dot);
                }
            }
            dots = new_dots;
        } else {
            let mut iter = line.split("fold along x=");
            iter.next();
            let coord = iter.next().unwrap().parse::<i32>().unwrap();
            let mut new_dots = HashSet::new();
            for dot in dots {
                if dot.0 > coord {
                    let delta = dot.0 - coord;
                    new_dots.insert((dot.0 - 2 * delta, dot.1));
                } else {
                    new_dots.insert(dot);
                }
            }
            dots = new_dots;
        }
        if !part1 {
            println!("part 1: {:?}", dots.len());
            part1 = true;
        }
    }
    println!("part2:");
    for y in 0..6 {
        let mut s = String::new();
        for x in 0..40 {
            if dots.contains(&(x, y)) {
                s += "#";
            } else {
                s += " ";
            }
        }
        println!("{:?}", s);
    }
}
