use std::collections::VecDeque;

pub struct IntCodeProgram {
    int_code: Vec<i128>,
    instruction_pointer: usize,
    input: VecDeque<i128>,
    output: Vec<i128>,
    status: IntCodeProgramStatus,
    relative_base: usize
}

#[derive(Eq, PartialEq, Clone)]
pub enum IntCodeProgramStatus {
    Ready,
    WaitingForInput,
    Halt
}

#[derive(Eq, PartialEq, Clone)]
enum ParameterMode {
    Position,
    Immediate,
    Relative
}

#[derive(Eq, PartialEq, Clone)]
enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LesserThan,
    Equals,
    AdjustRelativeBase,
    Halt
}

impl IntCodeProgram {
    pub fn run_until_stopped (&mut self) {
        while self.status == IntCodeProgramStatus::Ready {
            self.step();
        }
    }
    
    pub fn run_until_next_output (&mut self) -> Option<i128> {
        let mut current_output = None;
        while self.status == IntCodeProgramStatus::Ready && current_output.is_none() {
            current_output = self.step();
        }
        current_output
    }

    pub fn get_last_output (&self) -> Option<i128> {
        self.output.last().map(|i| i.clone())
    }

    pub fn get_status (&self) -> IntCodeProgramStatus {
        self.status.clone()
    }

    pub fn push_input (&mut self, input: i128) {
        self.input.push_back(input);
        if self.status == IntCodeProgramStatus::WaitingForInput {
            self.status = IntCodeProgramStatus::Ready }
    }

    pub fn day_02_initialize (&mut self, noun: i128, verb: i128) {
        self.int_code[1] = noun;
        self.int_code[2] = verb;
    }

    pub fn day_02_result (&self) -> i128 {
        self.int_code[0]
    }

    fn allocate_space_if_necessary (&mut self, until_position: usize)
    {
        if until_position >= self.int_code.len() {
            for _ in self.int_code.len()..=until_position {
                self.int_code.push(0);
            }
        }
    }

    fn get_int_code(&mut self, position: usize) -> i128 {
        self.allocate_space_if_necessary(position);
        self.int_code[position]
    }

    fn set_int_code(&mut self, position: usize, value: i128) {
        self.allocate_space_if_necessary(position);
        self.int_code[position] = value;
    }

    fn fetch_operator (&mut self, initial_position: usize, parameter_mode: ParameterMode) -> i128 {
        let operator = self.get_int_code(initial_position);
        match parameter_mode {
            ParameterMode::Position => self.get_int_code(operator as usize),
            ParameterMode::Immediate => operator,
            ParameterMode::Relative => self.get_int_code(operator as usize + self.relative_base)
        }
    }

    fn get_parameter_position (&mut self, initial_position: usize, parameter_mode: ParameterMode) -> usize {
        let operator = self.get_int_code(initial_position);
        match parameter_mode {
            ParameterMode::Position => operator as usize,
            ParameterMode::Relative => operator as usize + self.relative_base,
            _ => operator as usize
        }
    }

    fn step (&mut self) -> Option<i128> {
        

        fn parse_op_code (op_code: i128) -> (Instruction, ParameterMode, ParameterMode, ParameterMode) {
            fn parse_parameter_mode (parameter_value: i128) -> ParameterMode {
                match parameter_value {
                    0 => ParameterMode::Position,
                    1 => ParameterMode::Immediate,
                    2 => ParameterMode::Relative,
                    _ => ParameterMode::Position
                }
            }
            (match op_code % 100 {
                1 => Instruction::Add,
                2 => Instruction::Multiply,
                3 => Instruction::Input,
                4 => Instruction::Output,
                5 => Instruction::JumpIfTrue,
                6 => Instruction::JumpIfFalse,
                7 => Instruction::LesserThan,
                8 => Instruction::Equals,
                9 => Instruction::AdjustRelativeBase,
                99 | _ => Instruction::Halt,
            },
            parse_parameter_mode((op_code /    100) % 10),
            parse_parameter_mode((op_code /  1_000) % 10),
            parse_parameter_mode((op_code / 10_000) % 10))
        }

        let (instruction, mode_0, mode_1, mode_2) = parse_op_code(self.get_int_code(self.instruction_pointer));
    
        match instruction {
            Instruction::Add | Instruction::Multiply | Instruction::JumpIfTrue | Instruction::JumpIfFalse | Instruction::LesserThan | Instruction::Equals => {
                let operator_0 = self.fetch_operator(self.instruction_pointer + 1, mode_0);
                let operator_1 = self.fetch_operator(self.instruction_pointer + 2, mode_1);
    
                match instruction {
                    Instruction::JumpIfTrue | Instruction::JumpIfFalse => {
                        if instruction == Instruction::JumpIfTrue && operator_0 != 0 || instruction == Instruction::JumpIfFalse && operator_0 == 0 
                        { self.instruction_pointer = operator_1 as usize } 
                        else { self.instruction_pointer = self.instruction_pointer + 3 }
                    }
                    _ => {
                        let target_index = self.get_parameter_position(self.instruction_pointer + 3, mode_2);
                        let result = match instruction {
                            Instruction::Add => operator_0 + operator_1,
                            Instruction::Multiply => operator_0 * operator_1,
                            Instruction::LesserThan => if operator_0 < operator_1 { 1 } else { 0 },
                            _ => if operator_0 == operator_1 { 1 } else { 0 } // Should be op code Instruction::Equals
                        };
                        self.set_int_code(target_index as usize, result);
                        
                        self.instruction_pointer = self.instruction_pointer + 4;}
                }
                None
            }
            Instruction::Input => {
                if self.input.len() > 0 {
                    let target_index = self.get_parameter_position(self.instruction_pointer + 1, mode_0);
                    let input = self.input.pop_front().unwrap();
                    self.set_int_code(target_index as usize, input);
                    self.instruction_pointer = self.instruction_pointer + 2;
                }
                else {
                    self.status = IntCodeProgramStatus::WaitingForInput;
                }
                None
            }
            Instruction::Output => {
                let operator_0 = self.fetch_operator(self.instruction_pointer + 1, mode_0);

                self.output.push(operator_0);
                self.instruction_pointer = self.instruction_pointer + 2;
                
                Some(operator_0)
            },
            Instruction::AdjustRelativeBase => {
                let operator_0 = self.fetch_operator(self.instruction_pointer + 1, mode_0);

                self.relative_base = self.relative_base + operator_0 as usize;
                self.instruction_pointer = self.instruction_pointer + 2;
                
                None
            },
            Instruction::Halt => {
                self.status = IntCodeProgramStatus::Halt;
                self.instruction_pointer = self.int_code.len();
                None
            }
        }
    }
}

pub fn create_program (int_code: Vec<i128>) -> IntCodeProgram {
    IntCodeProgram { 
        int_code: int_code, 
        instruction_pointer: 0,
        input: VecDeque::new(),
        output: Vec::new(), 
        status: IntCodeProgramStatus::Ready,
        relative_base: 0 }
}

pub fn parse_into_int_code (input: &String) -> Vec<i128>{
    input.split(',').map(|text_number| text_number.parse::<i128>().unwrap()).collect()
}