use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// fn part1_test() {
//     let f = File::open("input_test").expect("Unable to open file");
//     let f = BufReader::new(f);
//     let mut grid = Vec::new();
//     for line in f.lines() {
//         let line = line.expect("Unable to read line");
//         let mut grid_line = Vec::new();
//         for val in line.chars().map(|s| s.to_string().parse::<i32>().unwrap()) {
//             grid_line.push(val);
//         }
//         grid.push(grid_line);
//     }
//     let pos = (0, 0);
//     let mut memory: HashMap<(i32, i32), i32> = HashMap::new();
//     memory.insert((grid[0].len() as i32 - 1, grid.len() as i32 - 1), 0);
//     let risk = step(pos, &grid);
//     println!("{:?}", risk);
// }

// fn step(pos: (i32, i32), grid: &Vec<Vec<i32>>) -> i32 {
//     let mut risk = 0;
//     if pos == (grid[0].len() as i32 - 1, grid.len() as i32 - 1) {
//         return 0;
//     }
//     let on_right_border: bool = (pos.1 as usize + 1) == grid[0].len();
//     let on_bottm_border: bool = (pos.0 as usize + 1) == grid.len();
//     if !on_right_border && !on_bottm_border {
//         let risk_bottom =
//             step((pos.0 + 1, pos.1), &grid) + grid[pos.0 as usize + 1][pos.1 as usize];
//         let risk_right = step((pos.0, pos.1 + 1), &grid) + grid[pos.0 as usize][pos.1 as usize + 1];
//         if risk_right < risk_bottom {
//             risk = risk_right;
//         } else {
//             risk = risk_bottom;
//         }
//     } else if on_bottm_border {
//         // we have to go right
//         risk = step((pos.0, pos.1 + 1), &grid) + grid[pos.0 as usize][pos.1 as usize + 1];
//     } else if on_right_border {
//         // we have to go down
//         risk = step((pos.0 + 1, pos.1), &grid) + grid[pos.0 as usize + 1][pos.1 as usize];
//     }
//     risk
// }

// fn part1() {
//     let f = File::open("input").expect("Unable to open file");
//     let f = BufReader::new(f);
//     let mut grid = Vec::new();
//     for line in f.lines() {
//         let line = line.expect("Unable to read line");
//         let mut grid_line = Vec::new();
//         for val in line.chars().map(|s| s.to_string().parse::<i32>().unwrap()) {
//             grid_line.push(val);
//         }
//         grid.push(grid_line);
//     }
//     let pos = (0, 0);
//     let mut memory: HashMap<(i32, i32), i32> = HashMap::new();
//     memory.insert((grid[0].len() as i32 - 1, grid.len() as i32 - 1), 0);
//     let (risk, memory) = step2(pos, &grid, memory);
//     println!("part 1: {:?}", risk);
// }

fn step2(
    pos: (i32, i32),
    grid: &Vec<Vec<i32>>,
    mut memory: HashMap<(i32, i32), i32>,
) -> (i32, HashMap<(i32, i32), i32>) {
    let mut risk = 0;
    if pos == (grid[0].len() as i32 - 1, grid.len() as i32 - 1) {
        return (0, memory);
    }
    if memory.contains_key(&pos) {
        return (*memory.get(&pos).unwrap(), memory);
    } else {
        let on_right_border: bool = (pos.1 as usize + 1) == grid[0].len();
        let on_bottm_border: bool = (pos.0 as usize + 1) == grid.len();
        if !on_right_border && !on_bottm_border {
            let (mut risk_bottom, memory_temp) = step2((pos.0 + 1, pos.1), &grid, memory);
            risk_bottom += grid[pos.0 as usize + 1][pos.1 as usize];
            let (mut risk_right, memory_temp) = step2((pos.0, pos.1 + 1), &grid, memory_temp);
            risk_right += grid[pos.0 as usize][pos.1 as usize + 1];
            if risk_right < risk_bottom {
                risk = risk_right;
            } else {
                risk = risk_bottom;
            }
            memory = memory_temp;
        } else if on_bottm_border {
            // we have to go right
            let (risk_temp, memory_temp) = step2((pos.0, pos.1 + 1), &grid, memory);
            risk = risk_temp + grid[pos.0 as usize][pos.1 as usize + 1];
            memory = memory_temp;
        } else if on_right_border {
            // we have to go down
            let (risk_temp, memory_temp) = step2((pos.0 + 1, pos.1), &grid, memory);
            risk = risk_temp + grid[pos.0 as usize + 1][pos.1 as usize];
            memory = memory_temp;
        }
        memory.insert(pos, risk);
        (risk, memory)
    }
}

fn increase(val: &i32, i: i32) -> i32 {
    let mut out = val.clone();
    for _k in 0..i + 1 {
        if out >= 9 {
            out = 1;
        } else {
            out = out + 1;
        }
    }
    out
}
fn main() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut grid = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut grid_line = Vec::new();
        for val in line.chars().map(|s| s.to_string().parse::<i32>().unwrap()) {
            grid_line.push(val);
        }
        let mut dupes = Vec::new();
        for i in 0..4 {
            for val in grid_line.iter() {
                dupes.push(increase(val, i));
            }
        }
        for val in dupes {
            grid_line.push(val);
        }
        grid.push(grid_line);
    }
    let mut dup_grid = Vec::new();
    for i in 0..4 {
        for row in grid.clone() {
            let mut dup_row = Vec::new();
            for val in row {
                dup_row.push(increase(&val, i));
            }
            dup_grid.push(dup_row);
        }
    }
    for row in dup_grid {
        grid.push(row);
    }
    println!("{:?}, {}", grid.len(), grid[0].len());
    for row in grid.clone() {
        println!("{:?}", row);
    }
    let pos = (0, 0);
    let mut memory: HashMap<(i32, i32), i32> = HashMap::new();
    let (risk, memory) = step2(pos, &grid, memory);
    println!("part 2: {:?}", risk);
}
