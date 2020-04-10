mod intcode;

use intcode::{Int, Program};
use std::collections::HashMap;

const INPUT: &str = include_str!("../input.txt");

type Color = Int;
type Point = (i64, i64);

fn main() {
    println!("Part one: {}", solve_part_one());
    println!("Part two:\n{}", solve_part_two());
}

fn solve_part_one() -> usize {
    let mut robot = PaintRobot::new();
    let mut panels: HashMap<Point, Color> = HashMap::new();
    robot.run(&mut panels);
    panels.len()
}

fn solve_part_two() -> String {
    let mut robot = PaintRobot::new();
    let mut panels: HashMap<Point, Color> = HashMap::new();
    panels.insert((0, 0), 1);
    robot.run(&mut panels);

    let white_panel_keys = panels.iter().filter(|(_, &v)| v == 1).map(|(&k, _)| k);
    let x_values = white_panel_keys.clone().map(|(x, _)| x);
    let min_x = x_values.clone().min().unwrap();
    let max_x = x_values.clone().max().unwrap();
    let y_values = white_panel_keys.map(|(_, y)| y);
    let min_y = y_values.clone().min().unwrap();
    let max_y = y_values.clone().max().unwrap();
    let mut result = String::new();
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let c = match panels.get(&(x, y)) {
                Some(1) => '#',
                _ => '.',
            };
            result.push(c);
        }
        result.push('\n');
    }
    result
}

#[derive(Debug, Clone)]
struct PaintRobot {
    position: Point,
    direction: Direction,
    program: Program,
}

impl PaintRobot {
    fn new() -> Self {
        Self {
            position: (0, 0),
            direction: Direction::Up,
            program: Program::from_input(INPUT),
        }
    }

    fn run(&mut self, panels: &mut HashMap<Point, Color>) {
        while !self.program.is_finished() {
            let color = panels.entry(self.position).or_insert(0);
            self.program.input(*color);
            let outputs = self.program.run();
            *color = outputs[0];
            let turn = Turn::from_intcode(outputs[1]);
            self.direction = turn.turn(self.direction);
            let (x, y) = self.position;
            self.position = match self.direction {
                Direction::Up => (x, y - 1),
                Direction::Down => (x, y + 1),
                Direction::Left => (x - 1, y),
                Direction::Right => (x + 1, y),
            };
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Left,
    Right,
}

impl Turn {
    fn from_intcode(intcode: Int) -> Self {
        match intcode {
            0 => Self::Left,
            1 => Self::Right,
            _ => panic!("invalid intcode for turn"),
        }
    }

    fn turn(&self, current_direction: Direction) -> Direction {
        match (current_direction, self) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
        }
    }
}
