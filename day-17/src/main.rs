const INPUT: &str = include_str!("../input.txt");

mod intcode;

use intcode::{Int, Program};
use std::collections::HashSet;
use std::fmt;

type World = Vec<Vec<Tile>>;

fn main() {
    let program = Program::from_input(INPUT);
    println!("Part one: {}", solve_part_one(&program));
    println!("Part two: {}", solve_part_two(&program));
}

fn parse_intcode_output(output: &[Int]) -> World {
    let bytes: Vec<_> = output.iter().map(|&v| v as u8).collect();
    bytes
        .split(|&b| b == b'\n')
        .map(|line| line.iter().copied().map(Tile::from_byte).collect())
        .collect()
}

fn solve_part_one(program: &Program) -> u32 {
    let mut program = program.clone();
    let output = program.run(&[]);
    let world = parse_intcode_output(&output);
    let intersections = find_intersections(&world);
    intersections.iter().map(|(x, y)| x * y).sum::<usize>() as u32
}

fn find_intersections(world: &World) -> Vec<(usize, usize)> {
    let mut intersections = vec![];
    for (y, row) in world.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile != Tile::Scaffold {
                continue;
            }
            let above = y
                .checked_sub(1)
                .and_then(|i| world.get(i))
                .and_then(|row| row.get(x));
            let below = world.get(y + 1).and_then(|row| row.get(x));
            let left = world
                .get(y)
                .and_then(|row| x.checked_sub(1).and_then(|i| row.get(i)));
            let right = world.get(y).and_then(|row| row.get(x + 1));
            let is_intersection = [above, below, left, right]
                .iter()
                .all(|t| t.is_some() && *t.unwrap() == Tile::Scaffold);
            if is_intersection {
                intersections.push((x, y));
            }
        }
    }
    intersections
}

fn solve_part_two(program: &Program) -> Int {
    // found manually based on path from find_path()
    let main_routine = "A,B,B,C,B,C,B,C,A,A\n";
    let a = "L,6,R,8,L,4,R,8,L,12\n";
    let b = "L,12,R,10,L,4\n";
    let c = "L,12,L,6,L,4,L,4\n";

    let mut program = program.clone();
    program.write_to_memory(0, 2);

    let inputs = [main_routine, a, b, c];
    for input in &inputs {
        let input: Vec<_> = input.bytes().map(|b| b as Int).collect();
        program.run(&input);
    }
    let input: Vec<_> = "n\n".bytes().map(|b| b as Int).collect();
    let output = program.run(&input);
    let solution = output[output.len() - 1];
    solution
}

fn find_path(world: &World) -> Vec<Movement> {
    let (start_position, start_direction) = find_robot(world);
    let scaffold_positions = scaffold_positions(world);
    let mut position = start_position;
    let mut direction = start_direction;
    let mut positions_covered = HashSet::new();
    let mut path = vec![];
    while !positions_covered.is_superset(&scaffold_positions) {
        positions_covered.insert(position);

        let forward_position = next_position(position, direction);
        let forward_tile = forward_position
            .and_then(|(x, y)| world.get(y).and_then(|row| row.get(x)))
            .copied();
        if forward_tile == Some(Tile::Scaffold) {
            let i = path.len() - 1;
            let prev_movement = &mut path[i];
            if let Movement::MoveForward(steps) = *prev_movement {
                *prev_movement = Movement::MoveForward(steps + 1);
            } else {
                path.push(Movement::MoveForward(1));
            }
            position = forward_position.unwrap();
            continue;
        }

        let left_direction = direction.turn_left();
        let left_position = next_position(position, left_direction);
        let left_tile = left_position
            .and_then(|(x, y)| world.get(y).and_then(|row| row.get(x)))
            .copied();
        if left_tile == Some(Tile::Scaffold) {
            path.push(Movement::TurnLeft);
            direction = left_direction;
            continue;
        }

        let right_direction = direction.turn_right();
        let right_position = next_position(position, right_direction);
        let right_tile = right_position
            .and_then(|(x, y)| world.get(y).and_then(|row| row.get(x)))
            .copied();
        if right_tile == Some(Tile::Scaffold) {
            path.push(Movement::TurnRight);
            direction = right_direction;
        }
    }
    path
}

fn next_position(position: (usize, usize), direction: Direction) -> Option<(usize, usize)> {
    let (x, y) = position;
    let next_position = match direction {
        Direction::Up => y.checked_sub(1).map(|y| (x, y)),
        Direction::Down => Some((x, y + 1)),
        Direction::Left => x.checked_sub(1).map(|x| (x, y)),
        Direction::Right => Some((x + 1, y)),
    };
    next_position
}

fn find_robot(world: &World) -> ((usize, usize), Direction) {
    for (y, row) in world.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if let Tile::Robot(direction) = tile {
                return ((x, y), direction);
            }
        }
    }
    panic!("no robot found");
}

fn scaffold_positions(world: &World) -> HashSet<(usize, usize)> {
    let mut positions = HashSet::new();
    for (y, row) in world.iter().enumerate() {
        for (x, &tile) in row.iter().enumerate() {
            if tile == Tile::Scaffold {
                positions.insert((x, y));
            }
        }
    }
    positions
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Movement {
    TurnLeft,
    TurnRight,
    MoveForward(usize),
}

impl fmt::Display for Movement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TurnLeft => String::from("L"),
            Self::TurnRight => String::from("R"),
            Self::MoveForward(steps) => steps.to_string(),
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Scaffold,
    Robot(Direction),
}

impl Tile {
    fn from_byte(byte: u8) -> Self {
        match byte {
            b'.' => Self::Empty,
            b'#' => Self::Scaffold,
            b'^' => Self::Robot(Direction::Up),
            b'>' => Self::Robot(Direction::Right),
            b'v' => Self::Robot(Direction::Down),
            b'<' => Self::Robot(Direction::Left),
            _ => panic!("invalid byte"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Left => Self::Up,
            Self::Down => Self::Left,
            Self::Right => Self::Down,
        }
    }
}
