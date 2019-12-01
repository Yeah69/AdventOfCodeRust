use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

pub struct Day03;

#[derive(Eq, PartialEq,Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct Instruction {
    direction: Direction,
    steps: i32
}

impl day_tasks::DayTasks for Day03 {
    fn day_number (&self) -> String {
        "03".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let (points_0, points_1) = get_paths(input);
        let hash_set_0: HashSet<(i32, i32)> = points_0.into_iter().collect();
        let hash_set_1: HashSet<(i32, i32)> = points_1.into_iter().collect();
        let result_maybe = hash_set_0
            .intersection(&hash_set_1)
            .map(|(x, y)| x.abs() + y.abs() )
            .min();
        result_maybe
            .map(|i| i.to_string())
            .unwrap_or("- No points that match -".to_string())
    }
    fn task_1 (&self, input: &String) -> String {
        let (points_0, points_1) = get_paths(input);
        let hash_set_0: HashSet<(i32, i32)> = points_0.clone().into_iter().collect();
        let hash_set_1: HashSet<(i32, i32)> = points_1.clone().into_iter().collect();
        let intersections: HashSet<&(i32, i32)> = hash_set_0
            .intersection(&hash_set_1)
            .collect();
        let distances_0 = get_distances_to_intersections(&points_0, &intersections);
        let distances_1 = get_distances_to_intersections(&points_1, &intersections);
        intersections
            .into_iter()
            .map(|intersection| distances_0[intersection] + distances_1[intersection])
            .min()
            .map(|i| i.to_string())
            .unwrap_or("- No points that match -".to_string())
    }
}

fn get_distances_to_intersections (path: &Vec<(i32, i32)>, intersections: &HashSet<&(i32, i32)>) -> HashMap<(i32, i32), i32> {
    let mut i = 0;
    let mut distances_to_intersections: HashMap<(i32, i32), i32> = HashMap::new();
    for element in path {
        i = i + 1;
        if intersections.contains(&element) && !distances_to_intersections.contains_key(&element) {
            distances_to_intersections.insert((element.0, element.1), i);
        }
    }
    distances_to_intersections
}

fn get_paths (input: &str) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
    let cables: Vec<Vec<Instruction>> = input
        .lines()
        .map(|line| {
            let result: Vec<Instruction> = line
                .split(',')
                .filter_map(|text_instruction| parse_to_instruction(text_instruction))
                .collect();
            result
        })
        .collect();
        if cables.len() == 2 {
            let points_0 = get_points(cables.get(0).unwrap());
            let points_1 = get_points(cables.get(1).unwrap());
            (points_0, points_1)
        }
        else { (vec![], vec![]) }
}

fn get_points(instructions: &Vec<Instruction>) -> Vec<(i32, i32)> {
    let map: HashMap<Direction, (i32, i32)> = 
        vec![(Direction::Up, (0, 1)), (Direction::Down, (0, -1)), (Direction::Left, (-1, 0)), (Direction::Right, (1, 0))]
        .into_iter()
        .collect();
    instructions
        .into_iter()
        .scan((0, 0), |(s_x, s_y), instruction| {
            let (d_x, d_y) = map[&(instruction.direction)];
            let (temp_x, temp_y) = (*s_x, *s_y);
            *s_x = *s_x + d_x * instruction.steps;
            *s_y = *s_y + d_y * instruction.steps;
            Some(((temp_x, temp_y), (d_x, d_y), instruction.steps))
        })
        .flat_map(|((s_x, s_y), (d_x, d_y), steps)| {
            iter::repeat((d_x, d_y))
                .take(steps as usize)
                .scan((s_x, s_y), |(curr_x, curr_y), (d_x, d_y)| {
                    *curr_x = *curr_x + d_x;
                    *curr_y = *curr_y + d_y;
                    Some((*curr_x, *curr_y))
                })
        })
        .collect()
}

fn parse_to_instruction(text: &str) -> Option<Instruction> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(U|D|L|R)(\\d*)").unwrap();
    }
    RE
        .captures(text)
        .map(|captures| {
            let direction_maybe = captures
                .get(1)
                .map(|text_direction| 
                     match text_direction.as_str() { 
                        "U"     =>  Direction::Up,
                        "D"     =>  Direction::Down,
                        "L"     =>  Direction::Left,
                        "R" | _ =>  Direction::Right});
            let steps_maybe = captures
                .get(2)
                .map(|text_steps| text_steps.as_str().parse::<i32>().unwrap());
            if direction_maybe.is_some() && steps_maybe.is_some() { 
                Some(Instruction { 
                    direction: direction_maybe.unwrap(),
                    steps: steps_maybe.unwrap() })}
            else { None }
        }).unwrap()
}
