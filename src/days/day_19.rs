use crate::day_tasks;
use super::int_code;
use std::iter;

pub struct Day19;

impl day_tasks::DayTasks for Day19 {
    fn day_number (&self) -> String {
        "19".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let mut i = 0;

        for y in 0..50 {
            for x in 0..50 {
                if check_position (x, y, input) {
                    i = i + 1;
                }
            }
        }

        i.to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let iter_left = iter::successors(Some((5, 8)), |(x, y)| {
            let new_y = *y + 1;
            let mut new_x = *x;
            loop {
                if check_position (new_x, new_y, input) {
                    break;
                }
                else {
                    new_x = new_x + 1;
                }
            };
            Some((new_x, new_y))
        });
        let iter_right = iter::successors(Some((5, 8)), |(x, y)| {
            let new_y = *y + 1;
            let mut new_x = *x + 1;
            loop {
                if !check_position (new_x, new_y, input) {
                    break;
                }
                else {
                    new_x = new_x + 1;
                }
            };
            Some((new_x - 1, new_y))
        });

        iter_left
            .skip(99)
            .zip(iter_right)
            .filter(|((left, _), (right, _))| *right - *left == 99)
            .map(|((left_x, _), (_, right_y))| { 
                left_x * 10_000 + right_y })
            .nth(0)
            .unwrap()
            .to_string()
    }
}

fn check_position (x: i128, y: i128, text_code: &String) -> bool {
    let mut program = int_code::create_program(text_code);
    program.push_input(x);
    program.push_input(y);
    if let Some(output) = program.run_until_next_output() { output == 1 }
    else { false }
}
