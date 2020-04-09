use std::cmp;
use std::collections::{BinaryHeap, HashMap, HashSet};

const INPUT: &str = include_str!("../input.txt");

type Location = (i32, i32);

fn main() {
    let asteroid_locations = parse_input();
    let (origin, detectable_count) = solve_part_one(&asteroid_locations);
    println!("{}", detectable_count);
    println!("{}", solve_part_two(&asteroid_locations, origin));
}

fn parse_input() -> Vec<Location> {
    INPUT
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|&(_, c)| c == '#')
                .map(move |(x, _)| (x as i32, y as i32))
        })
        .flatten()
        .collect()
}

fn solve_part_one(asteroid_locations: &[Location]) -> (Location, usize) {
    let mut results = Vec::with_capacity(asteroid_locations.len());
    for &origin in asteroid_locations {
        let mut detectable = HashSet::new();
        let (origin_x, origin_y) = origin;
        for &loc in asteroid_locations {
            if loc == origin {
                continue;
            }
            let (loc_x, loc_y) = loc;
            let (relative_x, relative_y) = (loc_x - origin_x, loc_y - origin_y);
            let line = match (relative_x, relative_y) {
                (0, 0) => panic!("line cannot start at origin"),
                (0, y) => (0, if y > 0 { 1 } else { -1 }),
                (x, 0) => (if x > 0 { 1 } else { -1 }, 0),
                (x, y) => {
                    let gcf = greatest_common_factor(x, y);
                    (x / gcf, y / gcf)
                }
            };
            detectable.insert(line);
        }
        results.push((origin, detectable.len()));
    }
    results.into_iter().max_by_key(|&(_, count)| count).unwrap()
}

fn greatest_common_factor(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a.abs()
}

fn solve_part_two(asteroids: &[Location], origin: Location) -> i32 {
    let mut asteroids_by_line: HashMap<Location, BinaryHeap<cmp::Reverse<Location>>> = asteroids
        .iter()
        .filter(|&&loc| loc != origin)
        .fold(Default::default(), |mut result, &asteroid| {
            let (origin_x, origin_y) = origin;
            let (x, y) = asteroid;
            let (relative_x, relative_y) = (x - origin_x, y - origin_y);
            let line = match (relative_x, relative_y) {
                (0, 0) => panic!("invalid line"),
                (0, y) => (0, if y > 0 { 1 } else { -1 }),
                (x, 0) => (if x > 0 { 1 } else { -1 }, 0),
                (x, y) => {
                    let gcf = greatest_common_factor(x, y);
                    (x / gcf, y / gcf)
                }
            };
            let asteroids = result.entry(line).or_default();
            asteroids.push(cmp::Reverse(asteroid));
            result
        });
    let mut lines: Vec<_> = asteroids_by_line.keys().copied().collect();
    lines.sort_unstable_by(|&a, &b| {
        let f = |(x, y): Location| {
            let quad_one = x >= 0 && y < 0;
            let quad_two = x > 0 && y >= 0;
            let quad_three = x <= 0 && y > 0;
            let x = x.abs() as f32;
            let y = y.abs() as f32;
            if quad_one {
                (1, cmp::Reverse(y / x))
            } else if quad_two {
                (2, cmp::Reverse(x / y))
            } else if quad_three {
                (3, cmp::Reverse(y / x))
            } else {
                (4, cmp::Reverse(x / y))
            }
        };
        f(a).partial_cmp(&f(b)).unwrap()
    });
    let mut vaporized = Vec::with_capacity(asteroids.len());
    while asteroids_by_line.values().any(|l| !l.is_empty()) {
        let remaining_lines: Vec<_> = lines
            .iter()
            .filter(|line| !asteroids_by_line[line].is_empty())
            .collect();
        for line in remaining_lines {
            let asteroids = asteroids_by_line.get_mut(line).unwrap();
            let cmp::Reverse(asteroid) = asteroids.pop().unwrap();
            vaporized.push(asteroid);
        }
    }
    let (x, y) = vaporized[199];
    x * 100 + y
}
