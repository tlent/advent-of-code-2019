use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_char(c: char) -> Self {
        use Direction::*;
        match c {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            _ => panic!("Invalid char"),
        }
    }
}

type Steps = u32;
type Path = Vec<(Direction, Steps)>;
type Position = (i32, i32);

fn main() {
    let (path_a, path_b) = parse_input();
    println!("Part one: {}", solve_part_one(&path_a, &path_b));
    println!("Part two: {}", solve_part_two(&path_a, &path_b));
}

fn parse_input() -> (Path, Path) {
    let mut lines = INPUT.lines();
    let path_a = parse_path(lines.next().unwrap());
    let path_b = parse_path(lines.next().unwrap());
    (path_a, path_b)
}

fn parse_path(path: &str) -> Path {
    path.split(',')
        .map(|step| {
            let direction = Direction::from_char(step.chars().next().unwrap());
            let steps = step[1..].parse().unwrap();
            (direction, steps)
        })
        .collect()
}

fn trace_path(path: &Path) -> Vec<Position> {
    let mut positions = vec![];
    let mut position = (0, 0);
    use Direction::*;
    for &(direction, steps) in path {
        for _ in 0..steps {
            let (x, y) = position;
            position = match direction {
                Up => (x, y + 1),
                Down => (x, y - 1),
                Right => (x + 1, y),
                Left => (x - 1, y),
            };
            positions.push(position);
        }
    }
    positions
}

fn manhattan_distance_from_origin(p: &Position) -> u32 {
    let (x, y) = p;
    x.abs() as u32 + y.abs() as u32
}

fn solve_part_one(path_a: &Path, path_b: &Path) -> u32 {
    let a_positions: HashSet<_> = trace_path(path_a).into_iter().collect();
    let b_positions: HashSet<_> = trace_path(path_b).into_iter().collect();
    let common_positions = a_positions.intersection(&b_positions);
    common_positions
        .map(manhattan_distance_from_origin)
        .min()
        .unwrap()
}

fn steps_to_position(path_trace: &[Position], position: Position) -> u32 {
    path_trace.iter().position(|&p| p == position).unwrap() as u32 + 1
}

fn solve_part_two(path_a: &Path, path_b: &Path) -> u32 {
    let a_positions = trace_path(path_a);
    let b_positions = trace_path(path_b);
    let a_position_set = a_positions.iter().collect::<HashSet<_>>();
    let b_position_set = b_positions.iter().collect::<HashSet<_>>();
    let common_positions = a_position_set.intersection(&b_position_set);
    common_positions
        .map(|&&pos| steps_to_position(&a_positions, pos) + steps_to_position(&b_positions, pos))
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const PART_ONE_SOLUTION: u32 = 5357;
    const PART_TWO_SOLUTION: u32 = 101956;

    #[test]
    fn part_one() {
        let (path_a, path_b) = parse_input();
        assert_eq!(solve_part_one(&path_a, &path_b), PART_ONE_SOLUTION);
    }

    #[test]
    fn part_two() {
        let (path_a, path_b) = parse_input();
        assert_eq!(solve_part_two(&path_a, &path_b), PART_TWO_SOLUTION);
    }
}
