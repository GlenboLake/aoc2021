fn parse_input(input: String) -> [i32; 2] {
    let pos = input.lines()
        .map(|line| line
            .split_whitespace()
            .last().unwrap()
            .parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    [pos[0], pos[1]]
}


fn do_part1(input: String) -> i32 {
    let mut pos = parse_input(input);
    let mut scores = [0, 0];
    let mut turns = 0;
    let mut active_player = 0;
    let mut die = (1..101).cycle();
    while scores.iter().max().unwrap() < &1000 {
        let mut rolls = vec![];
        for _ in 0..3 {
            turns += 1;
            let roll = die.next().unwrap();
            rolls.push(roll);
            pos[active_player] += roll;
        }
        while pos[active_player] > 10 {
            pos[active_player] -= 10;
        }
        scores[active_player] += pos[active_player];
        // println!("Player {} rolls {} and moves to space {} for a total score of {}.",
        //          active_player + 1,
        //          rolls.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("+"),
        //          pos[active_player],
        //          scores[active_player]);
        active_player = 1 - active_player;
    }
    turns * scores[active_player]
}

pub fn part1(input: String) { println!("{}", do_part1(input)) }


#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Clone)]
struct PlayerState {
    pos: i32,
    score: i32,
}

impl PlayerState {
    fn move_by(&self, roll: i32) -> PlayerState {
        let mut pos = self.pos + roll;
        while pos > 10 {
            pos -= 10;
        }
        PlayerState {
            pos,
            score: self.score + pos,
        }
    }
}

#[derive(Eq, PartialEq, Hash)]
#[derive(Debug)]
struct GameState {
    players: [PlayerState; 2],
    active_player: usize,
}

impl GameState {
    const ROLL_COUNTS: [(i32, i64); 7] = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    const WIN_SCORE: i32 = 21;

    fn winner(&self) -> Option<i32> {
        if self.players[0].score >= Self::WIN_SCORE {
            Some(0)
        } else if self.players[1].score >= Self::WIN_SCORE {
            Some(1)
        } else {
            None
        }
    }

    fn count_winners(&self) -> (i64, i64) {
        let mut p1_total = 0;
        let mut p2_total = 0;

        match self.winner() {
            Some(0) => { return (1, 0); }
            Some(1) => { return (0, 1); }
            None => {}
            _ => unreachable!()
        }

        for (roll_value, count) in Self::ROLL_COUNTS {
            let mut new_players: [PlayerState; 2] = self.players.clone();
            new_players[self.active_player] = self.players[self.active_player].move_by(roll_value);
            let new_state = GameState {
                players: new_players,
                active_player: 1 - self.active_player,
            };
            let (p1, p2) = new_state.count_winners();
            p1_total += p1 * count;
            p2_total += p2 * count;
        }
        (p1_total, p2_total)
    }
}


fn do_part2(input: String) -> i64 {
    let game = GameState {
        players: parse_input(input).map(|pos| PlayerState { pos, score: 0 }),
        active_player: 0,
    };

    let (p1_wins, p2_wins) = game.count_winners();
    p1_wins.max(p2_wins)
}

pub fn part2(input: String) { println!("{}", do_part2(input)) }

#[cfg(test)]
mod tests {
    use super::*;

    static SAMPLE: &str = "Player 1 starting position: 4\nPlayer 2 starting position: 8";

    #[test]
    fn test_part1() {
        assert_eq!(do_part1(SAMPLE.to_string()), 739785);
    }

    #[test]
    fn test_part2() {
        assert_eq!(do_part2(SAMPLE.to_string()), 444356092776315);
    }
}