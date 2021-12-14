use std::collections::HashMap;

type Pair = (char, char);


fn solve(input: String, iters: i32) -> usize {
    let mut lines = input.lines();
    let mut state: HashMap<Pair, usize> = HashMap::new();
    for x in lines.next().unwrap().chars().collect::<Vec<char>>().windows(2) {
        let pair = (*x.first().unwrap(), *x.last().unwrap());
        *state.entry(pair).or_insert(0) += 1;
    }
    lines.next();
    let mut rules: HashMap<Pair, Vec<Pair>> = HashMap::new();
    for line in lines {
        let mut chars = line.chars();
        let a = chars.next().unwrap();
        let c = chars.next().unwrap();
        let b = chars.last().unwrap();
        rules.insert((a, c), vec![(a, b), (b, c)]);
    }

    for _ in 0..iters {
        let mut new_state: HashMap<Pair, usize> = HashMap::new();
        for (pair, &count) in &state {
            for &result in rules.get(pair).unwrap() {
                *new_state.entry(result).or_insert(0) += count;
            }
        }
        state = new_state;
    }

    let mut freqs: HashMap<char, usize> = HashMap::new();
    freqs.insert(input.chars().next().unwrap(), 1);
    for ((_, b), &count) in &state {
        *freqs.entry(*b).or_insert(0) += count;
    }
    freqs.values().max().unwrap() - freqs.values().min().unwrap()
}

pub fn part1(input: String) {
    println!("{}", solve(input, 10));
}

pub fn part2(input: String) {
    println!("{}", solve(input, 40));
}