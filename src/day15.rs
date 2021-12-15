use std::collections::{HashMap, VecDeque};

type Coord = (usize, usize);

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
    let mut spelunking: VecDeque<(Coord, usize)> = VecDeque::from([((1, 1), 0)]);

    while !spelunking.is_empty() {
        let (pos, path_risk) = spelunking.pop_front().unwrap();
        for neighbor in get_neighbors(pos) {
            let current_risk = match risks.get(&neighbor) {
                Some(&r) => r,
                None => usize::MAX,
            };
            let new_risk = path_risk + risk_map[&neighbor];
            if new_risk < current_risk {
                risks.insert(neighbor, new_risk);
                spelunking.push_back((neighbor, new_risk));
            }
        }
    }
    let target = risk_map.keys().max().unwrap();
    *risks.get(target).unwrap()
}

pub fn part1(input: String) {
    let sample = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581".to_string();
    assert_eq!(solve(sample.clone(), 1), 40);
    println!("{}", solve(input, 1));
}

pub fn part2(input: String) {
    let sample = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581".to_string();
    assert_eq!(solve(sample.clone(), 5), 315);
    println!("{}", solve(input, 5));
}