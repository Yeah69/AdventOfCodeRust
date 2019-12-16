use crate::day_tasks;
use super::int_code;

pub struct Day05;

impl day_tasks::DayTasks for Day05 {
    fn day_number (&self) -> String {
        "05".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        iteration(input, 1).map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        iteration(input, 5).map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
    }
}

fn iteration(text_code: &String, input: i128) -> Option<i128> {
    let mut program = int_code::create_program(text_code);
    program.run_until_stopped();
    while program.get_status() == int_code::IntCodeProgramStatus::WaitingForInput {
        program.push_input(input);
        program.run_until_stopped();
    }
    program.get_last_output()
}
