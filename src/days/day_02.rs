use super::day_tasks;

pub struct Day02;

impl day_tasks::DayTasks for Day02 {
    fn day_number (self: &Self) -> String {
        "02".to_string()
    }
    fn task_0 (self: &Self, input: &String) -> String {
        let numbers = get_numbers(input);
        iteration(&numbers, 12, 2).to_string()
    }
    fn task_1 (self: &Self, input: &String) -> String {
        let numbers = get_numbers(input);
        let (noun, verb) = find_output(&numbers, 19690720);
        let result = 100 * noun + verb;
        result.to_string()
    }
}

fn get_numbers (input: &String) -> Vec<i32>{
    input.split(',').map(|text_number| text_number.parse::<i32>().unwrap()).collect()
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
    let mut numbers = initial_state.to_vec();
    numbers[1] = noun;
    numbers[2] = verb;
    let mut current_index = 0;
    while numbers[current_index] != 99 {
        let op_code = numbers[current_index];
        if op_code == 1 || op_code == 2 {
            let operator_0 = numbers[numbers[current_index + 1] as usize];
            let operator_1 = numbers[numbers[current_index + 2] as usize];
            let target_index = numbers[current_index + 3];
            let result = if op_code == 1 { operator_0 + operator_1 } else { operator_0 * operator_1};
            numbers[target_index as usize] = result;
        }
        current_index = (current_index + 4) % numbers.len();
    }

    numbers[0]
}