use crate::day_tasks;
use super::int_code;
use std::collections::HashSet;
use colored::Colorize;
use std::fs;
use std::time::Instant;

pub struct Day11;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

fn change_direction (current_direction: &Direction, value: i128) -> Direction {
    match (current_direction, value) {
        (Direction::Up, 0) | (Direction::Down, 1) => Direction::Left,
        (Direction::Up, 1) | (Direction::Down, 0) => Direction::Right,
        (Direction::Left, 0) | (Direction::Right, 1) => Direction::Down,
        (Direction::Left, 1) | (Direction::Right, 0) => Direction::Up,
        _ => Direction::Up
    }
}

fn step (position: (i32, i32), direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (position.0, position.1 + 1),
        Direction::Down => (position.0, position.1 - 1),
        Direction::Left => (position.0 - 1, position.1),
        Direction::Right => (position.0 + 1, position.1)
    }
}

impl day_tasks::DayTasks for Day11 {
    fn day_number (&self) -> String {
        "11".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let (painted_positions, _) = run_painting_robot(input, 0);

        painted_positions.len().to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let (_, white_positions) = run_painting_robot(input, 1);

        let (min_x, max_x, min_y, max_y) = white_positions
            .iter()
            .fold((0, 0, 0, 0), |(current_min_x, current_max_x, current_min_y, current_max_y), current_white_position| {
                let next_min_x = if current_min_x > current_white_position.0  { current_white_position.0 } else { current_min_x };
                let next_max_x = if current_max_x < current_white_position.0  { current_white_position.0 } else { current_max_x };
                let next_min_y = if current_min_y > current_white_position.1  { current_white_position.1 } else { current_min_y };
                let next_max_y = if current_max_y < current_white_position.1  { current_white_position.1 } else { current_max_y };
                (next_min_x, next_max_x, next_min_y, next_max_y)
            });

        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if white_positions.contains(&(x, y)) { print!("{}", "█".purple()) }
                else {  print!("{}", "█".cyan()) }
            }
            println!("") 
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

fn run_painting_robot (input: &String, first_input: i128) -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>) {
    let mut program = int_code::create_program(input);
    let mut painted_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut currently_white_positions: HashSet<(i32, i32)> = HashSet::new();

    let mut current_position = (0, 0);
    let mut current_direction = Direction::Up;

    program.push_input(first_input);
    program.run_until_next_output();
    while program.get_status() == int_code::IntCodeProgramStatus::Ready {
        let color = program.get_last_output();
        if color.unwrap_or(-1) == 1 && !currently_white_positions.contains(&current_position) {
            currently_white_positions.insert(current_position);
        }
        else if color.unwrap_or(-1) == 0 && currently_white_positions.contains(&current_position) {
            currently_white_positions.remove(&current_position);
        }
        if !painted_positions.contains(&current_position) { painted_positions.insert(current_position); }
        program.run_until_next_output();
        let direction_change = program.get_last_output();
        current_direction = change_direction(&current_direction, direction_change.unwrap_or(-1));
        current_position = step (current_position, &current_direction);
        let program_input = if currently_white_positions.contains(&current_position) { 1 } else { 0 };
        program.push_input(program_input);
        program.run_until_next_output();
    }
    
    (painted_positions, currently_white_positions)
}
