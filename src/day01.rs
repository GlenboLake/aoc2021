pub fn part1(input: String) {
    let mut prev = u16::MAX;
    let mut count = 0;
    for value in input.trim().split_whitespace().map(|s| s.parse().unwrap()) {
        if value > prev {
            count += 1
        }
        prev = value;
    }
    println!("{}", count);
}

pub fn part2(input: String) {
    let mut a = u32::MAX / 3;
    let mut b = u32::MAX / 3;
    let mut c = u32::MAX / 3;
    let mut count = 0;
    for value in input.trim().split_whitespace().map(|s| s.parse().unwrap()) {
        if a + b + c < b + c + value {
            count += 1
        }
        a = b;
        b = c;
        c = value;
    }
    println!("{}", count);
}