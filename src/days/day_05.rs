use crate::day_tasks;
use super::int_code;

pub struct Day05;

impl day_tasks::DayTasks for Day05 {
    fn day_number (&self) -> String {
        "05".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        iteration(&numbers, 1).map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        iteration(&numbers, 5).map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
    }
}

fn iteration(initial_state: &Vec<i32>, input: i32) -> Option<i32> {
    let mut numbers = initial_state.to_vec();
    let mut last_output: Option<i32> = None;
    let mut current_index: i32 = 0;
    while current_index != -1 && (current_index as usize) < numbers.len() {
        let (next_index, output_maybe, _) = int_code::step(&mut numbers, current_index, input);
        current_index = next_index;
        last_output = output_maybe.or(last_output);
    }
    last_output
}
