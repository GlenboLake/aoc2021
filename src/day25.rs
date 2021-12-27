use std::collections::HashMap;

pub fn part1(input: String) {
    println!("{}", solve(input));
    println!();
}

fn parse_input(input: String) -> (usize, usize, HashMap<(usize, usize), char>) {
    let nrows = input.lines().count();
    let ncols = input.lines().next().unwrap().chars().count();
    let cukes = input.lines().enumerate()
        .flat_map(|(row, line)|
            line.chars().enumerate()
                .map(move |(col, ch)| ((row, col), ch))
        )
        .filter(|(_, ch)| ch != &'.')
        .collect::<HashMap<(usize, usize), char>>();
    (nrows, ncols, cukes)
}

#[allow(unused)]
fn show_grid(nrows: usize, ncols: usize, grid: &HashMap<(usize, usize), char>) {
    for r in 0..nrows {
        for c in 0..ncols {
            print!("{}", grid.get(&(r, c)).unwrap_or(&'.'));
        }
        println!();
    }
    println!();
}

fn solve(input: String) -> i32 {
    let (nrows, ncols, mut grid) = parse_input(input);
    let mut done = false;
    let to_right = |pos| {
        let (r, c) = pos;
        let mut new_c = c + 1;
        if new_c >= ncols {
            new_c -= ncols;
        }
        (r, new_c)
    };
    let to_down = |pos| {
        let (r, c) = pos;
        let mut new_r = r + 1;
        if new_r >= nrows {
            new_r -= nrows;
        }
        (new_r, c)
    };

    let mut iters = 0;
    while !done {
        iters += 1;
        done = true;
        let can_move_right = grid.iter()
            .filter(|(_, &ch)| ch == '>')
            .filter(|(&pos, _)| !grid.contains_key(&to_right(pos)))
            .map(|((r, c), _)| (*r, *c))
            .collect::<Vec<_>>();
        if !can_move_right.is_empty() { done = false; }
        for pos in can_move_right {
            grid.remove(&pos);
            grid.insert(to_right(pos), '>');
        }

        let can_move_down = grid.iter()
            .filter(|(_, &ch)| ch == 'v')
            .filter(|(&pos, _)| !grid.contains_key(&to_down(pos)))
            .map(|((r, c), _)| (*r, *c))
            .collect::<Vec<_>>();
        if !can_move_down.is_empty() { done = false; }
        for pos in can_move_down {
            grid.remove(&pos);
            grid.insert(to_down(pos), 'v');
        }
    }
    iters
}

#[cfg(test)]
mod tests {
    use super::solve;

    #[test]
    fn sample() {
        let input = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

        assert_eq!(solve(input.to_string()), 58)
    }
}