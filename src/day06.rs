pub fn part1_naive(input: String) {
    // First, naive, approach. Doesn't scale well at all.
    let mut state = input.split(",").map(|x| x.trim().parse::<i32>().unwrap()).collect();
    fn cycle(state: Vec<i32>) -> Vec<i32> {
        let mut new_state: Vec<i32> = Vec::new();
        let mut num_new_fish = 0;
        for fish in state.clone() {
            if fish == 0 {
                num_new_fish += 1;
                new_state.push(6);
            } else {
                new_state.push(fish - 1);
            }
        }
        for _ in 0..num_new_fish {
            new_state.push(8);
        }
        new_state
    }

    for _ in 0..80 {
        state = cycle(state);
    }
    println!("{}", state.len())
}

fn solve(init_state: [i64; 9], iters: usize) -> i64 {
    let mut counts = init_state.clone();
    for _ in 0..iters {
        let new_fish = counts[0];
        counts[0] = counts[1];
        counts[1] = counts[2];
        counts[2] = counts[3];
        counts[3] = counts[4];
        counts[4] = counts[5];
        counts[5] = counts[6];
        counts[6] = new_fish + counts[7];
        counts[7] = counts[8];
        counts[8] = new_fish;
    }
    let result: i64 = counts.iter().sum();
    result
}

pub fn part1(input: String) {
    let mut counts = [0i64; 9];
    for n in input.split(",").map(|x| x.trim().parse::<usize>().unwrap()) {
        counts[n] += 1;
    }

    println!("{}", solve(counts, 80));
}

pub fn part2(input: String) {
    let mut counts = [0i64; 9];
    for n in input.split(",").map(|x| x.trim().parse::<usize>().unwrap()) {
        counts[n] += 1;
    }

    println!("{}", solve(counts, 256));
}