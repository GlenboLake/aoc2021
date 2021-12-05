use std::collections::HashMap;
use std::fmt;

use regex::Regex;

type Point = (i32, i32);

fn sign(x: i32) -> i32 {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{},{} -> {},{}", self.start.0, self.start.1, self.end.0, self.end.1)
    }
}

impl Line {
    fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }

    fn points(&self) -> Vec<Point> {
        let dx = sign(self.end.0 - self.start.0);
        let dy = sign(self.end.1 - self.start.1);
        let mut points: Vec<Point> = vec![self.start];
        let (mut x, mut y) = self.start;
        while (x, y) != self.end {
            x += dx;
            y += dy;
            points.push((x, y));
        }
        points
    }
}

fn parse_input(input: String) -> Vec<Line> {
    let r = Regex::new(r"(\d+),(\d+) -> (\d+),(\d+)").unwrap();
    let mut lines: Vec<Line> = Vec::new();
    for line in input.lines() {
        let caps = r.captures(line).unwrap();
        lines.push(Line {
            start: (caps.get(1).unwrap().as_str().parse::<i32>().unwrap(),
                    caps.get(2).unwrap().as_str().parse::<i32>().unwrap()),
            end: (caps.get(3).unwrap().as_str().parse::<i32>().unwrap(),
                  caps.get(4).unwrap().as_str().parse::<i32>().unwrap()),
        });
    };
    lines
}


pub fn part1(input: String) {
    let lines = parse_input(input);

    let mut vents: HashMap<Point, i32> = HashMap::new();

    for line in &lines {
        if line.is_diagonal() { continue; }
        for point in line.points() {
            *vents.entry(point).or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for (_, n) in vents {
        if n > 1 {
            total += 1
        }
    }
    println!("{}", total);
}

pub fn part2(input: String) {
    let lines = parse_input(input);
    let mut vents: HashMap<Point, i32> = HashMap::new();

    for line in &lines {
        for point in line.points() {
            *vents.entry(point).or_insert(0) += 1;
        }
    }

    let mut total = 0;
    for (_, n) in vents {
        if n > 1 {
            total += 1
        }
    }
    println!("{}", total);
}