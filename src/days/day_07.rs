use crate::day_tasks;
use super::int_code;
use permutohedron;

pub struct Day07;

impl day_tasks::DayTasks for Day07 {
    fn day_number (&self) -> String {
        "07".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        amplifier_sequence(&numbers, &mut [0, 1, 2, 3, 4], &phase_setting_iteration_0)
            .map(|i| i.to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        amplifier_sequence(&numbers, &mut [5, 6, 7, 8, 9], &phase_setting_iteration_1)
            .map(|i| i.to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
}

fn amplifier_sequence(
    initial_state: &Vec<i32>,
    phase_setting_values: &mut [i32],
    phase_setting_iteration: &dyn Fn(&Vec<i32>, Vec<i32>) -> i32) -> Option<i32> {
    let all_possible_phase_settings = get_all_possible_phase_settings(phase_setting_values);
    all_possible_phase_settings
        .into_iter()
        .map(|phase_setting| phase_setting_iteration(&initial_state, phase_setting))
        .max()
}

fn phase_setting_iteration_1 (initial_state: &Vec<i32>, phase_setting: Vec<i32>) -> i32 {
    let programs: &mut [(Vec<i32>, i32, Option<i32>)] = &mut [(initial_state.to_vec(), 0, None), (initial_state.to_vec(), 0, None), (initial_state.to_vec(), 0, None), (initial_state.to_vec(), 0, None), (initial_state.to_vec(), 0, None)];
    
    let mut input = 0;
    input = programs
        .into_iter()
        .zip(&phase_setting)
        .fold(input, |second_input, ((program, program_counter, last_output), phase)| 
            iteration_1(program, (*phase, second_input), program_counter, last_output).unwrap_or(-1));
    while programs.first().unwrap().1 != -1 {
        input = programs
            .into_iter()
            .fold(input, |input, (program, program_counter, last_output)|
                iteration_1_1(program, input, program_counter, last_output).unwrap_or(-1));
    }
    programs.last().unwrap().2.unwrap_or(-1)
}

fn iteration_1(program: &mut Vec<i32>, input: (i32, i32), program_counter: &mut i32, last_output: &mut Option<i32>) -> Option<i32> {
    let mut current_input = input.0;
    while *program_counter != -1 && (*program_counter as usize) < (*program).len() && last_output.is_none() {
        let (next_index, output_maybe, is_input_consumed) = int_code::step(program, *program_counter, current_input);
        if is_input_consumed {
            current_input = input.1;
        }
        *program_counter = next_index;
        *last_output = output_maybe.or(*last_output);
    }
    *last_output
}

fn iteration_1_1(program: &mut Vec<i32>, input: i32, program_counter: &mut i32, last_output: &mut Option<i32>) -> Option<i32> {
    let mut i = 0;
    let mut inner_last_output: Option<i32> = None;
    while *program_counter != -1 && (*program_counter as usize) < (*program).len() && inner_last_output.is_none() {
        let (next_index, output_maybe, asdf) = int_code::step(program, *program_counter, input);
        if asdf {
            i = i + 1
        }
        *program_counter = next_index;
        inner_last_output = output_maybe;
        *last_output = output_maybe.or(*last_output);
    }
    *last_output
}


fn phase_setting_iteration_0 (initial_state: &Vec<i32>, phase_setting: Vec<i32>) -> i32 {
    let mut second_input = 0;
    for first_input in phase_setting {
        second_input = iteration_0(&initial_state, (first_input, second_input)).unwrap_or(-1);
    }
    second_input
}

fn iteration_0(initial_state: &Vec<i32>, input: (i32, i32)) -> Option<i32> {
    let mut numbers = initial_state.to_vec();
    let mut last_output: Option<i32> = None;
    let mut current_index: i32 = 0;
    let mut current_input = input.0;
    while current_index != -1 && (current_index as usize) < numbers.len() {
        let (next_index, output_maybe, is_input_consumed) = int_code::step(&mut numbers, current_index, current_input);
        if is_input_consumed {
            current_input = input.1;
        }
        current_index = next_index;
        last_output = output_maybe.or(last_output);
    }
    last_output
}

fn get_all_possible_phase_settings (data: &mut [i32]) -> Vec<Vec<i32>> {
    let mut permutations = Vec::new();
    permutohedron::heap_recursive(data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    permutations
}
