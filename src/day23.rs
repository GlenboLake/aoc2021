use std::collections::{HashMap, VecDeque};

fn room(amph: char) -> usize {
    (amph as usize - 'A' as usize) * 2 + 3
}

fn move_cost(amph: char) -> usize {
    match amph {
        'A' => 1,
        'B' => 10,
        'C' => 100,
        'D' => 1000,
        _ => unreachable!(),
    }
}

const HALL: usize = 1;
const EMPTY: char = '.';
const WALL: char = '#';

type Row = usize;
type Col = usize;
type Point = (Row, Col);
type PathLength = usize;
type Grid = HashMap<Point, char>;

#[allow(unused)]
fn fmt_grid(grid: &Grid) -> String {
    let nrows = *grid.iter().map(|((r, _), _)| r).max().unwrap();
    let ncols = *grid.iter().map(|((_, c), _)| c).max().unwrap();

    (0..nrows + 1)
        .map(|r| {
            (0..ncols + 1)
                .map(|c| {
                    grid.get(&(r, c)).unwrap_or(&' ').to_string()
                })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_amphipods(grid: &Grid) -> Vec<Point> {
    grid.iter()
        .filter(|(_, ch)| ch.is_ascii_alphabetic())
        .map(|(&k, _)| k)
        .collect()
}

fn parse_input(input: String) -> Grid {
    input.lines().enumerate()
        .flat_map(|(row, line)| {
            line.chars().enumerate()
                .map(move |(col, ch)| ((row, col), ch))
        })
        .filter(|(_, ch)| ['.', '#', 'A', 'B', 'C', 'D'].contains(ch))
        .collect::<Grid>()
}

fn walk(grid: &Grid, start_row: Row, start_col: Col) -> HashMap<Point, PathLength> {
    let mut points = HashMap::new();
    let mut frontier = VecDeque::from([(start_row, start_col, 0 as PathLength)]);
    while !frontier.is_empty() {
        let (r, c, path) = frontier.pop_front().unwrap();
        let neighbors: [Point; 4] = [
            (r + 1, c), (r - 1, c),
            (r, c + 1), (r, c - 1),
        ];
        for n in neighbors {
            if points.contains_key(&n) {
                continue;
            }
            match grid.get(&n) {
                Some(&EMPTY) => {
                    points.insert(n, path + 1);
                    frontier.push_back((n.0, n.1, path + 1));
                }
                None => {}
                Some(_) => {}
            }
        }
    }
    points
}

#[derive(Debug)]
enum RoomStatus {
    Occupied,
    Ready,
    Done,
}

#[derive(Debug)]
enum AmphipodLocation {
    OwnRoom,
    Hallway,
    OtherRoom,
}

fn check_location(pos: &Point, amph_type: &char) -> AmphipodLocation {
    let (r, c) = pos;
    if r == &HALL {
        AmphipodLocation::Hallway
    } else if c == &room(*amph_type) {
        AmphipodLocation::OwnRoom
    } else {
        AmphipodLocation::OtherRoom
    }
}

fn check_room(grid: &Grid, amph_type: char) -> RoomStatus {
    let occupants = grid.iter()
        .filter(
            |((row, col), &ch)| {
                row != &HALL && col == &room(amph_type) && ch != WALL
            })
        .map(|(_, ch)| ch)
        .collect::<Vec<_>>();
    let mut empty_spaces = false;
    for &ch in occupants {
        if ch == EMPTY { empty_spaces = true; } else if ch != amph_type { return RoomStatus::Occupied; }
    }
    if empty_spaces { RoomStatus::Ready } else { RoomStatus::Done }
}

fn valid_destinations(grid: &Grid, row: Row, col: Col) -> Vec<(Point, PathLength)> {
    let atype = grid.get(&(row, col)).unwrap();
    let location = check_location(&(row, col), atype);
    let room_status = check_room(grid, *atype);
    // If it's already in its own room, and no other amphipod types are in its room, it shouldn't move
    if matches!(location, AmphipodLocation::OwnRoom) && !matches!(room_status, RoomStatus::Occupied) {
        return vec![];
    }
    let movable_points = walk(grid, row, col);
    // Find the space, if any, where this amphipod is allowed to move
    let room_dest = match room_status {
        RoomStatus::Occupied | RoomStatus::Done => { None }
        RoomStatus::Ready => {
            grid.iter()
                // Find empty tiles
                .filter(|(_, &ch)| ch == EMPTY)
                // Get the tiles that correspond to this amphipod's room
                .filter(|((row, col), _)| row != &HALL && col == &room(*atype))
                // Now just look at the actual points
                .map(|(&k, _)| k)
                // The max is the one the amphipod should try to move to
                .max()
        }
    };
    // Find valid hallway spaces that may be moved to
    let hallway_dest = grid.keys()
        .filter(|(row, _)| row == &HALL)
        .filter(|(_, col)| grid.get(&(HALL + 1, *col)).unwrap() == &WALL)
        .filter(|p| movable_points.contains_key(p))
        .map(|p| (*p, *movable_points.get(p).unwrap()))
        .collect::<Vec<_>>();

    // No matter where it is now, if an amphipod can reach it's own room, it can go there
    let mut dests = match (room_status, room_dest) {
        (RoomStatus::Ready, Some(p)) => {
            if movable_points.contains_key(&p) {
                vec![(p, *movable_points.get(&p).unwrap())]
            } else { vec![] }
        }
        _ => vec![]
    };
    // If it's NOT in the hallway, it can go INTO the hallway
    if !matches!(location, AmphipodLocation::Hallway) {
        dests.extend(hallway_dest);
    }
    dests
}

fn sort_layout(init: Grid) -> PathLength {
    let room_owners = "ABCD".chars()
        .map(|ch| (room(ch), ch))
        .collect::<HashMap<_, _>>();
    let solved_state: Grid = init.iter()
        .map(|((r, c), ch)| {
            let new_ch = *match ch {
                'A' | 'B' | 'C' | 'D' => room_owners.get(c).unwrap(),
                _ => ch
            };
            ((*r, *c), new_ch)
        })
        .collect();

    let mut states: HashMap<String, PathLength> = HashMap::new();
    let mut frontier: VecDeque<(Grid, PathLength)> = VecDeque::from([(init, 0usize)]);
    while !frontier.is_empty() {
        let (grid, path) = frontier.pop_front().unwrap();
        let grid_str = fmt_grid(&grid);
        match states.get(&grid_str) {
            Some(&score) => {
                // If there's already a better score for this state, don't bother proceeding
                if score <= path { continue; }
            }
            None => {}
        }
        states.insert(grid_str, path);
        for src_pos in find_amphipods(&grid) {
            let atype = *grid.get(&src_pos).unwrap();
            for (dest, dist) in valid_destinations(&grid, src_pos.0, src_pos.1) {
                let mut new_grid = grid.iter()
                    .map(|(&pos, &ch)|
                        if pos == src_pos {
                            (pos, '.')
                        } else if pos == dest {
                            (pos, atype)
                        } else {
                            (pos, ch)
                        }
                    )
                    .collect::<Grid>();

                new_grid.insert(dest, atype);
                new_grid.insert(src_pos, '.');
                frontier.push_back((new_grid, path + dist * move_cost(atype)));
            }
        }
    }
    *states.get(&fmt_grid(&solved_state)).unwrap()
}

pub fn part1(input: String) {
    println!("{}", sort_layout(parse_input(input)));
}

pub fn part2(input: String) {
    let extra_lines = vec![
        "  #D#C#B#A#",
        "  #D#B#A#C#",
    ];
    let mut lines = input.lines().collect::<Vec<_>>();
    for line in extra_lines.iter().rev() {
        lines.insert(3, line);
    }
    let layout = lines.join("\n");
    println!("{}", sort_layout(parse_input(layout)));
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########";

    const EXTENDED_SAMPLE: &str = "#############
#...........#
###B#C#B#D###
  #D#C#B#A#
  #D#B#A#C#
  #A#D#C#A#
  #########";

    #[test]
    fn test_check_room() {
        let input = String::from(
            "#############
#...........#
###.#.#.#D###
  #.#.#.#D#
  #.#B#B#D#
  #.#B#B#D#
  #########"
        );
        let grid = parse_input(input);

        assert!(matches!(check_room(&grid, 'A'), RoomStatus::Ready));
        assert!(matches!(check_room(&grid, 'B'), RoomStatus::Ready));
        assert!(matches!(check_room(&grid, 'C'), RoomStatus::Occupied));
        assert!(matches!(check_room(&grid, 'D'), RoomStatus::Done));
    }

    #[test]
    fn valid_dests() {
        let input = "#############
#.......D.CD#
###B#.#.#.###
  #A#.#C#A#
  #########";

        let grid = parse_input(input.to_string());

        // Check that B in the A room can go into the hallway or home
        let walkable = walk(&grid, 2, 3);
        let expected_walkable: HashMap<Point, PathLength> = HashMap::from([
            ((1, 1), 3),
            ((1, 2), 2),
            ((1, 3), 1),
            ((1, 4), 2),
            ((1, 5), 3),
            ((1, 6), 4),
            ((1, 7), 5),
            ((2, 5), 4),
            ((3, 5), 5),
            ((2, 7), 6),
        ]);
        assert_eq!(walkable, expected_walkable);
        let mut dests = valid_destinations(&grid, 2, 3);
        dests.sort();
        let expected_dests: Vec<(Point, PathLength)> = vec![
            ((1, 1), 3),
            ((1, 2), 2),
            ((1, 4), 2),
            ((1, 6), 4),
            ((3, 5), 5),
        ];
        assert_eq!(dests, expected_dests);

        // Check that leftmost D cannot go home
        let walkable = walk(&grid, 1, 8);
        let expected_walkable: HashMap<Point, PathLength> = HashMap::from([
            ((1, 1), 7),
            ((1, 2), 6),
            ((1, 3), 5),
            ((1, 4), 4),
            ((1, 5), 3),
            ((2, 5), 4),
            ((3, 5), 5),
            ((1, 6), 2),
            ((1, 7), 1),
            ((2, 7), 2),
            ((1, 9), 1),
            ((2, 9), 2),
        ]);
        assert_eq!(walkable, expected_walkable);
        let dests = valid_destinations(&grid, 1, 8);
        assert!(dests.is_empty(), "D should not be able to go anywhere");

        // Check that C that is already home shouldn't move
        assert!(valid_destinations(&grid, 3, 7).is_empty(), "C at home should not be able to go anywhere");
    }

    #[test]
    fn part1() {
        assert_eq!(sort_layout(parse_input(String::from(SAMPLE))), 12521);
    }

    #[test]
    fn part2() {
        assert_eq!(sort_layout(parse_input(String::from(EXTENDED_SAMPLE))), 44169);
    }
}