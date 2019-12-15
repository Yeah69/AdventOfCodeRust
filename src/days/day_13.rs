use crate::day_tasks;
use super::int_code::{create_program, parse_into_int_code, IntCodeProgramStatus::Ready};

pub struct Day13;

impl day_tasks::DayTasks for Day13 {
    fn day_number (&self) -> String {
        "13".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let mut program = create_program(parse_into_int_code(input));
        
        let mut i = 0;

        while program.get_status() == Ready {
            program.run_until_next_output().unwrap_or(-1);
            program.run_until_next_output().unwrap_or(-1);
            if program.run_until_next_output().unwrap_or(-1) == 2 {
                i = i + 1;
            }
        }
        
        i.to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let mut program = create_program(parse_into_int_code(input));
        program.day_13_part_two_initialize();

        let mut last_known_ball_x = 0;
        let mut last_known_paddle_x = 0;

        let mut last_known_score = 0;

        while program.get_status() == Ready {
            let x = program.run_until_next_output().unwrap_or(-1);
            let y = program.run_until_next_output().unwrap_or(-1);
            let tile_id = program.run_until_next_output().unwrap_or(-1);
            if tile_id == 3 {
                last_known_paddle_x = x;
            }
            if tile_id == 4 {
                last_known_ball_x = x;
                let next_input = if last_known_ball_x < last_known_paddle_x { -1 } else if last_known_ball_x > last_known_paddle_x { 1 } else { 0 };
                program.push_input(next_input);
            }
            if x == -1 && y == 0 {
                last_known_score = tile_id;
            }
        }
        
        last_known_score.to_string()
    }
}
