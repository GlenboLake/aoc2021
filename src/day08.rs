use std::collections::{HashMap, HashSet};

fn set(s: &String) -> HashSet<char> {
    s.chars().collect::<HashSet<char>>()
}

pub fn part1(input: String) {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let lengths = vec![2usize, 4, 3, 7];
    let mut total = 0usize;
    for line in lines {
        let (_, outputs) = line.split_once(" | ").unwrap();
        let mut outputs: Vec<usize> = outputs.split_whitespace().map(|s| s.len()).collect();
        outputs.retain(|x| lengths.contains(x));
        total += outputs.len();
    }
    println!("{}", total);
}

pub fn part2(input: String) {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    let nums: Vec<usize> = lines.iter().map(|line| {
        let mut mapping: HashMap<String, usize> = HashMap::new();
        let (signals, output) = line.split_once(" | ").unwrap();
        let mut signals: Vec<String> = signals.split(" ")
            .map(|s| {
                let mut c: Vec<char> = s.chars().collect();
                c.sort();
                c.iter().collect()
            }).collect();
        signals.sort_by_key(|x| x.len());

        mapping.insert(signals[0].clone(), 1);
        mapping.insert(signals[1].clone(), 7);
        mapping.insert(signals[2].clone(), 4);
        mapping.insert(signals[9].clone(), 8);

        let mut unknown_235 = vec![&signals[3], &signals[4], &signals[5]];
        let mut unknown_069 = vec![&signals[6], &signals[7], &signals[8]];

        // 0, 6, and 9 all have 6 segments. 6 is the only one that doesn't have both of 1's segments
        let one_segments = set(&signals[0]);
        let mut six_chars: HashSet<char> = HashSet::new();
        for signal in unknown_069.clone() {
            let segments = set(signal);
            if segments.intersection(&one_segments).count() < one_segments.len() {
                mapping.insert(signal.clone(), 6);
                six_chars = set(signal);
                unknown_069.retain(|&x| x != signal);
                break;
            }
        }

        // Solve 2, 3, and 5
        // 3 is the only one that has both of 1's digits
        for signal in unknown_235.clone() {
            let segments = set(signal);
            if segments.intersection(&one_segments).count() == one_segments.len() {
                mapping.insert(signal.clone(), 3);
                unknown_235.retain(|&x| x != signal);
                break;
            }
        }
        // 5 is the same as 6 with one segment missing
        for signal in unknown_235.clone() {
            let segments = set(signal);
            if segments.intersection(&six_chars).count() == 5 {
                mapping.insert(signal.clone(), 5);
                unknown_235.retain(|&x| x != signal);
                break;
            }
        }
        // 2 is the only other 5-segment digit
        mapping.insert(unknown_235.pop().unwrap().clone(), 2);

        // Only 9 and 0 remain. 9 is the one that has all the same segments as 4.
        let four_segments = set(&signals[2]);
        for signal in unknown_069.clone() {
            let segments = set(signal);
            if segments.intersection(&four_segments).count() == four_segments.len() {
                mapping.insert(signal.clone(), 9);
                unknown_069.retain(|&x| x != signal);
                break;
            }
        }
        mapping.insert(unknown_069.pop().unwrap().clone(), 0);
        let output: Vec<String> = output.split(" ")
            .map(|s| {
                let mut c: Vec<char> = s.chars().collect();
                c.sort();
                c.iter().collect()
            }).collect();

        let mut result = 0 as usize;
        for n in output {
            let value = mapping.get(&*n).unwrap();
            result = 10 * result + value;
        }
        result
    }).collect();
    println!("{}", nums.iter().sum::<usize>());
}