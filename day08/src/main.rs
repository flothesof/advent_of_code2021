use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn part1() {
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

fn common_letters(s1: &String, s2: &String) -> i32 {
    let mut common = 0;
    for c in s1.chars() {
        if s2.contains(c) {
            common += 1;
        }
    }
    common
}

fn decode(ten_sequences: Vec<String>, digits: Vec<String>) -> i32 {
    let mut mapping = HashMap::new();
    //let counts: Vec<usize> = ten_sequences.iter().map(|s| s.len()).collect();
    //println!("{:?}", ten_sequences);
    //println!("{:?}", counts);
    // maps from string to digit
    mapping.insert(
        1,
        ten_sequences[ten_sequences.iter().position(|s| s.len() == 2).unwrap()].clone(),
    );
    mapping.insert(
        7,
        ten_sequences[ten_sequences.iter().position(|s| s.len() == 3).unwrap()].clone(),
    );
    mapping.insert(
        4,
        ten_sequences[ten_sequences.iter().position(|s| s.len() == 4).unwrap()].clone(),
    );
    mapping.insert(
        8,
        ten_sequences[ten_sequences.iter().position(|s| s.len() == 7).unwrap()].clone(),
    );
    let seq: Vec<String> = ten_sequences
        .clone()
        .into_iter()
        .filter(|s| s.len() == 5)
        .filter(|s| common_letters(s, mapping.get(&1).unwrap()) == 2)
        .collect();

    mapping.insert(3, seq[0].clone());

    let seq: Vec<String> = ten_sequences
        .clone()
        .into_iter()
        .filter(|s| s.len() == 6)
        .filter(|s| common_letters(s, mapping.get(&3).unwrap()) == 5)
        .collect();
    mapping.insert(9, seq[0].clone());
    let seq: Vec<String> = ten_sequences
        .clone()
        .into_iter()
        .filter(|s| s.len() == 5)
        .filter(|s| common_letters(s, mapping.get(&4).unwrap()) == 2)
        .collect();
    mapping.insert(2, seq[0].clone());
    let seq: Vec<String> = ten_sequences
        .clone()
        .into_iter()
        .filter(|s| s.len() == 5)
        .filter(|s| common_letters(s, mapping.get(&2).unwrap()) == 3)
        .collect();
    mapping.insert(5, seq[0].clone());
    let seq: Vec<String> = ten_sequences
        .clone()
        .into_iter()
        .filter(|s| s.len() == 6)
        .filter(|s| common_letters(s, mapping.get(&7).unwrap()) == 2)
        .collect();
    mapping.insert(6, seq[0].clone());
    // last one is 0
    let mut missing = "".to_string();
    let mut found = Vec::new();
    for val in mapping.values() {
        found.push(val.clone());
    }
    for val in ten_sequences {
        if !found.contains(&val) {
            missing = val.clone();
            break;
        }
    }
    mapping.insert(0, missing);
    //println!("mapping {:?}", mapping);
    let mut reverse_mapping = HashMap::new();
    for (k, v) in mapping {
        //println!("k{:?} v{:?}", k, v);
        reverse_mapping.insert(v.clone(), k.clone());
    }
    println!("reverse mapping {:?}", reverse_mapping);
    println!("digits {:?}", digits);
    let mut code = 0;
    for (i, digit) in digits.iter().enumerate() {
        code += 10i32.pow(3 - i as u32) * reverse_mapping.get(digit).unwrap();
    }
    code
}

fn main() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut sum = 0;
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut line = line.split(" | ");
        let left = line.next().unwrap().split(" ");
        let mut ten_sequences = Vec::new();
        for item in left {
            let mut chars: Vec<char> = item.chars().collect();
            chars.sort();
            ten_sequences.push(chars.iter().cloned().collect::<String>());
        }
        let right = line.next().unwrap().split(" ");
        let mut digits = Vec::new();
        for item in right {
            let mut chars: Vec<char> = item.chars().collect();
            chars.sort();
            digits.push(chars.iter().cloned().collect::<String>());
        }
        let code = decode(ten_sequences, digits);
        println!("code: {}", code);
        sum += code;
    }
    println!("part 2: {}", sum);
}
