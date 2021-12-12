use std::collections::{HashMap, HashSet, VecDeque};

enum Size {
    Big,
    Small,
}

use Size::*;

fn size(cave: &str) -> Size {
    if cave.to_uppercase() == cave { Big } else { Small }
}


pub fn part1(input: String) {
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
    println!("There are {} paths", total_paths);
}

pub fn part2(input: String) {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once("-").unwrap();
        if a != "end" && b != "start" {
            map.entry(a).or_insert(HashSet::new()).insert(b);
        }
        if a != "start" && b != "end" {
            map.entry(b).or_insert(HashSet::new()).insert(a);
        }
    }

    // let mut complete_paths: HashSet<Vec<&str>> = HashSet::new();
    let mut total_paths = 0;
    let mut paths: VecDeque<Vec<&str>> = VecDeque::from(vec![vec!["start"]]);
    while !paths.is_empty() {
        let path = paths.pop_front().unwrap();
        // Check for previous revisit
        let mut revisited = false;
        let mut seen = HashSet::new();
        for &cave in path.iter() {
            if seen.contains(cave) && matches!(size(cave), Small) {
                revisited = true;
                break;
            }
            seen.insert(cave);
        }
        for &next in map.get(path.clone().last().unwrap()).unwrap() {
            if matches!(size(next), Small) && path.contains(&next) && revisited {
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
    println!("There are {} paths", total_paths);
}