use std::cmp::min;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: String) -> (String, Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut split = line.split(" ");
    let instr = split.next().unwrap().to_string();
    let coords = split.next().unwrap();
    let mut xyz = coords.split(",");
    let xlim: Vec<i32> = xyz
        .next()
        .unwrap()
        .split("x=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|s| s.parse().unwrap())
        .collect();
    let ylim: Vec<i32> = xyz
        .next()
        .unwrap()
        .split("y=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|s| s.parse().unwrap())
        .collect();
    let zlim: Vec<i32> = xyz
        .next()
        .unwrap()
        .split("z=")
        .skip(1)
        .next()
        .unwrap()
        .split("..")
        .map(|s| s.parse().unwrap())
        .collect();
    (instr, xlim, ylim, zlim)
}

fn part1() {
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut reactor = HashSet::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let (instr, xlim, ylim, zlim) = parse_line(line);
        if check(&xlim) && check(&ylim) && check(&zlim) {
            for x in xlim[0]..xlim[1] + 1 {
                for y in ylim[0]..ylim[1] + 1 {
                    for z in zlim[0]..zlim[1] + 1 {
                        let t = (x, y, z);
                        if instr == "on" {
                            reactor.insert(t);
                        } else if instr == "off" {
                            if reactor.contains(&t) {
                                reactor.remove(&t);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("part 1: {}", reactor.len())
}

fn check(vec: &Vec<i32>) -> bool {
    for item in vec {
        if *item >= -50 && *item <= 50 {
            continue;
        } else {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone)]
struct Cube {
    xlim: Vec<i32>,
    ylim: Vec<i32>,
    zlim: Vec<i32>,
    sign: f32,
}

impl Cube {
    fn volume(&self) -> i64 {
        (self.xlim[1] - self.xlim[0] + 1) as i64
            * (self.ylim[1] - self.ylim[0] + 1) as i64
            * (self.zlim[1] - self.zlim[0] + 1) as i64
            * self.sign as i64
    }
}

fn get_sign(instr: &String) -> f32 {
    if instr == "on" {
        return 1.;
    } else {
        return -1.;
    }
}
fn main() {
    let part2 = true;
    let f = File::open("input").expect("Unable to open file");
    let f = BufReader::new(f);
    let mut reactor = Vec::new();
    for line in f.lines() {
        let line = line.expect("Unable to read line");
        let (instr, xlim, ylim, zlim) = parse_line(line);
        if check(&xlim) && check(&ylim) && check(&zlim) || part2 {
            let sign = get_sign(&instr);
            let cube = Cube {
                xlim,
                ylim,
                zlim,
                sign,
            };
            //println!("{:?}", cube);
            add_cube(cube, &mut reactor);
        }
    }
    let mut volume: i64 = 0;
    for cube in &reactor {
        volume += cube.volume() as i64;
    }
    println!("volume: {}", volume)
}

fn add_cube(cube: Cube, reactor: &mut Vec<Cube>) {
    let mut new_cubes = Vec::new();
    if cube.sign == 1. {
        new_cubes.push(cube.clone());
    }
    for other in reactor.iter_mut() {
        let inter = intersect(other, &cube, -other.sign);
        if inter.is_some() {
            //println!("{:?}\nhas intersection with \n{:?}\n", cube, other);
            //println!("intersection is: {:?}", inter);
            new_cubes.push(inter.unwrap());
        }
    }
    for other in new_cubes {
        reactor.push(other);
    }
}

fn intersect(cube1: &Cube, cube2: &Cube, sign: f32) -> Option<Cube> {
    let xx = intersect1d(&cube1.xlim, &cube2.xlim);
    let yy = intersect1d(&cube1.ylim, &cube2.ylim);
    let zz = intersect1d(&cube1.zlim, &cube2.zlim);
    if xx.is_none() || yy.is_none() || zz.is_none() {
        return None;
    } else {
        return Some(Cube {
            xlim: xx.unwrap(),
            ylim: yy.unwrap(),
            zlim: zz.unwrap(),
            sign: sign,
        });
    }
}

fn intersect1d(x1: &Vec<i32>, x2: &Vec<i32>) -> Option<Vec<i32>> {
    if x1[0] <= x2[0] {
        if x1[1] >= x2[0] {
            return Some(vec![x2[0], min(x1[1], x2[1])]);
        } else {
            return None;
        };
    } else {
        if x2[1] >= x1[0] {
            return Some(vec![x1[0], min(x1[1], x2[1])]);
        } else {
            return None;
        }
    }
}
