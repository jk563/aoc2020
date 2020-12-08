use std::collections::HashSet;
use std::str::FromStr;

#[derive(Debug)]
pub struct Computer {
    program: Vec<Instruction>,
    instruction_index: usize,
    accumulator: isize,
    instructions_completed: HashSet<usize>,
}

impl Computer {
    pub fn new() -> Computer {
        Computer {
            program: vec![],
            instruction_index: 0,
            accumulator: 0,
            instructions_completed: HashSet::new(),
        }
    }

    pub fn get_accumulator(&self) -> isize {
        self.accumulator
    }

    pub fn load(&mut self, instructions: &Vec<&str>) {
        self.program = instructions.iter().map(|x| x.parse().unwrap()).collect();
        self.accumulator = 0;
        self.instruction_index = 0;
        self.instructions_completed = HashSet::new();
    }

    pub fn run(&mut self) -> bool {
        loop {
            match self.program.get(self.instruction_index) {
                Some(_) => {
                    if self
                        .instructions_completed
                        .contains(&self.instruction_index)
                    {
                        return false;
                    }
                    self.instructions_completed.insert(self.instruction_index);
                    self.execute();
                }
                None => return true,
            }
        }
    }

    fn execute(&mut self) {
        let instruction = &self.program[self.instruction_index];
        match instruction.operation {
            Operation::NOOP => self.instruction_index += 1,
            Operation::ACC => {
                self.accumulator += instruction.operand;
                self.instruction_index += 1;
            }
            Operation::JUMP => {
                self.instruction_index =
                    (self.instruction_index as isize + instruction.operand) as usize;
            }
        }
    }
}

#[derive(Debug)]
struct Instruction {
    operation: Operation,
    operand: isize,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<&str> = s.split(" ").collect();
        if components.len() != 2 {
            return Err(InstructionParseError);
        }
        let operation = match components[0].parse() {
            Ok(operation) => operation,
            Err(_) => return Err(InstructionParseError),
        };
        let operand = match components[1].parse() {
            Ok(operand) => operand,
            Err(_) => return Err(InstructionParseError),
        };

        Ok(Instruction { operation, operand })
    }
}

#[derive(Debug)]
struct InstructionParseError;

#[derive(Debug)]
pub enum Operation {
    NOOP,
    ACC,
    JUMP,
}

impl FromStr for Operation {
    type Err = OperationParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nop" => Ok(Operation::NOOP),
            "acc" => Ok(Operation::ACC),
            "jmp" => Ok(Operation::JUMP),
            _ => Err(OperationParseError),
        }
    }
}

#[derive(Debug)]
pub struct OperationParseError;
