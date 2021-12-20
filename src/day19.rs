use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
#[derive(Eq, PartialEq, Hash)]
#[derive(Copy, Clone)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({},{},{})", self.x, self.y, self.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        return Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        };
    }
}

impl Vec3 {
    /// Get the square of the Pythagorean distance between two points
    fn sqr_dist_to(&self, other: &Self) -> i32 {
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        let dz = (self.z - other.z).abs();
        dx * dx + dy * dy + dz * dz
    }

    fn manhattan_to(&self, other: &Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs()
    }

    fn transform(&self, rotation: [(usize, i32); 3], translation: [i32; 3]) -> Self {
        let v = self.to_vec();
        let new = rotation.iter().zip(translation).map(|((index, sign), diff)| {
            // println!("{}*{}+{}", index, sign, diff);
            v.get(*index).unwrap() * sign + diff
        }).collect::<Vec<_>>();

        Self::from_vec(new)
    }

    fn to_vec(&self) -> Vec<i32> {
        vec![self.x, self.y, self.z]
    }

    fn from_vec(v: Vec<i32>) -> Self {
        assert_eq!(v.len(), 3, "from_vec received {} numbers", v.len());
        Self { x: v[0], y: v[1], z: v[2] }
    }
}

#[derive(Debug)]
#[derive(Clone)]
struct Scanner {
    id: usize,
    pos: Option<Vec3>,
    beacons: HashMap<Vec3, HashSet<i32>>,
}

impl Scanner {
    fn from(id: usize, lines: Vec<&str>) -> Scanner {
        let vectors: Vec<_> = lines.iter().map(|line| {
            let mut nums = line.split(",");
            Vec3 {
                x: nums.next().unwrap().parse::<i32>().unwrap(),
                y: nums.next().unwrap().parse::<i32>().unwrap(),
                z: nums.next().unwrap().parse::<i32>().unwrap(),
            }
        }).collect();
        let mut beacons = HashMap::new();
        for &v in &vectors {
            let mut dists = vectors.iter()
                .map(|other| v.sqr_dist_to(other))
                .collect::<HashSet<_>>();
            dists.remove(&0);
            beacons.insert(v, dists);
        }
        Scanner { id, pos: None, beacons }
    }

    fn overlap_with(&self, other: &Self) -> HashMap<Vec3, Vec3> {
        HashMap::from_iter(self.beacons.iter()
            .flat_map(|b| {
                other.beacons.iter().map(move |o| (b, o))
            })
            .filter(|((_, d1), (_, d2))|
                {
                    d1.intersection(d2).count() >= 11
                })
            .map(|((&b1, _), (&b2, _))| (b1, b2))
        )
    }

    fn normalized(&self, mapping: HashMap<Vec3, Vec3>) -> Scanner {
        let mut it = mapping.iter();
        let (&ref1, &point1) = it.next().unwrap();
        let (&ref2, &point2) = it.next().unwrap();

        let ref_vec = (ref2 - ref1).to_vec();
        let cur_vec = (point2 - point1).to_vec();

        // if ref_vec.iter()
        //     .map(|x| x.abs())
        //     .collect::<HashSet<_>>().len() < 2 {
        //     panic!("I didn't consider this...")
        // }

        let mut rotation = [(0, 0); 3];
        for (ii, ref_value) in ref_vec.iter().enumerate() {
            let (idx, value) = cur_vec.iter().enumerate()
                .find(|(_, value)| value.abs() == ref_value.abs()).unwrap();
            rotation[ii] = (idx, value / ref_value);
        }
        let rotated = point1.transform(rotation, [0; 3]);
        let translation = [
            ref1.x - rotated.x,
            ref1.y - rotated.y,
            ref1.z - rotated.z,
        ];

        for (&r, orig) in mapping.iter() {
            assert_eq!(orig.transform(rotation, translation), r);
        }

        let new_beacons = self.beacons.iter()
            .map(|(v, dists)| (v.transform(rotation, translation), dists.clone()))
            .collect();
        Self { id: self.id, pos: Some(Vec3::from_vec(translation.to_vec())), beacons: new_beacons }
    }
}

fn parse_input(input: String) -> Vec<Scanner> {
    input.split("\n\n").enumerate()
        .map(|(i, chunk)| {
            let lines = chunk.lines()
                .filter(|line| !line.starts_with("---"))
                .collect::<Vec<_>>();
            Scanner::from(i, lines)
        }
        )
        .collect::<Vec<_>>()
}

fn solve_scanners(scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut scanners = VecDeque::from(scanners);
    let mut init_scanner = scanners.pop_front().unwrap();
    init_scanner.pos = Some(Vec3::from_vec(vec![0, 0, 0]));
    let mut known = vec![init_scanner];
    while !scanners.is_empty() {
        'outer: for k in known.clone().iter() {
            for (i, s) in scanners.clone().iter().enumerate() {
                let overlap = k.overlap_with(s);
                if overlap.len() >= 12 {
                    let new_s = s.normalized(overlap.clone());
                    for (orig, new) in overlap.clone() {
                        assert!(new_s.beacons.contains_key(&orig));
                        assert_eq!(s.beacons.get(&new).unwrap(), new_s.beacons.get(&orig).unwrap());
                    }
                    known.push(new_s);
                    scanners.remove(i);
                    break 'outer;
                }
            }
        }
    }
    known
}

pub fn part1(input: String) -> i32 {
    solve_scanners(parse_input(input)).iter()
        .flat_map(|s| s.beacons.keys())
        .collect::<HashSet<_>>()
        .len() as i32
}

pub fn part2(input: String) -> i32 {
    let scanners = solve_scanners(parse_input(input)).iter()
        .map(|s| match s.pos {
            Some(p) => p,
            None => unreachable!(),
        })
        .collect::<Vec<_>>();
    let mut result = 0;
    for i in 0..scanners.len() {
        for j in i+1..scanners.len() {
            let a = scanners.get(i).unwrap();
            let b = scanners.get(j).unwrap();
            let dist = a.manhattan_to(b);
            result = result.max(dist);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = String::from(include_str!("../inputs/day19_test.txt"));
        assert_eq!(part1(input), 79);
    }
}