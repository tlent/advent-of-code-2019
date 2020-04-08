const INPUT: &str = include_str!("../input.txt");

mod intcode;

use intcode::Program;

fn main() {
    let program = Program::from_input(INPUT);
    println!("Part one: {}", solve_part_one(&program));
    println!("Part two: {}", solve_part_two(&program));
}

fn solve_part_one(program: &Program) -> i64 {
    let mut program = program.clone();
    program.input(1);
    let outputs = program.run();
    outputs[0]
}

fn solve_part_two(program: &Program) -> i64 {
    let mut program = program.clone();
    program.input(2);
    let outputs = program.run();
    outputs[0]
}
