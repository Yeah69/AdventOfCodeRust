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
        amplifier_sequence(&numbers, &mut [0, 1, 2, 3, 4], &iteration_0)
            .map(|i| i.to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        let numbers = int_code::parse_into_int_code(input);
        amplifier_sequence(&numbers, &mut [5, 6, 7, 8, 9], &iteration_1)
            .map(|i| i.to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
}

fn amplifier_sequence(
    initial_state: &Vec<i128>,
    phase_setting_values: &mut [i128],
    phase_setting_iteration: &dyn Fn(&Vec<i128>, Vec<i128>) -> i128) -> Option<i128> {
    let all_possible_phase_settings = get_all_possible_phase_settings(phase_setting_values);
    all_possible_phase_settings
        .into_iter()
        .map(|phase_setting| phase_setting_iteration(&initial_state, phase_setting))
        .max()
}

fn iteration_1 (initial_state: &Vec<i128>, phase_setting: Vec<i128>) -> i128 {
    let programs: &mut [int_code::IntCodeProgram] = &mut [
        int_code::create_program(initial_state.to_vec()),
        int_code::create_program(initial_state.to_vec()),
        int_code::create_program(initial_state.to_vec()),
        int_code::create_program(initial_state.to_vec()),
        int_code::create_program(initial_state.to_vec())];
    
    let mut input = 0;
    for (program, phase) in programs.into_iter().zip(&phase_setting) {
        program.push_input(*phase);
    }
    while programs.first().unwrap().get_status() != int_code::IntCodeProgramStatus::Halt {
        input = programs
            .into_iter()
            .fold(input, |input, program| {
                program.push_input(input);
                program.run_until_next_output();
                program.get_last_output().unwrap_or(-1)
            });
    }
    programs.last().unwrap().get_last_output().unwrap_or(-1)
}

fn iteration_0 (initial_state: &Vec<i128>, phase_setting: Vec<i128>) -> i128 {
    let mut i = 0;
    for phase in phase_setting {
        let mut program = int_code::create_program(initial_state.to_vec());
        program.push_input(phase);
        program.push_input(i);
        i = program.run_until_next_output().unwrap_or(-1);
    }
    i
}

fn get_all_possible_phase_settings (data: &mut [i128]) -> Vec<Vec<i128>> {
    let mut permutations = Vec::new();
    permutohedron::heap_recursive(data, |permutation| {
        permutations.push(permutation.to_vec())
    });
    permutations
}
