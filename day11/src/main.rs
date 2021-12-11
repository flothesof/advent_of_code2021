use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut grid = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut grid_line = Vec::new();
        grid_line.push(None);
        for val in line.chars().map(|s| s.to_string().parse::<i32>().unwrap()) {
            grid_line.push(Some(val));
        }
        grid_line.push(None);
        grid.push(grid_line);
    }
    // make empty line
    let mut empty_grid_line = Vec::new();
    for _ in 0..grid[0].len() {
        empty_grid_line.push(None);
    }
    // insert at top and bottom of grid
    grid.insert(0, empty_grid_line.clone());
    grid.push(empty_grid_line.clone());

    // main loop
    let mut flashes = 0;
    for _step in 0..100 {
        // 1. add 1 to each octopus
        for x in 1..(grid.len() - 1) {
            for y in 1..(grid[0].len() - 1) {
                if grid[x][y].is_some() {
                    grid[x][y] = Some(grid[x][y].unwrap() + 1);
                }
            }
        }
        // 2. flash and propagate
        for x in 1..(grid.len() - 1) {
            for y in 1..(grid[0].len() - 1) {
                if grid[x][y].is_some() {
                    let val = grid[x][y].unwrap();
                    if val > 9 {
                        grid[x][y] = Some(i32::MIN);
                        grid = flash(x, y, grid);
                    }
                }
            }
        }
        // 3. reset min energy to 0
        for x in 1..(grid.len() - 1) {
            for y in 1..(grid[0].len() - 1) {
                if grid[x][y].is_some() {
                    if grid[x][y].unwrap() < 0 {
                        grid[x][y] = Some(0);
                        flashes += 1;
                    }
                }
            }
        }
    }
    println!("part 1: {:?}", flashes);
}

fn flash(x: usize, y: usize, mut grid: Vec<Vec<Option<i32>>>) -> Vec<Vec<Option<i32>>> {
    let neighbor_coors = vec![
        (x - 1, y),
        (x + 1, y),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y + 1),
        (x - 1, y + 1),
        (x - 1, y - 1),
        (x + 1, y - 1),
    ];
    for coor in neighbor_coors {
        if grid[coor.0][coor.1].is_some() {
            let val = grid[coor.0][coor.1].unwrap();
            if val <= 8 {
                grid[coor.0][coor.1] = Some(val + 1);
            } else {
                grid[coor.0][coor.1] = Some(i32::MIN);
                grid = flash(coor.0, coor.1, grid);
            }
        }
    }
    grid
}

fn part2() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut grid = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let mut grid_line = Vec::new();
        grid_line.push(None);
        for val in line.chars().map(|s| s.to_string().parse::<i32>().unwrap()) {
            grid_line.push(Some(val));
        }
        grid_line.push(None);
        grid.push(grid_line);
    }
    // make empty line
    let mut empty_grid_line = Vec::new();
    for _ in 0..grid[0].len() {
        empty_grid_line.push(None);
    }
    // insert at top and bottom of grid
    grid.insert(0, empty_grid_line.clone());
    grid.push(empty_grid_line.clone());

    // main loop
    let mut flashes_this_turn = 0;
    let mut steps = 0;
    while flashes_this_turn < 100 {
        flashes_this_turn = 0;
        // 1. add 1 to each octopus
        for x in 1..(grid.len() - 1) {
            for y in 1..(grid[0].len() - 1) {
                if grid[x][y].is_some() {
                    grid[x][y] = Some(grid[x][y].unwrap() + 1);
                }
            }
        }
        // 2. flash and propagate
        for x in 1..(grid.len() - 1) {
            for y in 1..(grid[0].len() - 1) {
                if grid[x][y].is_some() {
                    let val = grid[x][y].unwrap();
                    if val > 9 {
                        grid[x][y] = Some(i32::MIN);
                        grid = flash(x, y, grid);
                    }
                }
            }
        }
        // 3. reset min energy to 0
        for x in 1..(grid.len() - 1) {
            for y in 1..(grid[0].len() - 1) {
                if grid[x][y].is_some() {
                    if grid[x][y].unwrap() < 0 {
                        grid[x][y] = Some(0);
                        flashes_this_turn += 1;
                    }
                }
            }
        }
        steps += 1
    }
    println!("part 2: {:?}", steps);
}

fn main() {
    part1();
    part2();
}
