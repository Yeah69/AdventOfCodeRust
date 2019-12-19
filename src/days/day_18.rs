use crate::day_tasks;
use std::collections::HashMap;

pub struct Day18;

struct Node {
    up: Option<(i32, i32, i32)>,
    down: Option<(i32, i32, i32)>,
    left: Option<(i32, i32, i32)>,
    right: Option<(i32, i32, i32)>,
    key: Option<char>,
    door: Option<char>
}

struct Map {
    nodes: HashMap<(i32, i32), Node>
}

impl Map {
    fn parse (&mut self, input: &String) {
        let mut map = input
            .lines()
            .enumerate()
            .flat_map(|(y, line)| 
                line
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| *c != '#')
                    .map(move |(x, c)| (x, y, c))
            )
            .map(|(x, y, c)| {
                let node = Node {
                    up: None,
                    down: None,
                    left: None,
                    right: None,
                    key: if c >= 'a' && c <= 'z' { Some(c) } else { None },
                    door: if c >= 'A' && c <= 'Z' { Some(c.to_lowercase().nth(0).unwrap()) } else { None }
                };
                ((x as i32, y as i32), node)
            })
            .collect::<HashMap<(i32, i32), Node>>();
        
    }
}

impl day_tasks::DayTasks for Day18 {
    fn day_number (&self) -> String {
        "18".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        input.to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        input.to_string()
    }
}
