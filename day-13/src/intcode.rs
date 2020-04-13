use std::fmt;

pub type Int = i64;

#[derive(Debug, Clone)]
pub struct Program {
    state: ProgramState,
    instruction_pointer: usize,
    relative_base: Int,
    memory: Vec<Int>,
    inputs: Vec<Int>,
    outputs: Vec<Int>,
}

impl Program {
    pub fn from_input(input: &str) -> Self {
        let memory = input
            .trim()
            .split(",")
            .map(|s| s.parse().unwrap())
            .collect();
        Self {
            state: ProgramState::InitialState,
            instruction_pointer: 0,
            relative_base: 0,
            memory,
            inputs: vec![],
            outputs: vec![],
        }
    }

    pub fn is_finished(&self) -> bool {
        self.state == ProgramState::Halted
    }

    pub fn run(&mut self, inputs: &[Int]) -> Vec<Int> {
        self.outputs.clear();
        self.inputs = inputs.to_vec();
        self.state = ProgramState::Running;
        while self.state == ProgramState::Running {
            let opcode = self.memory[self.instruction_pointer];
            let instruction = self.parse_instruction(opcode);
            instruction.run(self);
        }
        self.outputs.clone()
    }

    pub fn read_from_memory(&self, address: usize) -> Int {
        self.memory.get(address).copied().unwrap_or(0)
    }

    pub fn write_to_memory(&mut self, address: usize, value: Int) {
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value;
    }

    fn parse_instruction(&self, opcode: Int) -> Box<dyn Instruction> {
        let ip = self.instruction_pointer;
        match opcode % 100 {
            1 => Box::new(Add::new(&self.memory[ip..ip + 4])),
            2 => Box::new(Mult::new(&self.memory[ip..ip + 4])),
            3 => Box::new(Input::new(&self.memory[ip..ip + 2])),
            4 => Box::new(Output::new(&self.memory[ip..ip + 2])),
            5 => Box::new(JumpIfTrue::new(&self.memory[ip..ip + 3])),
            6 => Box::new(JumpIfFalse::new(&self.memory[ip..ip + 3])),
            7 => Box::new(LessThan::new(&self.memory[ip..ip + 4])),
            8 => Box::new(Equals::new(&self.memory[ip..ip + 4])),
            9 => Box::new(RelativeBaseOffset::new(&self.memory[ip..ip + 2])),
            99 => Box::new(Halt),
            _ => panic!("Invalid operation"),
        }
    }

    fn evaluate(&self, parameter: Parameter) -> Int {
        use ParameterMode::*;
        let Parameter { parameter, mode } = parameter;
        match mode {
            Position => self.read_from_memory(parameter as usize),
            Immediate => parameter,
            Relative => {
                let index = (parameter + self.relative_base) as usize;
                self.read_from_memory(index)
            }
        }
    }

    fn evaluate_address(&self, parameter: Parameter) -> usize {
        use ParameterMode::*;
        let Parameter { parameter, mode } = parameter;
        let address = match mode {
            Position => parameter,
            Relative => parameter + self.relative_base,
            Immediate => panic!("can't interpret immediate mode parameter as address"),
        };
        address as usize
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
    a: Parameter,
    b: Parameter,
    address: Parameter,
}

impl Add {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 3);
        let [a, b, address] = [parameters[0], parameters[1], parameters[2]];
        Self { a, b, address }
    }
}

impl Instruction for Add {
    fn run(&self, program: &mut Program) {
        let a = program.evaluate(self.a);
        let b = program.evaluate(self.b);
        let address = program.evaluate_address(self.address);
        program.write_to_memory(address, a + b);
        program.instruction_pointer += 4;
    }
}

#[derive(Debug)]
struct Mult {
    a: Parameter,
    b: Parameter,
    address: Parameter,
}

impl Mult {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 3);
        let [a, b, address] = [parameters[0], parameters[1], parameters[2]];
        Self { a, b, address }
    }
}

impl Instruction for Mult {
    fn run(&self, program: &mut Program) {
        let a = program.evaluate(self.a);
        let b = program.evaluate(self.b);
        let address = program.evaluate_address(self.address);
        program.write_to_memory(address, a * b);
        program.instruction_pointer += 4;
    }
}

#[derive(Debug)]
struct Input {
    address: Parameter,
}

impl Input {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 1);
        let address = parameters[0];
        Self { address }
    }
}

impl Instruction for Input {
    fn run(&self, program: &mut Program) {
        if program.inputs.is_empty() {
            program.state = ProgramState::WaitingForInput;
            return;
        }
        let value = program.inputs.remove(0);
        let address = program.evaluate_address(self.address);
        program.write_to_memory(address, value);
        program.instruction_pointer += 2;
    }
}

#[derive(Debug)]
struct Output {
    value: Parameter,
}

impl Output {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 1);
        let value = parameters[0];
        Self { value }
    }
}

impl Instruction for Output {
    fn run(&self, program: &mut Program) {
        let value = program.evaluate(self.value);
        program.outputs.push(value);
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
    value: Parameter,
    address: Parameter,
}

impl JumpIfTrue {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 2);
        let [value, address] = [parameters[0], parameters[1]];
        Self { value, address }
    }
}

impl Instruction for JumpIfTrue {
    fn run(&self, program: &mut Program) {
        let value = program.evaluate(self.value);
        let address = program.evaluate(self.address) as usize;
        program.instruction_pointer = if value != 0 {
            address
        } else {
            program.instruction_pointer + 3
        }
    }
}

#[derive(Debug)]
struct JumpIfFalse {
    value: Parameter,
    address: Parameter,
}

impl JumpIfFalse {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 2);
        let [value, address] = [parameters[0], parameters[1]];
        Self { value, address }
    }
}

impl Instruction for JumpIfFalse {
    fn run(&self, program: &mut Program) {
        let value = program.evaluate(self.value);
        let address = program.evaluate(self.address) as usize;
        program.instruction_pointer = if value == 0 {
            address
        } else {
            program.instruction_pointer + 3
        }
    }
}

#[derive(Debug)]
struct LessThan {
    a: Parameter,
    b: Parameter,
    address: Parameter,
}

impl LessThan {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 3);
        let [a, b, address] = [parameters[0], parameters[1], parameters[2]];
        Self { a, b, address }
    }
}

impl Instruction for LessThan {
    fn run(&self, program: &mut Program) {
        let a = program.evaluate(self.a);
        let b = program.evaluate(self.b);
        let address = program.evaluate_address(self.address);
        let value = if a < b { 1 } else { 0 };
        program.write_to_memory(address, value);
        program.instruction_pointer += 4;
    }
}

#[derive(Debug)]
struct Equals {
    a: Parameter,
    b: Parameter,
    address: Parameter,
}

impl Equals {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 3);
        let [a, b, address] = [parameters[0], parameters[1], parameters[2]];
        Self { a, b, address }
    }
}

impl Instruction for Equals {
    fn run(&self, program: &mut Program) {
        let a = program.evaluate(self.a);
        let b = program.evaluate(self.b);
        let address = program.evaluate_address(self.address);
        let value = if a == b { 1 } else { 0 };
        program.write_to_memory(address, value);
        program.instruction_pointer += 4;
    }
}

#[derive(Debug)]
struct RelativeBaseOffset {
    change: Parameter,
}

impl RelativeBaseOffset {
    fn new(instruction: &[Int]) -> Self {
        let parameters = Parameter::from_instruction(instruction, 1);
        let change = parameters[0];
        Self { change }
    }
}

impl Instruction for RelativeBaseOffset {
    fn run(&self, program: &mut Program) {
        let change = program.evaluate(self.change);
        program.relative_base += change;
        program.instruction_pointer += 2;
    }
}

#[derive(Debug, Clone, Copy)]
struct Parameter {
    parameter: Int,
    mode: ParameterMode,
}

impl Parameter {
    fn from_instruction(instruction: &[Int], count: usize) -> Vec<Self> {
        let modes = ParameterMode::from_opcode(instruction[0], count);
        let parameters = &instruction[1..count + 1];
        parameters
            .iter()
            .zip(modes)
            .map(|(&parameter, mode)| Parameter { parameter, mode })
            .collect()
    }
}

#[derive(Debug, Clone, Copy)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn from_opcode(mut opcode: Int, count: usize) -> Vec<Self> {
        let mut modes = Vec::with_capacity(count);
        opcode /= 100;
        for _ in 0..count {
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
            2 => Self::Relative,
            mode => panic!("Invalid parameter mode: {}", mode),
        }
    }
}
