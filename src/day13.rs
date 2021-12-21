use std::collections::HashSet;
use regex::Regex;

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}


const DOT: char = '█';
const EMPTY: char = ' ';

impl Coord {
    fn fold_over(&self, dim: &str, val: i32) -> Coord {
        let new_x = match dim {
            "x" => {
                val - (self.x - val).abs()
            }
            "y" => self.x,
            _ => panic!(),
        };
        let new_y = match dim {
            "x" => self.y,
            "y" => {
                val - (self.y - val).abs()
            }
            _ => panic!(),
        };
        Coord { x: new_x, y: new_y }
    }
}


pub fn solve(input: String, max_folds: usize, print_result: bool) {
    let (coord_input, command_input) = input.split_once("\n\n").unwrap();
    let mut coords: HashSet<Coord> = coord_input.lines()
        .map(|line| {
            let (a, b) = line.split_once(",").unwrap();
            let x = a.parse::<i32>().unwrap();
            let y = b.parse::<i32>().unwrap();
            Coord { x, y }
        })
        .collect();

    let r = Regex::new(r"fold along (.)=(\d+)").unwrap();
    for (i, command) in command_input.lines().enumerate() {
        if i >= max_folds { break; }
        let caps = r.captures(command).unwrap();
        let dim = caps.get(1).unwrap().as_str();
        let value = caps.get(2).unwrap().as_str().parse::<i32>().unwrap();
        coords = coords.iter().map(|c| c.fold_over(dim, value)).collect();
    }
    println!("{}", coords.len());

    if print_result {
        let x_max = coords.iter().map(|c| c.x).max().unwrap();
        let y_max = coords.iter().map(|c| c.y).max().unwrap();
        for y in 0..y_max + 1 {
            for x in 0..x_max + 1 {
                if coords.contains(&Coord { x, y }) {
                    print!("{}", DOT);
                } else {
                    print!("{}", EMPTY);
                }
            }
            println!();
        }
    }
}

pub fn part1(input: String) {
    solve(input, 1, false);
}

pub fn part2(input: String) {
    solve(input, usize::MAX, true);
}