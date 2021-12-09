use std::collections::HashSet;
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
    let mut risk = 0;
    for x in 1..(grid.len() - 1) {
        for y in 1..(grid[0].len() - 1) {
            let center = grid[x][y].unwrap();
            let vals = vec![
                grid[x - 1][y],
                grid[x + 1][y],
                grid[x][y - 1],
                grid[x][y + 1],
            ];
            let somes = vals.iter().filter(|s| s.is_some());
            let mut lowest = true;
            for some in somes {
                if some.unwrap() <= center {
                    lowest = false;
                    break;
                }
            }
            if lowest {
                risk += 1 + grid[x][y].unwrap();
            }
        }
    }
    println!("part 1: {}", risk);
}

fn neighbors(x: usize, y: usize, grid: &Vec<Vec<Option<i32>>>) -> Vec<Option<i32>> {
    vec![
        grid[x - 1][y],
        grid[x + 1][y],
        grid[x][y - 1],
        grid[x][y + 1],
    ]
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
    let mut low_points = Vec::new();
    for x in 1..(grid.len() - 1) {
        for y in 1..(grid[0].len() - 1) {
            let center = grid[x][y].unwrap();
            let vals = neighbors(x, y, &grid);
            let somes = vals.iter().filter(|s| s.is_some());
            let mut lowest = true;
            for some in somes {
                if some.unwrap() <= center {
                    lowest = false;
                    break;
                }
            }
            if lowest {
                low_points.push((x, y));
            }
        }
    }
    let mut basin_sizes = Vec::new();
    for start in &low_points {
        let mut basin = Vec::new();
        basin.push(start.clone());
        basin = expand(basin, &grid);
        //println!("start: {:?}, len: {:?}", start, basin.len());
        basin_sizes.push(basin.len());
    }
    basin_sizes.sort();
    //println!("{:?}", basin_sizes);
    println!(
        "part 2: {}",
        basin_sizes[basin_sizes.len() - 1]
            * basin_sizes[basin_sizes.len() - 2]
            * basin_sizes[basin_sizes.len() - 3]
    )
}

fn expand(basin: Vec<(usize, usize)>, grid: &Vec<Vec<Option<i32>>>) -> Vec<(usize, usize)> {
    let (x, y) = basin[0];
    let mut basin = HashSet::new();
    basin.insert((x, y));
    for height in grid[x][y].unwrap() + 1..9 {
        let mut new_points = HashSet::new();
        for p in basin.iter() {
            let xxs = vec![p.0 - 1, p.0 + 1, p.0, p.0];
            let yys = vec![p.1, p.1, p.1 - 1, p.1 + 1];
            let it = xxs.iter().zip(yys.iter());
            for (xx, yy) in it {
                let n = grid[*xx][*yy];
                if n.is_some() {
                    let local_height = n.unwrap();
                    if local_height <= height {
                        new_points.insert((xx.clone(), yy.clone()));
                    }
                }
            }
        }
        for p in new_points {
            basin.insert(p);
        }
    }

    let out: Vec<_> = basin.clone().into_iter().collect();
    return out;
}

fn main() {
    part1();
    part2();
}
