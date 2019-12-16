use crate::day_tasks;
use std::collections::HashMap;

pub struct Day16;

impl day_tasks::DayTasks for Day16 {
    fn day_number (&self) -> String {
        "16".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let pattern = [0, 1, 0, -1];
        let digits = input.chars().map(|c| c.to_string().parse::<i32>().unwrap()).collect::<Vec<_>>();
        let mut cache: HashMap<(usize, i32), i32> = HashMap::new();
        (0..8).map(|i| get_digity(i, 100, &digits, &pattern, &mut cache, digits.len())).fold("".to_string(), |current, next| format!("{}{}", current, next))
    }
    fn task_1 (&self, input: &String) -> String {
        let len = input.chars().count() * 10_000;
        let offset = input.chars().take(7).fold("".to_string(), |current, next| format!("{}{}", current, next)).parse::<i32>().unwrap();
        let mut digits = input.chars().map(|c| c.to_string().parse::<i32>().unwrap()).cycle().skip(offset as usize % input.len()).take(len - offset as usize).collect::<Vec<_>>();
        
        for _ in 0..100 {
            digits = generate_numbers(digits);
        }

        digits.iter().take(8).fold("".to_string(), |current, next| format!("{}{}", current, next))
    }
}

fn get_digity (index: usize, iteration: i32, signal: &Vec<i32>, pattern: &[i32; 4], cache: &mut HashMap<(usize, i32), i32>, length: usize) -> i32 {
    if iteration == 0 {
        signal[index % signal.len()]
    }
    else if let Some(digit) = cache.get(&(index, iteration)) {
        *digit
    }
    else {
        let new_iteration = iteration - 1;
        let small_step = index + 1;
        let big_step = small_step * 4;
        let plus: i32 = ((small_step - 1)..length)
            .step_by(big_step)
            .flat_map(|start| (start..length).take(small_step))
            .map(|i| get_digity(i, new_iteration, signal, pattern, cache, length))
            .sum();
        let minus: i32 = ((small_step * 3 - 1)..length)
            .step_by(big_step)
            .flat_map(|start| (start..length).take(small_step))
            .map(|i| get_digity(i, new_iteration, signal, pattern, cache, length))
            .sum();
        let number = plus - minus;
        let digit = number.abs() % 10;
        cache.insert((index, iteration), digit);
        digit
    }
}

fn generate_numbers (mut input: Vec<i32>) -> Vec<i32> {
    input
        .reverse();
    let mut output = input.iter()
        .scan(0, |current, next| {
            *current = next + *current;
            Some(*current)
        })
        .map(|i| i.abs() % 10)
        .collect::<Vec<i32>>();
    output.reverse();
    output
}