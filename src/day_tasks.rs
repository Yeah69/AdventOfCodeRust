use std::fs;
use std::time::Instant;

pub trait DayTasks {
    fn day_number (&self) -> String;
    fn task_0 (&self, input: &String) -> String;
    fn task_1 (&self, input: &String) -> String;

    fn run (&self) {
        let path = format! (".\\Input\\2019\\{}.txt", self.day_number());
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the input file");
        println!("Day {}:", self.day_number());
        let start = Instant::now();
        let result = self.task_0(&contents);
        let finish = Instant::now();
        println!("First Result = {} (took {:?})", result, finish - start);
        let start = Instant::now();
        let result = self.task_1(&contents);
        let finish = Instant::now();
        println!("Second Result = {} (took {:?})", result, finish - start);
    
    }
}
