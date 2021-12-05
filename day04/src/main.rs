use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_lines(fname: String) -> std::vec::Vec<String> {
    let f = File::open(fname).expect("Unable to open file");
    let f = BufReader::new(f);
    let mut lines = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        lines.push(line);
    }
    return lines;
}

fn won(board: &Vec<i32>, hash_set: &HashSet<i32>) -> bool {
    for row in 0..5 {
        let mut has_won = true;
        for i in 0..5 {
            if !hash_set.contains(&board[row + 5 * i]) {
                has_won = false;
                break;
            }
        }
        if has_won {
            return true;
        }
    }
    for col in 0..5 {
        let mut has_won = true;
        for i in 0..5 {
            if !hash_set.contains(&board[5 * col + i]) {
                has_won = false;
                break;
            }
        }
        if has_won {
            return true;
        }
    }
    false
}

fn part1() {
    let lines = read_lines("input".to_string());
    let mut iter_lines = lines.iter();
    let random_numbers: Vec<i32> = iter_lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    println!("{:?}", random_numbers);

    let mut boards = Vec::new();
    while let Some(_) = iter_lines.next() {
        let mut current_board = Vec::new();
        for _i in 0..5 {
            for num in iter_lines
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
            {
                current_board.push(num);
            }
        }
        println!("{:?}", current_board);
        boards.push(current_board);
    }
    for i in 5..random_numbers.len() {
        let mut has_won = false;
        let hash_set: HashSet<i32> = HashSet::from_iter(random_numbers[0..i].iter().cloned());
        println!("{:?}", hash_set);
        for board in &boards {
            if won(board, &hash_set) {
                println!("has won");
                has_won = true;
                let just_called = random_numbers[i - 1];
                let mut sum = 0;
                for number in board {
                    if !hash_set.contains(number) {
                        sum += number;
                    }
                }
                let score = sum * just_called;
                println!("part 1 {}, {}, {}", score, sum, just_called);
                break;
            }
        }
        if has_won {
            break;
        }
    }
}
fn part2() {
    let lines = read_lines("input".to_string());
    let mut iter_lines = lines.iter();
    let random_numbers: Vec<i32> = iter_lines
        .next()
        .unwrap()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    let mut boards = Vec::new();
    while let Some(_) = iter_lines.next() {
        let mut current_board = Vec::new();
        for _i in 0..5 {
            for num in iter_lines
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
            {
                current_board.push(num);
            }
        }
        boards.push(current_board);
    }
    let mut finished_boards: HashSet<usize> = HashSet::new();
    let mut scores: Vec<i32> = Vec::new();
    for i in 5..random_numbers.len() {
        let hash_set: HashSet<i32> = HashSet::from_iter(random_numbers[0..i].iter().cloned());
        for board_index in 0..boards.len() {
            let board = &boards[board_index];
            if !finished_boards.contains(&board_index) {
                if won(board, &hash_set) {
                    let just_called = random_numbers[i - 1];
                    let mut sum = 0;
                    for number in board {
                        if !hash_set.contains(number) {
                            sum += number;
                        }
                    }
                    let score = sum * just_called;
                    //println!("board {}, {}", board_index, score);
                    scores.push(score);
                    finished_boards.insert(board_index);
                }
            }
        }
    }
    println!("part 1 {}", scores[0]);
    println!("part 2 {}", scores[scores.len() - 1]);
}

fn main() {
    part1();
    part2();
}

// failed attempts below
// fn load_game(fname: String) -> std::vec::Vec<i32> {
//     let lines = read_lines(fname);
//     let mut random_numbers: Option<std::vec::Vec<i32>> = None;
//     let mut current_board: Option<std::vec::Vec<std::vec::Vec<i32>>> = None;
//     let mut boards = Vec::new();
//     for line in lines {
//         if random_numbers == None {
//             let numbers = line.split(",").map(|s| s.parse().unwrap()).collect();
//             random_numbers = Some(numbers);
//         } else {
//             println!("{}", line.len());
//             if line.len() == 0 {
//                 if current_board == None {
//                     // start
//                 } else {
//                     // stop
//                     boards.push(current_board.clone());
//                     current_board = None;
//                 }
//             } else {
//                 // add numbers to current board
//                 let numbers = line
//                     .trim()
//                     .split_whitespace()
//                     .map(|s| s.parse::<i32>().unwrap())
//                     .collect();
//                 if current_board == None {
//                     //current_board = Some(numbers);
//                 } else {
//                     //current_board.unwrap().push(numbers);
//                 }
//             }
//         }
//     }
//     return random_numbers.unwrap();
// }
