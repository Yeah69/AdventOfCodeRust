use crate::day_tasks;
use super::int_code;
use std::collections::{HashMap, VecDeque, HashSet};

pub struct Day15;

impl day_tasks::DayTasks for Day15 {
    fn day_number (&self) -> String {
        "15".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let mut program = int_code::create_program(input);
        let mut map: HashMap<(i32, i32), i128> = HashMap::new();
        map.insert((0, 0), 1);
        explore_map(&mut program, &mut map);
        get_length_of_shortest_path_to_oxygen_system(&map)
            .map(|(_, i)| i.to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        let mut program = int_code::create_program(input);
        let mut map: HashMap<(i32, i32), i128> = HashMap::new();
        map.insert((0, 0), 1);
        explore_map(&mut program, &mut map);
        get_length_of_shortest_path_to_oxygen_system(&map)
            .map(|(start, _)| fill_room_with_oxygen(start, &map).to_string())
            .unwrap_or("- Something went wrong -".to_string())
    }
}

fn fill_room_with_oxygen (start: (i32, i32), map: &HashMap<(i32, i32), i128>) -> i32 {
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut max_minutes = 0;
    queue.push_back((start, 0));
    while !queue.is_empty() {
        let (current_position, distance_sofar) = queue.pop_front().unwrap();
        visited_positions.insert(current_position);
        let (x, y) = current_position;
        let next_distance_sofar = distance_sofar + 1;
        for next_point in vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .filter(|next_point| map.contains_key(next_point) 
                && *map.get(next_point).unwrap() != 0 
                && !visited_positions.contains(next_point)) {
            queue.push_back((*next_point, next_distance_sofar));
            if next_distance_sofar > max_minutes {
                max_minutes = next_distance_sofar;
            }
        }
    };
    max_minutes
}

fn get_length_of_shortest_path_to_oxygen_system (map: &HashMap<(i32, i32), i128>) -> Option<((i32, i32), i32)> {
    let mut queue: VecDeque<((i32, i32), i32)> = VecDeque::new();
    let mut visited_positions: HashSet<(i32, i32)> = HashSet::new();
    queue.push_back(((0, 0), 0));
    while !queue.is_empty() {
        let (current_position, distance_sofar) = queue.pop_front().unwrap();
        visited_positions.insert(current_position);
        let (x, y) = current_position;
        let next_distance_sofar = distance_sofar + 1;
        for next_point in vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)]
            .iter()
            .filter(|next_point| map.contains_key(next_point) 
                && *map.get(next_point).unwrap() != 0 
                && !visited_positions.contains(next_point)) {
            if let Some(2) = map.get(next_point) {
                return Some((*next_point, next_distance_sofar));
            }
            queue.push_back((*next_point, next_distance_sofar));
        }
    };
    None
}

fn explore_map (program: &mut int_code::IntCodeProgram, map: &mut HashMap<(i32, i32), i128>) {
    let mut stack: VecDeque<((i32, i32), i128)> = VecDeque::new();
    stack.push_front(((0, 0), 1));
    while !stack.is_empty() {
        let (current_point, return_command) = stack.front().unwrap();
        let (x, y) = *current_point;
        let next_point_maybe = (1..=4).map(|direction| (match direction {
                1 => (x, y - 1),
                2 => (x, y + 1),
                3 => (x - 1, y),
                4 => (x + 1, y),
                _ => (x, y)
            }, direction))
            .filter(|(next_point, _)| !map.contains_key(&next_point))
            .nth(0);
        if let Some((next_point, direction)) = next_point_maybe {
            program.push_input(direction);
            let status = program.run_until_next_output().unwrap();
            map.insert(next_point, status);
            if status != 0 {
                let next_return_command = match direction {
                    1 => 2,
                    2 => 1,
                    3 => 4,
                    4 => 3,
                    _ => direction
                };
                stack.push_front((next_point, next_return_command));
            }
        }
        else {
            program.push_input(*return_command);
            program.run_until_next_output().unwrap();
            stack.pop_front();
        }
    }
    
}
