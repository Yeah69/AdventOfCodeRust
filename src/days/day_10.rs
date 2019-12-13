use crate::day_tasks;
use std::collections::HashSet;
use num_integer::gcd;
use std::f64::consts::PI;

pub struct Day10;

impl day_tasks::DayTasks for Day10 {
    fn day_number (&self) -> String {
        "10".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let asteroid_points = get_asteroid_points(input);
        let mut max = 0;
        let mut max_position = (-1, -1);

        for position in &asteroid_points {
            let count = count_sightable_asteroids(*position, &asteroid_points);
            if max < count { 
                max = count;
                max_position = *position; }
        }
        format!("{} coordinates = {:?}", max, max_position)
    }
    fn task_1 (&self, input: &String) -> String {
        let asteroid_points = get_asteroid_points(input);
        let set = get_visible_aseteroids((8, 16), &asteroid_points);

        let mut sorted = set
            .into_iter()
            .map(|(x, y)| {
                let angle = (-(y as f64) / ((x * x + y * y) as f64).sqrt()).acos();
                let angle = if x < 0 { 2.0 * PI - angle } else { angle };
                (angle, x, y)
            })
            .collect::<Vec<(f64, i32, i32)>>();
        sorted.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        sorted
            .into_iter()
            .nth(199)
            .map(|(_, x, y)| {
                let (x, y) = (x + 8, y + 16);
                (x * 100 + y).to_string()})
            .unwrap_or("- Something went wrong -".to_string())
    }
}

fn get_asteroid_points (input: &String) -> Vec<(i32, i32)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line
            .chars()
            .enumerate()
            .filter_map(move |(x, c)| if c == '#' { Some((x as i32, y as i32)) } else { None }))
        .collect()
}

fn count_sightable_asteroids (position: (i32, i32), all_asteroid_positions: &Vec<(i32, i32)>) -> i32 {
    get_visible_aseteroids(position, &all_asteroid_positions).into_iter().count() as i32
}

fn get_visible_aseteroids (position: (i32, i32), all_asteroid_positions: &Vec<(i32, i32)>) -> HashSet<(i32, i32)> {
    let distances = all_asteroid_positions
        .into_iter()
        .filter(|p| **p != position)
        .map(|(x, y)| (*x - position.0, *y - position.1))
        .map(|(x, y)| {
            if x != 0 && y != 0 {
                let gcd = gcd(x, y);
                (x / gcd, y / gcd)
            }
            else if x == 0 && y >= 1 {
                (0, 1)
            }
            else if x == 0 && y <= 1 {
                (0, -1)
            }
            else if y == 0 && x >= 1 {
                (1, 0)
            }
            else  {
                (-1, 0)
            }
        })
        .collect::<HashSet<(i32, i32)>>();
    distances
}
