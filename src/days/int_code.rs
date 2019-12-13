use std::collections::VecDeque;

pub struct IntCodeProgram {
    int_code: Vec<i32>,
    instruction_pointer: usize,
    input: VecDeque<i32>,
    output: Vec<i32>,
    status: IntCodeProgramStatus
}

#[derive(Eq, PartialEq,Clone)]
pub enum IntCodeProgramStatus {
    Ready,
    WaitingForInput,
    Halt
}

impl IntCodeProgram {
    pub fn run_until_stopped (&mut self) {
        while self.status == IntCodeProgramStatus::Ready {
            self.step();
        }
    }
    
    pub fn run_until_next_output (&mut self) -> Option<i32> {
        let mut current_output = None;
        while self.status == IntCodeProgramStatus::Ready && current_output.is_none() {
            current_output = self.step();
        }
        current_output
    }

    pub fn get_last_output (&self) -> Option<i32> {
        self.output.last().map(|i| i.clone())
    }

    pub fn get_status (&self) -> IntCodeProgramStatus {
        self.status.clone()
    }

    pub fn push_input (&mut self, input: i32) {
        self.input.push_back(input);
        if self.status == IntCodeProgramStatus::WaitingForInput {
            self.status = IntCodeProgramStatus::Ready }
    }

    pub fn day_02_initialize (&mut self, noun: i32, verb: i32) {
        self.int_code[1] = noun;
        self.int_code[2] = verb;
    }

    pub fn day_02_result (&self) -> i32 {
        self.int_code[0]
    }

    fn step (&mut self) -> Option<i32> {
        let full_op_code = self.int_code[self.instruction_pointer];
        let op_code = full_op_code % 100;
    
        match op_code {
            1 | 2 | 5 | 6 | 7 | 8 => {
                let position_mode_0 = (full_op_code /    100) % 10 == 0;
                let position_mode_1 = (full_op_code /  1_000) % 10 == 0;
        
                let operator_0 = self.int_code[self.instruction_pointer + 1];
                let operator_0 = if position_mode_0 { self.int_code[operator_0 as usize] } else { operator_0 };
                let operator_1 = self.int_code[self.instruction_pointer + 2];
                let operator_1 = if position_mode_1 { self.int_code[operator_1 as usize] } else { operator_1 };
    
                match op_code {
                    5 | 6 => {
                        if op_code == 5 && operator_0 != 0 || op_code == 6 && operator_0 == 0 { self.instruction_pointer = operator_1 as usize } 
                        else { self.instruction_pointer = self.instruction_pointer + 3 }
                    }
                    _ => {
                        let target_index = self.int_code[self.instruction_pointer + 3];
                        let result = match op_code {
                            1 => operator_0 + operator_1,
                            2 => operator_0 * operator_1,
                            7 => if operator_0 < operator_1 { 1 } else { 0 },
                            _ => if operator_0 == operator_1 { 1 } else { 0 } // Should be op code 8
                        };
                        self.int_code[target_index as usize] = result;
                        
                        self.instruction_pointer = self.instruction_pointer + 4;}
                }
                None
            }
            3 => {
                if self.input.len() > 0 {
                    let target_index = self.int_code[self.instruction_pointer + 1];
                    self.int_code[target_index as usize] = self.input.pop_front().unwrap();
                    self.instruction_pointer = self.instruction_pointer + 2;
                }
                else {
                    self.status = IntCodeProgramStatus::WaitingForInput;
                }
                None
            }
            4 => {
                let position_mode_0 = (full_op_code /    100) % 10 == 0;
                
                let operator_0 = self.int_code[self.instruction_pointer + 1];
                let operator_0 = if position_mode_0 { self.int_code[operator_0 as usize] } else { operator_0 };

                self.output.push(operator_0);
                self.instruction_pointer = self.instruction_pointer + 2;
                
                Some(operator_0)
            }
            99 => {
                self.status = IntCodeProgramStatus::Halt;
                self.instruction_pointer = self.int_code.len();
                None
            }
            _ => None
        }
    }
}

pub fn create_program (int_code: Vec<i32>) -> IntCodeProgram {
    IntCodeProgram { 
        int_code: int_code, 
        instruction_pointer: 0,
        input: VecDeque::new(),
        output: Vec::new(), 
        status: IntCodeProgramStatus::Ready }
}

pub fn parse_into_int_code (input: &String) -> Vec<i32>{
    input.split(',').map(|text_number| text_number.parse::<i32>().unwrap()).collect()
}