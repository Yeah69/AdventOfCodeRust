use crate::day_tasks;
use super::int_code;

pub struct Day02;

impl day_tasks::DayTasks for Day02 {
    fn day_number (&self) -> String {
        "02".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        iteration(&numbers, 12, 2).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        let (noun, verb) = find_output(&numbers, 19690720);
        let result = 100 * noun + verb;
        result.to_string()
    }
}

fn find_output (initial_state: &Vec<i32>, seeked_output: i32) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = iteration(&(initial_state.to_vec()), noun, verb);
            if result == seeked_output {
                return (noun, verb);
            }
        }
    }
    (-1, -1)
}

fn iteration(initial_state: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut program = int_code::create_program(initial_state.to_vec());
    program.day_02_initialize(noun, verb);
    program.run_until_stopped();
    program.day_02_result()
}
