use crate::day_tasks;
use itertools::Itertools;
use colored::Colorize;
use std::fs;
use std::time::Instant;

pub struct Day08;

impl day_tasks::DayTasks for Day08 {
    fn day_number (&self) -> String {
        "08".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        input
            .chars()
            .chunks(150) // 25 * 6 = 150
            .into_iter()
            .map(|chunk| {
                chunk.fold((0, 0, 0), |(zeros, ones, twos), character| 
                    match character { 
                        '0' => (zeros + 1, ones, twos), 
                        '1' => (zeros, ones + 1, twos), 
                        '2' => (zeros, ones, twos + 1), 
                        _ => (zeros, ones, twos)})
            })
            .min_by_key(|t| t.0)
            .map(|(_, ones, twos)| (ones * twos).to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        let lines = (0..150)
            .into_iter()
            .map(|i| {
                let mut inner_i = i;
                let mut seeked_char = '2';
                while inner_i < input.len() && seeked_char == '2' {
                    let current_char = input.chars().nth(inner_i).unwrap_or('a');
                    if current_char == '0' || current_char == '1' { seeked_char = current_char }
                    inner_i = inner_i + 150;
                }
                seeked_char
            })
            .chunks(25);
        for line in lines.into_iter() {
            for c in line {
                if c == '0' { print!("{}", "█".blue()) }
                else if c == '1' {  print!("{}", "█".white()) }
            }
            println!()
        }
        "".to_string()
    }

    fn run (&self) {
        let path = format! (".\\Input\\2019\\{}.txt", self.day_number());
        let contents = fs::read_to_string(path)
            .expect("Something went wrong reading the input file");
        println!("Day {}:", self.day_number());
        let start = Instant::now();
        let result = self.task_0(&contents);
        let finish = Instant::now();
        println!("First Result = {} (took {:?})", result, finish - start);
        println!("Second Result =");
        let start = Instant::now();
        self.task_1(&contents);
        let finish = Instant::now();
        println!("(took {:?})", finish - start);
    
    }
}
