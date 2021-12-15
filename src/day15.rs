use std::collections::{HashMap, VecDeque};

type Coord = (i32, i32);

const UP: Coord = (-1, 0);
const DOWN: Coord = (1, 0);
const LEFT: Coord = (0, -1);
const RIGHT: Coord = (0, 1);
const DIRECTIONS: [Coord; 4] = [UP, DOWN, LEFT, RIGHT];

fn solve_old(input: String, tiles: i32) -> usize {
    let grid: Vec<Vec<usize>> = input.lines().map(|line| {
        line.chars().map(|ch| {
            ch.to_string().parse::<usize>().unwrap()
        }).collect::<Vec<usize>>()
    }).collect();
    let width: i32 = grid.iter().next().unwrap().len() as i32;
    let height: i32 = grid.len() as i32;

    let mut risk_map: HashMap<Coord, usize> = HashMap::new();
    risk_map.insert((0, 0), 0);
    let mut queue: VecDeque<(Coord, usize)> = VecDeque::new();
    queue.push_back(((0, 0), 0));
    while !queue.is_empty() {
        let ((x, y), risk) = queue.pop_front().unwrap();
        for (dx, dy) in DIRECTIONS {
            let new_x = x + dx;
            let new_y = y + dy;
            if new_x < 0 || new_x > (width * tiles) - 1 || new_y < 0 || new_y > (height * tiles) - 1 { continue; }
            let current_risk = match risk_map.get(&(new_x, new_y)) {
                Some(n) => *n,
                None => usize::MAX,
            };
            let mut added_risk = grid[(new_x % width) as usize][(new_y % height) as usize] + (new_x / 10 + new_y / 10) as usize;
            while added_risk > 9 { added_risk -= 9 };
            let new_risk = risk + added_risk;
            if new_risk < current_risk {
                risk_map.insert((new_x, new_y), new_risk);
                queue.push_back(((new_x, new_y), new_risk));
            }
        }
    }
    // dbg!(&risk_map);
    risk_map[&((width * tiles) - 1, (height * tiles) - 1)]
}

struct Grid {
    base: Vec<Vec<usize>>,
    num_tiles: usize,
}

impl Grid {
    fn get_risk(&self, x: usize, y: usize) -> Option<usize> {
        // println!("Requested {},{}",x,y);
        let tile_width = self.base.iter().next().unwrap().len();
        let tile_height = self.base.len();
        if x > tile_width * self.num_tiles - 1 || y > tile_height * self.num_tiles - 1 {
            None
        } else {
            let mut add = 0;
            let mut base_x = x;
            let mut base_y = y;
            while base_x >= tile_width {
                base_x -= tile_width;
                add += 1;
            }
            while base_y >= tile_height {
                base_y -= tile_height;
                add += 1;
            }
            // println!("{},{} => {},{}", x, y, base_x, base_y);
            let mut risk = self.base[base_y][base_x];
            for _ in 0..add {
                risk = match risk {
                    9 => 1,
                    _ => risk + 1
                };
            }
            Some(risk)
        }
    }
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
    assert_eq!(solve_old(sample.clone(), 1), 40);
    println!("{}", solve_old(input, 1));
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
    assert_eq!(solve_old(sample.clone(), 5), 315);
    println!("{}", solve_old(input, 5));
}