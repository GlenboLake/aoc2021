use std::collections::HashMap;

type SumFunction = fn(&Vec<i32>, i32) -> i32;

fn lin_cost(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().map(|crab| (crab - pos).abs()).sum()
}

fn cum_cost(crabs: &Vec<i32>, pos: i32) -> i32 {
    crabs.iter().map(|crab| {
        let dist = (crab - pos).abs();
        (1..dist + 1).sum::<i32>()
    }).sum()
}

#[allow(dead_code)]
fn linear_solve(input: String, sum_fn: SumFunction) {
    // First attempt; runs part 2 in 7s
    let crabs: Vec<i32> = input.trim().split(",").map(|crab| crab.parse::<i32>().unwrap()).collect();

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap() + 1;
    let mut best = i32::MAX;
    for i in min..max {
        let move_cost = sum_fn(&crabs, i);
        if move_cost < best {
            best = move_cost;
        } else { break; }
    }

    println!("{}", best);
}

fn solve(input: String, sum_fn: SumFunction) -> i32 {
    // Second attempt with binary search;
    let crabs: Vec<i32> = input.trim().split(",").map(|crab| crab.parse::<i32>().unwrap()).collect();

    let mut slants: HashMap<i32, i32> = HashMap::new();

    let mut slant = |pos: i32| {
        *slants.entry(pos).or_insert(
            {
                let cost = sum_fn(&crabs, pos);
                let left_cost = sum_fn(&crabs, pos - 1);
                let right_cost = sum_fn(&crabs, pos + 1);

                if cost < left_cost && cost < right_cost { 0 } else if left_cost < cost { -1 } else if right_cost < cost { 1 } else { panic!("How did we get here? {} {} {}", left_cost, cost, right_cost) }
            })
    };

    let mut left = *crabs.iter().min().unwrap();
    let mut right = *crabs.iter().max().unwrap();
    let mut pos = (left + right) / 2;

    while slant(pos) != 0 {
        if slant(pos) == -1 {
            right = pos;
        } else {
            left = pos;
        }
        pos = (left + right) / 2;
    }
    sum_fn(&crabs, pos)
}

pub fn part1(input: String)-> i32 {
    solve(input, lin_cost)
}

pub fn part2(input: String)-> i32 {
    solve(input, cum_cost)
}