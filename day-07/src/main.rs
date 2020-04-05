const INPUT: &str = include_str!("../input.txt");

use intcode::Program;

fn main() {
    println!("Part one: {}", solve_part_one());
    println!("Part two: {}", solve_part_two());
}

fn solve_part_one() -> i32 {
    permutations(&[0, 1, 2, 3, 4])
        .iter()
        .map(|phases| {
            let mut input_signal = 0;
            for &phase_setting in phases {
                let mut program = Program::from_input(INPUT);
                program.set_inputs(&[phase_setting, input_signal]);
                let output = program.run();
                input_signal = output[0];
            }
            input_signal
        })
        .max()
        .unwrap()
}

// https://en.wikipedia.org/wiki/Heap%27s_algorithm
fn permutations(values: &[i32]) -> Vec<Vec<i32>> {
    let mut permutations = vec![];
    let n = values.len();
    let mut a = values.to_vec();
    let mut c = vec![0; n];
    permutations.push(a.clone());
    let mut i = 0;
    while i < n {
        if c[i] < i {
            if i % 2 == 0 {
                a.swap(0, i);
            } else {
                a.swap(c[i], i);
            }
            permutations.push(a.clone());
            c[i] += 1;
            i = 0;
        } else {
            c[i] = 0;
            i += 1;
        }
    }
    permutations
}

fn solve_part_two() -> i32 {
    let initial_program = Program::from_input(INPUT);
    permutations(&[5, 6, 7, 8, 9])
        .iter()
        .map(|phases| {
            let mut programs = vec![initial_program.clone(); 5];
            for (program, &phase) in programs.iter_mut().zip(phases) {
                program.input(phase);
            }
            let mut input_signal = 0;
            while !programs[4].is_finished() {
                for program in programs.iter_mut() {
                    program.input(input_signal);
                    let output = program.run();
                    input_signal = output[output.len() - 1];
                }
            }
            input_signal
        })
        .max()
        .unwrap()
}

mod intcode {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct Program {
        instruction_pointer: usize,
        memory: Vec<i32>,
        state: ProgramState,
        inputs: Vec<i32>,
        outputs: Vec<i32>,
    }

    impl Program {
        pub fn from_input(input: &str) -> Self {
            let memory = input
                .trim()
                .split(",")
                .map(|s| s.parse().unwrap())
                .collect();
            Self {
                memory,
                instruction_pointer: 0,
                state: ProgramState::InitialState,
                inputs: vec![],
                outputs: vec![],
            }
        }

        pub fn set_inputs(&mut self, inputs: &[i32]) {
            self.inputs = inputs.to_vec();
        }

        pub fn input(&mut self, input: i32) {
            self.inputs.push(input);
        }

        pub fn is_finished(&self) -> bool {
            self.state == ProgramState::Halted
        }

        pub fn run(&mut self) -> Vec<i32> {
            self.state = ProgramState::Running;
            while self.state == ProgramState::Running {
                let opcode = self.memory[self.instruction_pointer];
                let instruction = self.load_instruction(opcode);
                instruction.run(self);
            }
            self.outputs.clone()
        }

        fn load_instruction(&self, opcode: i32) -> Box<dyn Instruction> {
            match opcode % 100 {
                1 => Box::new(Add::new(&self.memory, self.instruction_pointer)),
                2 => Box::new(Mult::new(&self.memory, self.instruction_pointer)),
                3 => Box::new(Input::new(&self.memory, self.instruction_pointer)),
                4 => Box::new(Output::new(&self.memory, self.instruction_pointer)),
                5 => Box::new(JumpIfTrue::new(&self.memory, self.instruction_pointer)),
                6 => Box::new(JumpIfFalse::new(&self.memory, self.instruction_pointer)),
                7 => Box::new(LessThan::new(&self.memory, self.instruction_pointer)),
                8 => Box::new(Equals::new(&self.memory, self.instruction_pointer)),
                99 => Box::new(Halt),
                _ => panic!("Invalid operation"),
            }
        }
    }

    fn evaluate_parameters(memory: &[i32], instruction_address: usize, values: &mut [i32]) {
        let opcode = memory[instruction_address];
        let modes = ParameterMode::modes_from_opcode(opcode, values.len());
        let parameters_address = instruction_address + 1;
        use ParameterMode::*;
        for i in 0..values.len() {
            let address = parameters_address + i;
            let parameter = memory[address];
            let value = match modes[i] {
                Position => {
                    let index = parameter as usize;
                    memory[index]
                }
                Immediate => parameter,
            };
            values[i] = value;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum ProgramState {
        InitialState,
        Running,
        WaitingForInput,
        Halted,
    }

    trait Instruction: fmt::Debug {
        fn run(&self, program: &mut Program);
    }

    #[derive(Debug)]
    struct Add {
        a: i32,
        b: i32,
        address: usize,
    }

    impl Add {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 2];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let a = parameters[0];
            let b = parameters[1];
            let address = memory[instruction_address + 3] as usize;
            Self { a, b, address }
        }
    }

    impl Instruction for Add {
        fn run(&self, program: &mut Program) {
            program.memory[self.address] = self.a + self.b;
            program.instruction_pointer += 4;
        }
    }

    #[derive(Debug)]
    struct Mult {
        a: i32,
        b: i32,
        address: usize,
    }

    impl Mult {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 2];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let a = parameters[0];
            let b = parameters[1];
            let address = memory[instruction_address + 3] as usize;
            Self { a, b, address }
        }
    }

    impl Instruction for Mult {
        fn run(&self, program: &mut Program) {
            program.memory[self.address] = self.a * self.b;
            program.instruction_pointer += 4;
        }
    }

    #[derive(Debug)]
    struct Input {
        address: usize,
    }

    impl Input {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let address = memory[instruction_address + 1] as usize;
            Self { address }
        }
    }

    impl Instruction for Input {
        fn run(&self, program: &mut Program) {
            if program.inputs.is_empty() {
                program.state = ProgramState::WaitingForInput;
                return;
            }
            program.memory[self.address] = program.inputs.remove(0);
            program.instruction_pointer += 2;
        }
    }

    #[derive(Debug)]
    struct Output {
        value: i32,
    }

    impl Output {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 1];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let value = parameters[0];
            Self { value }
        }
    }

    impl Instruction for Output {
        fn run(&self, program: &mut Program) {
            program.outputs.push(self.value);
            program.instruction_pointer += 2;
        }
    }

    #[derive(Debug)]
    struct Halt;

    impl Instruction for Halt {
        fn run(&self, program: &mut Program) {
            program.state = ProgramState::Halted;
        }
    }

    #[derive(Debug)]
    struct JumpIfTrue {
        value: i32,
        address: usize,
    }

    impl JumpIfTrue {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 2];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let value = parameters[0];
            let address = parameters[1] as usize;
            Self { value, address }
        }
    }

    impl Instruction for JumpIfTrue {
        fn run(&self, program: &mut Program) {
            program.instruction_pointer = if self.value != 0 {
                self.address
            } else {
                program.instruction_pointer + 3
            }
        }
    }

    #[derive(Debug)]
    struct JumpIfFalse {
        value: i32,
        address: usize,
    }

    impl JumpIfFalse {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 2];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let value = parameters[0];
            let address = parameters[1] as usize;
            Self { value, address }
        }
    }

    impl Instruction for JumpIfFalse {
        fn run(&self, program: &mut Program) {
            program.instruction_pointer = if self.value == 0 {
                self.address
            } else {
                program.instruction_pointer + 3
            }
        }
    }

    #[derive(Debug)]
    struct LessThan {
        a: i32,
        b: i32,
        address: usize,
    }

    impl LessThan {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 2];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let a = parameters[0];
            let b = parameters[1];
            let address = memory[instruction_address + 3] as usize;
            Self { a, b, address }
        }
    }

    impl Instruction for LessThan {
        fn run(&self, program: &mut Program) {
            program.memory[self.address] = if self.a < self.b { 1 } else { 0 };
            program.instruction_pointer += 4;
        }
    }

    #[derive(Debug)]
    struct Equals {
        a: i32,
        b: i32,
        address: usize,
    }

    impl Equals {
        fn new(memory: &[i32], instruction_address: usize) -> Self {
            let mut parameters = [0; 2];
            evaluate_parameters(memory, instruction_address, &mut parameters);
            let a = parameters[0];
            let b = parameters[1];
            let address = memory[instruction_address + 3] as usize;
            Self { a, b, address }
        }
    }

    impl Instruction for Equals {
        fn run(&self, program: &mut Program) {
            program.memory[self.address] = if self.a == self.b { 1 } else { 0 };
            program.instruction_pointer += 4;
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum ParameterMode {
        Position,
        Immediate,
    }

    impl ParameterMode {
        fn modes_from_opcode(mut opcode: i32, parameter_count: usize) -> Vec<Self> {
            let mut modes = Vec::with_capacity(parameter_count);
            opcode /= 100;
            for _ in 0..parameter_count {
                let digit = (opcode % 10) as u8;
                modes.push(Self::from_opcode_digit(digit));
                opcode /= 10;
            }
            modes
        }

        fn from_opcode_digit(digit: u8) -> Self {
            match digit % 10 {
                0 => Self::Position,
                1 => Self::Immediate,
                mode => panic!("Invalid parameter mode: {}", mode),
            }
        }
    }
}
