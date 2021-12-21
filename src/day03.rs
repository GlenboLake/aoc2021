use std::collections::HashMap;

fn common_value(items: &Vec<&str>, pos: usize) -> char {
    let mut chars = HashMap::new();

    for i in items {
        let ch = i.chars().nth(pos).unwrap();
        *chars.entry(ch).or_insert(0) += 1;
    }
    chars.into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(value, _)| value)
        .unwrap()
}

pub fn part1(input: String) {
    let instructions: Vec<&str> = input.lines().collect();
    let n_bits = instructions[0].len();

    let mut gamma = 0;
    let mut epsilon = 0;

    for i in 0..n_bits {
        gamma *= 2;
        epsilon *= 2;
        match common_value(&instructions, i) {
            '1' => gamma += 1,
            '0' => epsilon += 1,
            _ => panic!(),
        };
    }

    println!("{}", gamma * epsilon);
}

pub fn part2(input: String) {
    let instructions: Vec<&str> = input.lines().collect();

    let mut candidates = instructions.clone();
    for pos in 0..instructions[0].len() {
        let filter_value = common_value(&candidates, pos);
        candidates.retain(|x| x.chars().nth(pos).unwrap() == filter_value);
        if candidates.len() == 1 { break; };
    }
    let o2_rating = i32::from_str_radix(candidates.pop().unwrap(), 2).unwrap();

    candidates = instructions.clone();
    for pos in 0..instructions[0].len() {
        let filter_value = common_value(&candidates, pos);
        candidates.retain(|x| x.chars().nth(pos).unwrap() != filter_value);
        if candidates.len() == 1 { break; };
    }
    let co2_rating = i32::from_str_radix(candidates.pop().unwrap(), 2).unwrap();

    println!("{}", o2_rating * co2_rating);
}