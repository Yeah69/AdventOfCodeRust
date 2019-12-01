use super::int_code;
use std::io;

pub struct AsciiCodeProgram {
    int_code_program: int_code::IntCodeProgram
}

impl AsciiCodeProgram {
    pub fn run_script (&mut self, script: &String) {
        println!("{}", script);
        for c in script.chars() {
            self.int_code_program.push_input(c as i128)
        }
        loop {
            let mut output_occured = false;
            while let Some(c_as_i) = self.int_code_program.run_until_next_output() {
                output_occured = true;
                if c_as_i >= 128 {
                    print!("{}", c_as_i)
                }
                else {
                    print!("{}", c_as_i as u8 as char)
                }
            }
            if output_occured { println!(); }

            match self.int_code_program.get_status() {
                int_code::IntCodeProgramStatus::WaitingForInput => {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            for c in input.chars() {
                                self.int_code_program.push_input(c as i128)
                            }
                        }
                        Err(error) => println!("error: {}", error),
                    }
                },
                int_code::IntCodeProgramStatus::Halt => {
                    break;
                },
                _ => ()
            }
        }
    }

    pub fn push_script_as_input (&mut self, script: &String) {
        for c in script.chars() {
            self.int_code_program.push_input(c as i128);
        }
    }

    pub fn run_and_prompt_user_input (&mut self, user_input_mapping: &dyn Fn(&String) -> String) {
        loop {
            let mut output_occured = false;
            while let Some(c_as_i) = self.int_code_program.run_until_next_output() {
                output_occured = true;
                if c_as_i >= 128 {
                    print!("{}", c_as_i)
                }
                else {
                    print!("{}", c_as_i as u8 as char)
                }
            }
            if output_occured { println!(); }

            match self.int_code_program.get_status() {
                int_code::IntCodeProgramStatus::WaitingForInput => {
                    let mut input = String::new();
                    match io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            let mapped_input = user_input_mapping(&input);
                            println!("{}", mapped_input);
                            for c in mapped_input.chars() {
                                self.int_code_program.push_input(c as i128)
                            }
                        }
                        Err(error) => println!("error: {}", error),
                    }
                },
                int_code::IntCodeProgramStatus::Halt => {
                    break;
                },
                _ => ()
            }
        }
    }
}

pub fn create_program (text_code: &String) -> AsciiCodeProgram {
    AsciiCodeProgram { int_code_program: int_code::create_program(text_code) }
}