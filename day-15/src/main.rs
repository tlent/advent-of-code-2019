const INPUT: &str = include_str!("../input.txt");

mod intcode;

use intcode::{Int, Program};
use std::collections::{HashMap, HashSet};
use std::fmt;
use Direction::*;

type Position = (i32, i32);

fn main() {
    let program = Program::from_input(INPUT);
    let map = map_world(&program);
    print_map(&map);
    println!("Part one: {}", solve_part_one(&map));
    println!("Part two: {}", solve_part_two(&map));
}

fn map_world(program: &Program) -> HashMap<Position, Tile> {
    let mut map = HashMap::new();
    let start_position = (0, 0);
    map.insert(start_position, Tile::Empty);
    let mut stack = vec![];
    for &direction in [North, East, South, West].iter() {
        stack.push((program.clone(), start_position, direction));
    }
    while let Some((mut program, drone_position, direction)) = stack.pop() {
        let destination = next_position(drone_position, direction);
        let command = direction.to_command();
        let output = program.run(&[command]);
        let output = output[0];
        if output == 0 {
            map.insert(destination, Tile::Wall);
            continue;
        }
        let tile = match output {
            1 => Tile::Empty,
            2 => Tile::OxygenSystem,
            _ => panic!("invalid output"),
        };
        map.insert(destination, tile);
        let drone_position = destination;
        for &direction in [North, East, South, West].iter() {
            let destination = next_position(drone_position, direction);
            if map.contains_key(&destination) {
                continue;
            }
            stack.push((program.clone(), drone_position, direction));
        }
    }
    map
}

fn print_map(map: &HashMap<Position, Tile>) {
    let x_values = map.keys().map(|(x, _)| x);
    let min_x = *x_values.clone().min().unwrap();
    let max_x = *x_values.clone().max().unwrap();
    let y_values = map.keys().map(|(_, y)| y);
    let min_y = *y_values.clone().min().unwrap();
    let max_y = *y_values.clone().max().unwrap();
    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            if (x, y) == (0, 0) {
                print!("S");
                continue;
            }
            print!("{}", map.get(&(x, y)).unwrap_or(&Tile::Unknown));
        }
        println!()
    }
}

fn next_position(current_position: Position, direction: Direction) -> Position {
    let (x, y) = current_position;
    match direction {
        North => (x, y + 1),
        East => (x + 1, y),
        South => (x, y - 1),
        West => (x - 1, y),
    }
}

fn shortest_path_length(
    map: &HashMap<Position, Tile>,
    start: Position,
    target: Position,
) -> Option<u32> {
    let mut steps = 0;
    let mut queue = vec![start];
    let mut seen = HashSet::new();
    while !queue.is_empty() {
        let mut next_queue = vec![];
        for position in queue {
            if position == target {
                return Some(steps);
            }
            for &direction in &[North, East, South, West] {
                let next_pos = next_position(position, direction);
                if seen.contains(&next_pos) || map[&next_pos] == Tile::Wall {
                    continue;
                }
                seen.insert(next_pos);
                next_queue.push(next_pos);
            }
        }
        queue = next_queue;
        steps += 1;
    }
    None
}

fn oxygen_system_position(map: &HashMap<Position, Tile>) -> Position {
    map.iter()
        .find_map(|(&position, &tile)| {
            if tile == Tile::OxygenSystem {
                Some(position)
            } else {
                None
            }
        })
        .unwrap()
}

fn solve_part_one(map: &HashMap<Position, Tile>) -> u32 {
    let target = oxygen_system_position(map);
    shortest_path_length(map, (0, 0), target).unwrap()
}

fn steps_to_fill(map: &HashMap<Position, Tile>, start: Position) -> u32 {
    let mut steps = 0;
    let mut queue = vec![start];
    let mut seen = HashSet::new();
    seen.insert(start);
    while !queue.is_empty() {
        let mut next_queue = vec![];
        for position in queue {
            for &direction in &[North, East, South, West] {
                let next_pos = next_position(position, direction);
                if seen.contains(&next_pos) || map[&next_pos] == Tile::Wall {
                    continue;
                }
                seen.insert(next_pos);
                next_queue.push(next_pos);
            }
        }
        queue = next_queue;
        if !queue.is_empty() {
            steps += 1;
        }
    }
    steps
}

fn solve_part_two(map: &HashMap<Position, Tile>) -> u32 {
    let start = oxygen_system_position(map);
    steps_to_fill(map, start)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    OxygenSystem,
    Unknown,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Wall => '#',
            Self::OxygenSystem => 'O',
            Self::Unknown => ' ',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn to_command(&self) -> Int {
        match self {
            Self::North => 1,
            Self::South => 2,
            Self::West => 3,
            Self::East => 4,
        }
    }
}
