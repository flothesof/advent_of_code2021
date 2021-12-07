use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[allow(dead_code)]
fn part1() {
    // this function does not work for part2 since it grows exponentially
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


fn main() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut fishs: HashMap<i64, i64> = HashMap::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let ints: Vec<i64> = line.split(",").map(|s| s.parse::<i64>().unwrap()).collect();
        for fish in ints {
            let count = fishs.entry(fish).or_insert(0);
            *count += 1;
        }

    }
    for day in 0..256 {
        let mut new_fishs: HashMap<i64, i64> = HashMap::new();
        for (timer, &fish_count) in fishs.iter(){
            //println!("{}", timer);
            if timer == &0 {
                //new_fishs.insert(6, fish_count);
                let count = new_fishs.entry(6).or_insert(0);
                *count += fish_count;
                new_fishs.insert(8, fish_count);
            }
            else {
                let count = new_fishs.entry(timer - 1).or_insert(0);
                *count += fish_count;
            }
        }
        //println!("new fish: {:?}", new_fishs);
        fishs = new_fishs;
        let mut fish_count = 0;
        for (_, &c) in fishs.iter() {
            fish_count += c;
        }
        if (day == 79) | (day == 255) {
            println!("day: {}, count: {:?}", day, fish_count);
        }
        
    }    
    
}