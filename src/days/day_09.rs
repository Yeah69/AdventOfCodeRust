use crate::day_tasks;
use super::int_code;

pub struct Day09;

impl day_tasks::DayTasks for Day09 {
    fn day_number (&self) -> String {
        "09".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        task_impl(input, 1)
    }
    fn task_1 (&self, input: &String) -> String {
        task_impl(input, 2)
    }
}

fn task_impl (input: &String, input_number: i128) -> String {
    let mut program = int_code::create_program(int_code::parse_into_int_code(input));
    program.push_input(input_number);
    program.run_until_stopped();
    program.get_last_output().map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
}
