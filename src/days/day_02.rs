use crate::day_tasks;
use super::int_code;

pub struct Day02;

impl day_tasks::DayTasks for Day02 {
    fn day_number (&self) -> String {
        "02".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        iteration(input, 12, 2).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let (noun, verb) = find_output(input, 19690720);
        let result = 100 * noun + verb;
        result.to_string()
    }
}

fn find_output (text_code: &String, seeked_output: i128) -> (i128, i128) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = iteration(text_code, noun, verb);
            if result == seeked_output {
                return (noun, verb);
            }
        }
    }
    (-1, -1)
}

fn iteration(text_code: &String, noun: i128, verb: i128) -> i128 {
    let mut program = int_code::create_program(text_code);
    program.day_02_initialize(noun, verb);
    program.run_until_stopped();
    program.day_02_result()
}
