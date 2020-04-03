const INPUT: &str = include_str!("../input.txt");

const PART_TWO_GOAL_OUTPUT: u32 = 19_690_720;

fn main() {
    let program = parse_input();
    println!("Part one: {}", solve_part_one(&program));
    println!("Part two: {}", solve_part_two(&program));
}

fn parse_input() -> Vec<u32> {
    INPUT
        .split(",")
        .map(|value| value.trim().parse().unwrap())
        .collect()
}

fn solve_part_one(program: &[u32]) -> u32 {
    let mut program = program.to_vec();
    program[1] = 12;
    program[2] = 2;
    run_program(&mut program);
    program[0]
}

fn solve_part_two(program: &[u32]) -> u32 {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.to_vec();
            program[1] = noun;
            program[2] = verb;
            run_program(&mut program);
            if program[0] == PART_TWO_GOAL_OUTPUT {
                return 100 * noun + verb;
            }
        }
    }
    panic!("No solution found")
}

fn run_program(program: &mut [u32]) {
    let mut position = 0;
    loop {
        let opcode = program[position];
        let a_index = program[position + 1] as usize;
        let b_index = program[position + 2] as usize;
        let output_index = program[position + 3] as usize;
        program[output_index] = match opcode {
            1 => program[a_index] + program[b_index],
            2 => program[a_index] * program[b_index],
            99 => return,
            _ => panic!("Unknown opcode"),
        };
        position += 4;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const PART_ONE_SOLUTION: u32 = 4_462_686;
    const PART_TWO_SOLUTION: u32 = 5936;

    #[test]
    fn part_one() {
        let program = parse_input();
        assert_eq!(solve_part_one(&program), PART_ONE_SOLUTION);
    }

    #[test]
    fn part_two() {
        let program = parse_input();
        assert_eq!(solve_part_two(&program), PART_TWO_SOLUTION);
    }
}
