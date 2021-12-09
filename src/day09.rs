use std::collections::{HashMap, HashSet, VecDeque};

type Coord = (i32, i32);


fn parse_input(input: String) -> HashMap<Coord, i32> {
    let mut height_map: HashMap<Coord, i32> = HashMap::new();

    for (r, line) in input.split("\n").enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let coord: Coord = (r as i32, c as i32);
            height_map.insert(coord, ch.to_string().parse::<i32>().unwrap());
        }
    }
    height_map
}

pub fn part1(input: String) {
    let height_map = parse_input(input);
    let mut risk = 0;
    for ((r, c), value) in height_map.clone() {
        let &north = height_map.get(&(r - 1, c)).unwrap_or(&i32::MAX);
        let &south = height_map.get(&(r + 1, c)).unwrap_or(&i32::MAX);
        let &east = height_map.get(&(r, c + 1)).unwrap_or(&i32::MAX);
        let &west = height_map.get(&(r, c - 1)).unwrap_or(&i32::MAX);
        if north > value && south > value && east > value && west > value {
            risk += value + 1;
        }
    }
    println!("{}", risk);
}

type Basin = HashSet<Coord>;

fn neighbors(c: &Coord) -> [Coord; 4] {
    return [
        (c.0 - 1, c.1),
        (c.0 + 1, c.1),
        (c.0, c.1 - 1),
        (c.0, c.1 + 1),
    ];
}

fn explore_basin(map: &HashMap<Coord, i32>, start: &Coord) -> Basin {
    let mut basin: Basin = HashSet::new();
    basin.insert(*start);
    let mut check: VecDeque<Coord> = VecDeque::new();
    check.extend(neighbors(start));
    while !check.is_empty() {
        let coord = &check.pop_front().unwrap();
        if basin.contains(coord) { continue; }
        if !map.contains_key(coord) { continue; }
        if map.get(coord).unwrap() == RIDGE { continue; }

        basin.insert(*coord);
        for n in neighbors(coord) {
            check.push_back(n);
        }
    }
    basin
}

const RIDGE: &i32 = &9;

pub fn part2(input: String) {
    let height_map = parse_input(input);
    let mut seen: HashSet<Coord> = HashSet::new();

    let mut basin_sizes: Vec<usize> = Vec::new();

    for (coord, value) in &height_map {
        if seen.contains(coord) { continue; }
        if value == RIDGE { continue; }
        let basin = explore_basin(&height_map, coord);
        basin_sizes.push(basin.len());
        seen.extend(basin.iter());
    }
    basin_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    basin_sizes.truncate(3);
    println!("{}", basin_sizes.into_iter().reduce(|a,b|a*b).unwrap());
}