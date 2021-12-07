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

fn solve(input: String, sum_fn: SumFunction) {
    let crabs: Vec<i32> = input.trim().split(",").map(|crab| crab.parse::<i32>().unwrap()).collect();
    // let crabs = vec![16, 1, 2, 0, 4, 2, 7, 1, 2, 14];

    let min = *crabs.iter().min().unwrap();
    let max = *crabs.iter().max().unwrap() + 1;
    let mut best = i32::MAX;
    for i in min..max {
        let move_cost = sum_fn(&crabs, i);
        // println!("{} -> {}", i, move_cost);
        if move_cost < best {
            best = move_cost;
        }
    }

    println!("{}", best);
}

pub fn part1(input: String) {
    solve(input, lin_cost)
}

pub fn part2(input: String) {
    solve(input, cum_cost)
}