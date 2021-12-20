use std::collections::HashSet;
use std::fmt;

type Coord = (i32, i32);

struct OctoGrid {
    total_flashes: i32,
    grid: Vec<Vec<i32>>,
}

const ADJACENT: [Coord; 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl fmt::Display for OctoGrid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let text: String = self.grid.iter()
            .map(|line| {
                line.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("")
            }).collect::<Vec<String>>().join("\n");
        write!(f, "{}", text)
    }
}

impl OctoGrid {
    fn from_str(s: String) -> OctoGrid {
        let grid: Vec<Vec<i32>> = s.trim().split("\n").map(|line| {
            line.chars().map(|ch| ch.to_string().parse().unwrap()).collect()
        }).collect();
        OctoGrid { grid, total_flashes: 0 }
    }

    fn size(&self) -> i32 {
        self.grid.len().pow(2) as i32
    }

    fn step(&mut self) -> i32 {
        let size = self.grid.len() as i32;
        let mut flashed: HashSet<Coord> = HashSet::new();
        // Increase all by 1
        for row in self.grid.iter_mut() {
            for value in row.iter_mut() {
                *value += 1;
            }
        }
        // Repeatedly check flashes
        loop {
            let mut tens: HashSet<Coord> = HashSet::new();
            for (i, row) in self.grid.iter().enumerate() {
                for (j, &value) in row.iter().enumerate() {
                    let coord = (i as i32, j as i32);
                    if value > 9 && !flashed.contains(&coord) {
                        tens.insert(coord);
                        flashed.insert(coord);
                    }
                }
            }
            if tens.is_empty() { break; }
            for (i, j) in tens {
                for (di, dj) in ADJACENT {
                    let (i2, j2) = (i + di, j + dj);
                    if 0 <= i2 && i2 < size && 0 <= j2 && j2 < size {
                        self.grid[i2 as usize][j2 as usize] += 1;
                    }
                }
            }
        }
        // Reset all flashed octopuses to 0
        for (i, j) in &flashed {
            self.grid[*i as usize][*j as usize] = 0;
        }
        let num_flashes = flashed.len() as i32;
        self.total_flashes += num_flashes;
        num_flashes
    }
}

pub fn part1(input: String) -> i32 {
    let mut octos = OctoGrid::from_str(input);

    for _ in 0..100 {
        octos.step();
    }
    octos.total_flashes
}

pub fn part2(input: String) -> i32 {
    let mut octos = OctoGrid::from_str(input);

    let mut steps = 0;
    loop {
        let num_flashes = octos.step();
        steps += 1;
        if num_flashes == octos.size() {
            return steps;
        }
    }
}