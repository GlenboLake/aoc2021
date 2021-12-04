use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
struct Board {
    nums: [[i32; 5]; 5],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rows = self.nums.map(|row| {
            format!("{:>2} {:>2} {:>2} {:>2} {:>2}", row[0], row[1], row[2], row[3], row[4])
        });
        write!(f, "{}\n{}\n{}\n{}\n{}", rows[0], rows[1], rows[2], rows[3], rows[4])
    }
}

impl Board {
    fn from_str(s: &str) -> Board {
        let mut nums = [[0; 5]; 5];
        for (i, row) in s.split("\n").enumerate() {
            // println!("Row {} is {:?}", i, row);
            for (j, num) in row.split_whitespace().map(|n| n.parse::<i32>().unwrap()).enumerate() {
                nums[i][j] = num;
                // println!("{} -> {}", j, num);
            }
        }
        Board { nums }
    }

    fn row(&self, idx: usize) -> [i32; 5] {
        self.nums[idx]
    }
    fn column(&self, idx: usize) -> [i32; 5] {
        let mut col = [0; 5];
        for (i, row) in self.nums.iter().enumerate() {
            col[i] = row[idx];
        }
        col
    }

    fn check(&self, called_nums: &[i32]) -> bool {
        for x in 0..5 {
            if self.row(x).iter().all(|n| called_nums.contains(n)) {
                return true;
            }
            if self.column(x).iter().all(|n| called_nums.contains(n)) {
                return true;
            }
        };
        false
    }

    fn score(&self, called_nums: &[i32]) -> i32 {
        let mut uncalled_sum = 0;
        for row in self.nums {
            for num in row {
                if !called_nums.contains(&num) {
                    uncalled_sum += num;
                }
            }
        }
        uncalled_sum * called_nums.last().unwrap()
    }
}


fn parse_input(s: String) -> (Vec<i32>, Vec<Board>) {
    let mut inputs: std::str::Split<&str> = s.split("\n\n");

    let nums: Vec<i32> = inputs.next().unwrap().split(",").map(|x| x.parse::<i32>().unwrap()).collect();
    let boards: Vec<Board> = inputs.map(|s| Board::from_str(s)).collect();

    (nums, boards)
}


pub fn part1(input: String) {
    let (nums, boards) = parse_input(input);

    for i in 1..nums.len() {
        let called_nums = &nums[0..i];
        for board in &boards {
            if board.check(called_nums) {
                println!("{}", board.score(called_nums));
                return;
            }
        }
    }
}

pub fn part2(input: String) {
    let (nums, boards) = parse_input(input);

    let mut last_score = 0;
    let mut completed: HashSet<&Board> = HashSet::new();

    for i in 1..nums.len() {
        let called_nums = &nums[0..i];
        for board in &boards {
            if completed.contains(board) {
                continue
            }
            if board.check(called_nums) {
                completed.insert(board);
                last_score = board.score(called_nums);
            }
        }
    }
    println!("{}", last_score);
}