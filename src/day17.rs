use std::ops::Range;
use regex::Regex;

fn sign(x: i32) -> i32 {
    if x > 0 { 1 } else if x < 0 { -1 } else { 0 }
}


fn parse_target(s: String) -> (Range<i32>, Range<i32>) {
    let r = Regex::new(r"target area: x=(-?\d+)..(-?\d+), y=(-?\d+)..(-?\d+)").unwrap();
    let caps = r.captures(s.as_str()).unwrap();
    (
        caps.get(1).unwrap().as_str().parse::<i32>().unwrap()..caps.get(2).unwrap().as_str().parse::<i32>().unwrap() + 1,
        caps.get(3).unwrap().as_str().parse::<i32>().unwrap()..caps.get(4).unwrap().as_str().parse::<i32>().unwrap() + 1,
    )
}

fn launch_probe(mut dx: i32, mut dy: i32, x_range: Range<i32>, y_range: Range<i32>) -> (bool, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut peak = y;
    while y > y_range.start {
        x += dx;
        y += dy;
        dx -= sign(dx);
        dy -= 1;
        peak = peak.max(y);
        if x_range.contains(&x) && y_range.contains(&y) {
            return (true, peak);
        }
    }
    return (false, peak);
}

pub fn part1(input: String) {
    let (_, y_range) = parse_target(input);

    // Find possible initial Y values
    let mut best_peak = 0;
    for dy in 1..y_range.start.abs() {
        let (success, peak) = launch_probe(0, dy, -1..2, y_range.clone());
        if success {
            best_peak = best_peak.max(peak);
        }
    }
    println!("{}", best_peak);
}

pub fn part2(input: String) {
    let (x_range, y_range) = parse_target(input);

    let ymin = y_range.start;
    let dy_values: Vec<i32> = (-ymin.abs()..ymin.abs())
        .filter(|dy| {
            launch_probe(0, *dy, -1..2, y_range.clone()).0
        })
        .collect();

    let mut dx_values = vec![];
    for init_dx in 1..x_range.end {
        let mut dx = init_dx;
        let mut x = 0;
        while x < x_range.end {
            x += dx;
            dx -= sign(dx);
            if x_range.contains(&x) {
                dx_values.push(init_dx);
                break;
            } else if dx == 0 {
                break;
            }
        }
    }

    let mut total = 0;
    for dx in &dx_values {
        for dy in &dy_values {
            if launch_probe(*dx, *dy, x_range.clone(), y_range.clone()).0 {
                total += 1;
            }
        }
    }
    println!("{}", total);
}
