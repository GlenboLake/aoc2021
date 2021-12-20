use crate::day10::State::{Complete, Corrupt, Incomplete};

enum State {
    Complete,
    Incomplete,
    Corrupt,
}

fn score_line(line: &str) -> (State, u64) {
    let mut stack: Vec<char> = vec![];

    for ch in line.chars() {
        match ch {
            '(' => {
                stack.push(ch);
                0
            }
            ')' => match stack.pop() {
                Some('(') => 0,
                _ => return (Corrupt, 3),
            }
            '[' => {
                stack.push(ch);
                0
            }
            ']' => match stack.pop() {
                Some('[') => 0,
                _ => return (Corrupt, 57),
            }
            '{' => {
                stack.push(ch);
                0
            }
            '}' => match stack.pop() {
                Some('{') => 0,
                _ => return (Corrupt, 1197),
            }
            '<' => {
                stack.push(ch);
                0
            }
            '>' => match stack.pop() {
                Some('<') => 0,
                _ => return (Corrupt, 25137),
            }
            _ => panic!("Didn't see {} coming", ch)
        };
    }
    if stack.is_empty() {
        return (Complete, 0);
    }
    let mut score:u64 = 0;
    while !stack.is_empty() {
        score = score * 5 + match stack.pop() {
            Some('(') => 1,
            Some('[') => 2,
            Some('{') => 3,
            Some('<') => 4,
            _ => panic!("Not an opening brace")
        };
    };
    (Incomplete, score)
}

pub fn part1(input: String) -> i32 {
    let total: u64 = input.split("\n")
        .map(|line| {
            let (state, score) = score_line(line);
            match state {
                State::Complete => 0,
                State::Incomplete => 0,
                State::Corrupt => score,
            }
        })
        .sum();
    total as i32
}

pub fn part2(input: String) -> i32 {
    let mut scores: Vec<u64> = input.split("\n")
        .map(|line| {
            let (state, score) = score_line(line);
            match state {
                State::Complete => 0,
                State::Incomplete => score,
                State::Corrupt => 0,
            }
        }).collect();
    scores.retain(|&score| score > 0);
    scores.sort();
    scores[scores.len()/2] as i32
}