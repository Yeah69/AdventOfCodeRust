use crate::day_tasks;
use super::ascii_code;

pub struct Day21;

impl day_tasks::DayTasks for Day21 {
    fn day_number (&self) -> String {
        "21".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let mut program = ascii_code::create_program(input);
        program.run_script(&"NOT C J\nNOT A T\nOR T J\nAND D J\nWALK\n".to_string());
        "".to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let mut program = ascii_code::create_program(input);
        program.run_script(&"NOT C J\nNOT A T\nOR T J\nNOT B T\nOR T J\nOR E T\nOR H T\nAND D T\nAND T J\nRUN\n".to_string());
        "".to_string()
    }
}
