use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

type Coord = (usize, usize);

#[derive(Eq, PartialEq)]
struct State {
    risk: usize,
    pos: Coord,
}

// Copy ordering code from BinaryHeap example on official docs
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn adjusted_risk(risk: usize, offset: usize) -> usize {
    let mut new_risk = risk + offset;
    while new_risk > 9 {
        new_risk -= 9;
    }
    new_risk
}

fn solve(input: String, tiles: usize) -> usize {
    let tile_width = input.lines().next().unwrap().len();
    let tile_height = input.lines().count();
    let mut risk_map: HashMap<Coord, usize> = HashMap::new();
    for (r, row) in input.lines().enumerate() {
        for (c, risk) in row.chars().enumerate() {
            let risk = risk.to_string().parse().unwrap();
            for h_tile in 0usize..tiles {
                for v_tile in 0usize..tiles {
                    let coord = (r + 1 + v_tile * tile_height, c + 1 + h_tile * tile_width);
                    let new_risk = adjusted_risk(risk, h_tile + v_tile);
                    risk_map.insert(coord, new_risk);
                }
            }
        }
    }

    let get_neighbors = |(r, c): (usize, usize)| {
        let mut neighbors = vec![];
        for (dr, dc) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let current_r = (r as i32) + dr;
            let current_c = (c as i32) + dc;
            if current_r < 0 || current_c < 0 { continue; }
            let key = (current_r as usize, current_c as usize);
            if risk_map.contains_key(&key) {
                neighbors.push(key);
            }
        }
        neighbors
    };

    let mut risks: HashMap<Coord, usize> = HashMap::from([((1, 1), 0)]);
    let mut spelunking: BinaryHeap<State> = BinaryHeap::from([State { risk: 0, pos: (1, 1) }]);

    while !spelunking.is_empty() {
        let state = spelunking.pop().unwrap();
        for neighbor in get_neighbors(state.pos) {
            let current_risk = match risks.get(&neighbor) {
                Some(&r) => r,
                None => usize::MAX,
            };
            let new_risk = state.risk + risk_map[&neighbor];
            if new_risk < current_risk {
                risks.insert(neighbor, new_risk);
                spelunking.push(State { risk: new_risk, pos: neighbor });
            }
        }
    }
    let target = risk_map.keys().max().unwrap();
    *risks.get(target).unwrap()
}

pub fn part1(input: String) -> i32 {
    solve(input, 1) as i32
}

pub fn part2(input: String) -> i32 {
    solve(input, 5) as i32
}