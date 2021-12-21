use std::collections::VecDeque;
use regex::Regex;

#[derive(Debug)]
#[derive(Clone)]
#[derive(Eq, PartialEq)]
enum Token {
    Start,
    End,
    Sep,
    Number(u32),
}

type SnailfishNumber = Vec<Token>;

fn fmt_tokens(tokens: &SnailfishNumber) -> String {
    use Token::*;
    tokens.iter()
        .map(|token| {
            match token {
                Start => "[".to_string(),
                End => "]".to_string(),
                Sep => ",".to_string(),
                Number(n) => format!("{}", n),
            }
        })
        .collect::<Vec<_>>()
        .join("").to_string()
}


fn tokenize(s: &str) -> SnailfishNumber {
    use Token::*;
    let mut tokens = vec![];
    let mut current_num: Option<u32> = None;
    for ch in s.chars() {
        match ch.to_digit(10) {
            Some(d) => {
                current_num = Some(current_num.unwrap_or(0) * 10 + d);
            }
            None => {
                match current_num {
                    None => {}
                    Some(n) => {
                        tokens.push(Number(n));
                        current_num = None;
                    }
                }
                match ch {
                    '[' => tokens.push(Start),
                    ']' => tokens.push(End),
                    ',' => tokens.push(Sep),
                    _ => unreachable!(),
                }
            }
        }
    }
    tokens
}

fn reduce(tokens: SnailfishNumber) -> SnailfishNumber {
    use Token::*;
    let mut sn = tokens.clone();
    loop {
        let mut index = None;
        let mut prev_number = None;
        let mut depth = 0;
        // Find a place to explode
        for (i, token) in sn.iter().enumerate() {
            match token {
                Number(n) => prev_number = Some((i, n)),
                Start => {
                    if depth == 4 {
                        index = Some(i);
                        break;
                    }
                    depth += 1
                }
                End => depth -= 1,
                _ => {}
            }
        }
        match index {
            Some(pair_start) => {
                // Unwrap numbers in pair; should be guaranteed that it's a pair of normal numbers
                let left = match sn[pair_start + 1] {
                    Number(n) => n,
                    _ => unreachable!()
                };
                let right = match sn[pair_start + 3] {
                    Number(n) => n,
                    _ => unreachable!()
                };
                // Add left to previous number
                match prev_number {
                    Some((i, n)) => sn[i] = Number(n + left),
                    None => {}
                };
                // See if there's another number to the right
                let mut next_number = None;
                for (i, token) in sn[pair_start + 5..].iter().enumerate() {
                    match token {
                        Number(n) => {
                            next_number = Some((pair_start + 5 + i, n));
                            break;
                        }
                        _ => {}
                    }
                };
                match next_number {
                    Some((i, n)) => sn[i] = Number(n + right),
                    None => {}
                };
                for _ in 0..5 {
                    sn.remove(pair_start);
                }
                sn.insert(pair_start, Number(0));
                continue;
            }
            None => {}
        };
        // Now try to split
        let mut did_split = false;
        for (i, token) in sn.clone().iter().enumerate() {
            match token {
                Number(n) => {
                    if *n >= 10 {
                        sn.remove(i);
                        did_split = true;
                        let left = Number(n / 2);
                        let right = Number(n - n / 2);
                        for t in [End, right, Sep, left, Start] {
                            sn.insert(i, t);
                        }
                        break;
                    }
                }
                _ => {}
            }
        }
        if !did_split { break; }
    }
    sn
}

fn add(sn1: SnailfishNumber, sn2: SnailfishNumber) -> SnailfishNumber {
    let mut sn = sn1.clone();
    sn.insert(0, Token::Start);
    sn.push(Token::Sep);
    sn.extend(sn2);
    sn.push(Token::End);
    sn
}

fn magnitude(sn: SnailfishNumber) -> i32 {
    let mut simple_rep = fmt_tokens(&sn);
    let simple_pair = Regex::new(r"\[(\d+),(\d+)]").unwrap();
    while let Some(caps) = simple_pair.captures(simple_rep.as_str()) {
        let left: i32 = caps.get(1).unwrap().as_str().parse().unwrap();
        let right: i32 = caps.get(2).unwrap().as_str().parse().unwrap();
        let new_value = 3 * left + 2 * right;
        simple_rep = simple_rep.replace(caps.get(0).unwrap().as_str(), new_value.to_string().as_str());
    }
    simple_rep.parse().unwrap()
}

pub fn part1(input: String) {
    let mut numbers: VecDeque<SnailfishNumber> = VecDeque::from(
        input.lines().map(|line| reduce(tokenize(line))).collect::<Vec<_>>());
    let mut result = numbers.pop_front().unwrap();
    for next in numbers {
        result = reduce(add(result, next));
    }
    println!("{}", magnitude(result));
}

pub fn part2(input: String) {
    let numbers: Vec<_> = input.lines().map(|line| reduce(tokenize(line))).collect();

    let mut best = 0;
    for a in &numbers {
        for b in &numbers {
            if a == b { continue; }
            best = best.max(magnitude(reduce(add(a.clone(), b.clone()))));
        }
    }
    println!("{}", best);
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_magnitude() {
        let cases = HashMap::from([
            ("[[1,2],[[3,4],5]]", 143),
            ("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384),
            ("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445),
            ("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791),
            ("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137),
            ("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488),
        ]);

        for (input, expected) in cases {
            assert_eq!(magnitude(tokenize(input)), expected);
        }
    }
}