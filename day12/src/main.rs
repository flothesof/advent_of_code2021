use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input_test1").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut ways: HashMap<String, Vec<String>> = HashMap::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut iter = line.split("-");
        let from = iter.next().unwrap();
        let to = iter.next().unwrap();
        if from == "start" || to == "end" {
            let destinations = ways.entry(from.to_string()).or_insert(Vec::new());
            destinations.push(to.to_string());
        } else {
            let destinations = ways.entry(to.to_string()).or_insert(Vec::new());
            destinations.push(from.to_string());
            let destinations = ways.entry(from.to_string()).or_insert(Vec::new());
            destinations.push(to.to_string());
        }
    }
    println!("{:?}", ways);
    let position = "start".to_string();
    let small_caves_visited: Vec<String> = Vec::new();
    println!(
        "number of paths {}",
        number_of_paths(position, small_caves_visited, &ways)
    )
}

fn number_of_paths(
    position: String,
    mut small_caves_visited: Vec<String>,
    ways: &HashMap<String, Vec<String>>,
) -> i32 {
    let mut current_number = 0;
    if position != "end".to_string() {
        if ways.contains_key(&position) {
            for destination in &ways[&position] {
                if !small_caves_visited.contains(destination) {
                    if is_small(&destination) {
                        small_caves_visited.push(destination.clone())
                    }
                    let p =
                        number_of_paths(destination.clone(), small_caves_visited.clone(), &ways);

                    println!(
                        "from {:?} to {:?}, there are {} paths",
                        position, destination, p
                    );
                    current_number += p;
                }
            }
            return current_number;
        }
        return 0;
    } else {
        return 1;
    }
}

fn is_small(destination: &String) -> bool {
    if destination.to_uppercase() == *destination {
        return false;
    }
    true
}
