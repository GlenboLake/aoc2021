#[derive(Debug)]
enum Form {
    Push,
    Pop,
}

#[derive(Debug)]
struct Stage {
    a: i64,
    b: i64,
    form: Form,
}

fn parse_input(input: &String) -> Vec<Stage> {
    let num_stages = input.lines().count() / 18;
    let lines: Vec<_> = input.lines().collect();
    (0..num_stages)
        .map(|i| i * 18)
        .map(|start| {
            let form = match lines[start + 4] {
                "div z 1" => Form::Push,
                "div z 26" => Form::Pop,
                _ => unreachable!()
            };
            let a = lines[start + 5]
                .split_whitespace()
                .last().unwrap()
                .parse::<i64>().unwrap();
            let b = lines[start + 15]
                .split_whitespace()
                .last().unwrap()
                .parse::<i64>().unwrap();
            Stage { a, b, form }
        })
        .collect::<Vec<_>>()
}

#[derive(Debug)]
struct Pair {
    first: usize,
    second: usize,
    diff: i64
}

fn get_pairs(input: String) -> Vec<Pair> {
    let stages = parse_input(&input);
    let mut stack = vec![];
    let mut pairs = vec![];
    for (i, stage) in stages.iter().enumerate() {
        match stage.form {
            Form::Push => stack.push( (i, stage.b) ),
            Form::Pop => {
                let (first, b) = stack.pop().unwrap();
                pairs.push(Pair {first, second: i, diff: stage.a+b} )
            }
        }
    }
    pairs
}

pub fn part1(input: String) {
    let pairs= get_pairs(input);
    let mut result = [0; 14];
    for pair in pairs.iter() {
        if pair.diff > 0 {
            result[pair.first] = 9 - pair.diff;
            result[pair.second] = 9;
        } else {
            result[pair.first] = 9;
            result[pair.second] = 9 + pair.diff;
        }
    }
    println!("{}", result.map(|x| x.to_string()).join(""));
}

pub fn part2(input: String) {
    let pairs= get_pairs(input);
    let mut result = [0; 14];
    for pair in pairs.iter() {
        if pair.diff > 0 {
            result[pair.first] = 1;
            result[pair.second] = 1 + pair.diff;
        } else {
            result[pair.first] = 1 - pair.diff;
            result[pair.second] = 1;
        }
    }
    println!("{}", result.map(|x| x.to_string()).join(""));
}
