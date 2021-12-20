use std::collections::{HashMap, HashSet, VecDeque};
use std::vec;

enum Size {
    Big,
    Small,
}

use Size::*;

fn size(cave: &str) -> Size {
    if cave.to_uppercase() == cave { Big } else { Small }
}


pub fn part1(input: String) -> i32 {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        if a != "end" {
            map.entry(a).or_insert(HashSet::new()).insert(b);
        }
        if a != "start" {
            map.entry(b).or_insert(HashSet::new()).insert(a);
        }
    }

    let mut total_paths = 0;
    let mut paths: VecDeque<Vec<&str>> = VecDeque::from(vec![vec!["start"]]);
    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        for &next in map.get(path.last().unwrap()).unwrap() {
            if path.iter().filter(|&value| *value == next).count() == 1 && matches!(size(next), Small) {
                // println!("Cannot revisit {}", next);
                continue;
            }
            let mut new_path = path.clone();
            new_path.push(next);
            if next == "end" {
                total_paths += 1;
            } else {
                paths.push_back(new_path);
            }
        }
    }
    total_paths
}

pub fn part2(input: String) -> i32 {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        if a != "end" && b != "start" {
            map.entry(a).or_insert(Vec::new()).push(b);
        }
        if a != "start" && b != "end" {
            map.entry(b).or_insert(Vec::new()).push(a);
        }
    }

    let mut total_paths = 0;
    let mut paths: VecDeque<(Vec<&str>, bool)> = VecDeque::from(vec![(vec!["start"], false)]);
    while !paths.is_empty() {
        let (path, has_backtrack) = paths.pop_front().unwrap();
        for &next in map.get(path.last().unwrap()).unwrap() {
            let mut backtrack = has_backtrack;
            if matches!(size(next), Small) && path.contains(&next) {
                if backtrack { continue; } else { backtrack = true; }
            }
            if next == "end" {
                total_paths += 1;
            } else {
                let mut new_path = path.clone();
                new_path.push(next);
                paths.push_back((new_path, backtrack));
            }
        }
    }
    total_paths
}
