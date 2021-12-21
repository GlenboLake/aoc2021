use std::collections::HashSet;
use std::fmt;
use std::fmt::Formatter;
use std::ops::Range;

type Point = (i32, i32);

const LIT: char = '#';
const DARK: char = '.';

struct Image {
    row_bounds: Range<i32>,
    col_bounds: Range<i32>,
    background: char,
    lit_pixels: HashSet<Point>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let min_row = *self.lit_pixels.iter().map(|(x, _)| x).min().unwrap() - 2;
        let min_col = *self.lit_pixels.iter().map(|(_, y)| y).min().unwrap() - 2;
        let max_row = *self.lit_pixels.iter().map(|(x, _)| x).max().unwrap() + 3;
        let max_col = *self.lit_pixels.iter().map(|(_, y)| y).max().unwrap() + 3;
        let mut s = String::with_capacity(((max_row - min_row) * (max_col - min_col)) as usize);
        for r in min_row..max_row {
            for c in min_col..max_col {
                let ch =
                    if !self.pixel_in_bounds(r, c) {
                        self.background
                    } else if self.lit_pixels.contains(&(r, c)) {
                        '#'
                    } else {
                        '.'
                    };
                s.push(ch);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

impl Image {
    fn from_text(text: &str) -> Image {
        let mut lit_pixels = HashSet::new();
        for (i, line) in text.lines().enumerate() {
            for (j, ch) in line.chars().enumerate() {
                match ch {
                    DARK => {}
                    LIT => { lit_pixels.insert((i as i32, j as i32)); }
                    _ => unreachable!()
                }
            };
        }
        Image {
            row_bounds: 0i32..text.lines().count() as i32,
            col_bounds: 0i32..text.lines().next().unwrap().len() as i32,
            background: DARK,
            lit_pixels,
        }
    }

    fn pixel_in_bounds(&self, row: i32, col: i32) -> bool {
        self.row_bounds.contains(&row) && self.col_bounds.contains(&col)
    }

    fn pixel_value(&self, row: i32, col: i32) -> i32 {
        let diffs = [
            (-1, -1), (-1, 0), (-1, 1),
            (0, -1), (0, 0), (0, 1),
            (1, -1), (1, 0), (1, 1),
        ];
        let mut value = 0;
        for (dr, dc) in diffs {
            let r = row + dr;
            let c = col + dc;
            value = value * 2;
            if self.lit_pixels.contains(&(r, c)) {
                value += 1;
            } else if self.background == LIT && !self.pixel_in_bounds(r, c) {
                value += 1;
            }
        }
        value
    }

    fn enhance(&self, algorithm: &Vec<char>) -> Image {
        let row_range = self.row_bounds.start - 2..self.row_bounds.end + 2;
        let col_range = self.col_bounds.start - 2..self.col_bounds.end + 2;

        let mut new_pixels = HashSet::new();
        for row in row_range {
            for col in col_range.clone() {
                if algorithm[self.pixel_value(row, col) as usize] == LIT {
                    new_pixels.insert((row, col));
                }
            }
        }

        let new_background = algorithm[
            match self.background {
                DARK => 0,
                LIT => 0b111111111,
                _ => unreachable!()
            }
            ];

        Image {
            row_bounds: self.row_bounds.start - 1..self.row_bounds.end + 1,
            col_bounds: self.col_bounds.start - 1..self.col_bounds.end + 1,
            background: new_background,
            lit_pixels: new_pixels,
        }
    }
}

pub fn solve(input: String, iterations: usize) -> i32 {
//     let input = String::from("..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#
//
// #..#.
// #....
// ##..#
// ..#..
// ..###");
    let (algorithm, image) = input.split_once("\n\n").unwrap();
    let algorithm = algorithm.chars().collect::<Vec<_>>();
    let mut image = Image::from_text(image);
    // println!("{}", image);
    for _ in 0..iterations {
        image = image.enhance(&algorithm);
        // println!("\n{}", image);
    }
    image.lit_pixels.len() as i32
}

pub fn part1(input: String) { println!("{}", solve(input, 2)); }

pub fn part2(input: String) { println!("{}", solve(input, 50)); }