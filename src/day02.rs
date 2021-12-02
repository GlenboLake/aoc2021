pub fn part1(input: String) {
    let instructions: Vec<&str> = input.lines().collect();

    let mut pos = 0;
    let mut depth = 0;

    for inst in instructions {
        let (dir, dist) = inst.split_once(' ').unwrap();
        let dist: i32 = dist.parse().unwrap();

        match dir {
            "down" => depth += dist,
            "up" => depth -= dist,
            "forward" => pos += dist,
            _ => panic!()
        }
    }
    println!("{}", pos * depth);
}

pub fn part2(input: String) {
    let instructions: Vec<&str> = input.lines().collect();

    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;

    for inst in instructions {
        let (dir, x) = inst.split_once(' ').unwrap();
        let x: i32 = x.parse().unwrap();

        match dir {
            "down" => aim += x,
            "up" => aim -= x,
            "forward" => {
                pos += x;
                depth += aim * x;
            },
            _ => panic!()
        }
    }
    println!("{}", pos * depth);
}