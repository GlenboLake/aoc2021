use std::collections::HashMap;
use std::slice::Iter;

#[derive(Debug)]
#[derive(Eq, PartialEq)]
enum Op {
    Inp,
    Add,
    Mul,
    Div,
    Mod,
    Eql,
}

impl Op {
    fn from_str(s: &str) -> Op {
        match s {
            "inp" => Op::Inp,
            "add" => Op::Add,
            "mul" => Op::Mul,
            "div" => Op::Div,
            "mod" => Op::Mod,
            "eql" => Op::Eql,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
enum Arg {
    Register(char),
    Value(i64),
}

#[derive(Debug)]
#[derive(Eq, PartialEq)]
struct Instruction {
    op: Op,
    args: Vec<Arg>,
}

impl Instruction {
    fn from(line: &str) -> Instruction {
        let (op, args) = line.split_once(" ").unwrap();
        let op = Op::from_str(op);
        let args = args.split_whitespace()
            .map(|arg| {
                match arg.parse::<i64>() {
                    Ok(i) => Arg::Value(i),
                    Err(_) => Arg::Register(arg.chars().next().unwrap())
                }
            })
            .collect::<Vec<_>>();
        Instruction { op, args }
    }

    fn run(&self, regs: &mut Registers, inputs: &mut Iter<i64>) {
        use Op::*;
        use Arg::*;
        if matches!(self.op, Inp) {
            let value = *inputs.next().unwrap();
            assert_eq!(self.args.len(), 1);
            match self.args[0] {
                Register(r) => regs.insert(r, value),
                Value(_) => unreachable!(),
            };
        } else {
            let dest = match self.args.get(0).unwrap() {
                Register(r) => r,
                Value(_) => { unreachable!() }
            };
            let value = match self.args.get(1).unwrap() {
                Register(r) => regs.get(r).unwrap(),
                Value(v) => v,
            };

            let result = match self.op {
                Inp => unreachable!(),
                Add => {
                    regs.get(dest).unwrap() + value
                }
                Mul => {
                    regs.get(dest).unwrap() * value
                }
                Div => {
                    regs.get(dest).unwrap() / value
                }
                Mod => {
                    regs.get(dest).unwrap() % value
                }
                Eql => {
                    if regs.get(dest).unwrap() == value {
                        1
                    } else {
                        0
                    }
                }
            };
            regs.insert(*dest, result);
        }
    }
}

fn read_program(program: String) -> Vec<Instruction> {
    program.lines()
        .map(|line| Instruction::from(line))
        .collect()
}

type Registers = HashMap<char, i64>;

fn new_registers() -> Registers {
    HashMap::from([
        ('w', 0),
        ('x', 0),
        ('y', 0),
        ('z', 0),
    ])
}

fn simulate_program(program: &Vec<Instruction>, inputs: Vec<i64>) -> Registers {
    let mut regs = new_registers();
    let mut inputs = inputs.iter();

    for inst in program {
        if matches!(inst.op, Op::Inp) {
            dbg!(regs.get(&'z').unwrap());
        }
        inst.run(&mut regs, &mut inputs)
    }
    regs
}

fn next_input(input: [i64; 14]) -> Option<[i64; 14]> {
    let mut new = input.clone();
    for pos in (0..14).rev() {
        match new[pos] {
            1 => {
                if pos == 0 { return None; }
                new[pos] = 9;
            }
            _ => {
                new[pos] -= 1;
                break;
            }
        }
    }
    Some(new)
}

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

fn run_stage(z: i64, w: i64, stage: Stage) -> i64 {
    dbg!(&stage);
    match stage.form {
        Form::Push => {
            if z % 26 + stage.a == w {
                println!("Unmodified    !!!!");
                z
            } else {
                println!("Add more");
                z * 26 + w + stage.b
            }
        }
        Form::Pop => {
            if z % 26 + stage.a == w {
                println!("Pop!");
                z / 26
            } else {
                println!("Add more      !!!!");
                z + w + stage.b
            }
        }
    }
}

fn run_program(program: Vec<Stage>, inputs: Vec<i64>) -> i64 {
    let mut z = 0i64;
    let mut nums = inputs.iter();
    let mut z_stack = vec![];
    for stage in program {
        dbg!(z);
        let w = nums.next().unwrap();
        match stage.form {
            Form::Push => {
                z_stack.push(w + stage.b);
            }
            Form::Pop => {}
        }
    }
    z
}

/*
        0
Push    12
Push    321
Push    8356
Pop     321
Push    8367
Push    217555
Pop     8367
Pop     321
Push    8361
Pop     8355
Push    217239
Pop     8355
Pop     321
Pop
*/

pub fn part1(input: String) {
    let program = parse_input(&input);
    for stage in &program {
        let form = format!("{:?}:", stage.form);
        println!("{:<5} {:>3},{:>3}", form, stage.a, stage.b);
    }
    let instructions = input.lines()
        .map(|line|Instruction::from(line))
        .collect::<Vec<_>>();

    fn arr2str(arr: [i64; 14]) -> String {
        arr.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join("")
    }

    // let mut nums: Option<[i64; 14]> = Some([9; 14]);

    let nums = [9,2,9,6,7,6,9,9,9,4,9,8,9,1];
    //                   1 2 3 4 5 6 7 8 9 0 1 2 3 4
    // dbg!(run_program(program, nums.to_vec()));
    dbg!(simulate_program(&instructions, nums.to_vec()));

    // while let Some(inp) = nums {
    //     let result = *run_program(&program, inp.to_vec()).get(&'z').unwrap();
    //     if result < 100000 {
    //         println!("{} {}", arr2str(inp), result);
    //     }
    //     if result == 0 { break; }
    //     nums = next_input(inp);
    // }

    // for nums in nums_to_check {
    //     let num_str = nums.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(",");
    //     let result = *run_program(&program, nums).get(&'z').unwrap();
    //     println!("{} {}", num_str, result);
    // }
}


#[cfg(test)]
mod tests {
    use super::*;

    const BINARY_PROGRAM: &str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";

    #[test]
    fn test_parsing() {
        use super::Op::*;
        use super::Arg::*;

        let expected = vec![
            Instruction { op: Inp, args: vec![Register('w')] },
            Instruction { op: Add, args: vec![Register('z'), Register('w')] },
            Instruction { op: Mod, args: vec![Register('z'), Value(2)] },
            Instruction { op: Div, args: vec![Register('w'), Value(2)] },
            Instruction { op: Add, args: vec![Register('y'), Register('w')] },
            Instruction { op: Mod, args: vec![Register('y'), Value(2)] },
            Instruction { op: Div, args: vec![Register('w'), Value(2)] },
            Instruction { op: Add, args: vec![Register('x'), Register('w')] },
            Instruction { op: Mod, args: vec![Register('x'), Value(2)] },
            Instruction { op: Div, args: vec![Register('w'), Value(2)] },
            Instruction { op: Mod, args: vec![Register('w'), Value(2)] },
        ];

        for (line, exp) in BINARY_PROGRAM.lines().zip(&expected) {
            assert_eq!(Instruction::from(line), *exp);
        }

        assert_eq!(read_program(String::from(BINARY_PROGRAM)), expected);
    }

    #[test]
    fn dec2bin() {
        let program = BINARY_PROGRAM.lines()
            .map(|line| Instruction::from(line))
            .collect::<Vec<_>>();
        let result = simulate_program(&program, vec![12]);
        let expected = HashMap::from([
            ('w', 1),
            ('x', 1),
            ('y', 0),
            ('z', 0),
        ]);

        assert_eq!(result, expected);
    }

    #[test]
    fn check_first_digit() {
        let program = "inp w
mul x 0
add x z
mod x 26
div z 1
add x 11
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 3
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
".lines().map(|line| Instruction::from(line)).collect::<Vec<_>>();

        for a in 1..=9 {
            for b in 1..=9 {
                dbg!([a,b], simulate_program(&program, vec![a, b]).get(&'z').unwrap());
            }
        }
    }
}