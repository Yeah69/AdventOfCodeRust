use crate::day_tasks;
use super::ascii_code;
use std::fs;
use std::time::Instant;


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
    fn run (&self) {
        let path = format! (".\\Input\\2019\\{}.txt", self.day_number());
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the input file");
        println!("Day {}:", self.day_number());
        let start = Instant::now();
        let _ = self.task_0(&contents);
        let finish = Instant::now();
        println!("(took {:?})", finish - start);
        let start = Instant::now();
        let _ = self.task_1(&contents);
        let finish = Instant::now();
        println!("(took {:?})", finish - start);
    
    }
}
