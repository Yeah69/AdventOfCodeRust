use crate::day_tasks;
use std::collections::{HashSet, VecDeque};

pub struct Day24;

impl day_tasks::DayTasks for Day24 {
    fn day_number (&self) -> String {
        "24".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        get_biodiversity_of_recurring_layout(parse_0(input)).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        get_bugs_count(parse_1(input)).to_string()
    }
}

fn get_bugs_count (initial_layout: u32) -> i32 {
    let mut layouts: VecDeque<u32> = VecDeque::new();
    layouts.push_front(0);
    layouts.push_front(initial_layout);
    layouts.push_front(0);
    
    for _ in 0..200 {
        let mut new_layouts: VecDeque<u32> = VecDeque::new();

        for i in 0..layouts.len() {
            let higher = if i == 0 { 0u32 } else { *layouts.get(i - 1).expect("out of bounds") };
            let current = layouts.get(i).expect("out of bounds");
            let lower = if i == layouts.len() - 1 { 0u32 } else { *layouts.get(i + 1).expect("out of bounds") };

            let mut new: u32 = 0;
            for i in 0..25 {
                if i == 12 { continue; }
                let bit_index = 1u32<<i;
                let is_bug_alive = current & bit_index == bit_index;
                let x = i % 5;
                let y = i / 5;

                // current layout
                let mut adjacent_alive_count = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
                    .iter()
                    .filter(|(x, y)| *x >= 0 && *x < 5 && *y >= 0 && *y < 5)
                    .filter(|(x, y)| { let bit_index = 1u32<<(*x + *y * 5); (current & bit_index) == bit_index })
                    .count();
                
                // higher layout
                let adjacent_higher_indices = match (x, y) {
                    (0, 0) => [1u32<<7, 1u32<<11],
                    (4, 0) => [1u32<<7, 1u32<<13],
                    (4, 4) => [1u32<<13, 1u32<<17],
                    (0, 4) => [1u32<<11, 1u32<<17],
                    (0, _) => [1u32<<11, 1u32<<12],
                    (4, _) => [1u32<<13, 1u32<<12],
                    (_, 0) => [1u32<<7, 1u32<<12],
                    (_, 4) => [1u32<<17, 1u32<<12],
                    _ => [1u32<<12, 1u32<<12]
                };
                adjacent_alive_count += adjacent_higher_indices
                    .iter()
                    .filter(|higher_bit_index| higher & **higher_bit_index == **higher_bit_index)
                    .count();

                // lower layout
                let adjacent_lower_indices = match (x, y) {
                    (2, 1) => [1u32<<0, 1u32<<1, 1u32<<2, 1u32<<3, 1u32<<4],
                    (3, 2) => [1u32<<4, 1u32<<9, 1u32<<14, 1u32<<19, 1u32<<24],
                    (2, 3) => [1u32<<20, 1u32<<21, 1u32<<22, 1u32<<23, 1u32<<24],
                    (1, 2) => [1u32<<0, 1u32<<5, 1u32<<10, 1u32<<15, 1u32<<20],
                    _ => [1u32<<12, 1u32<<12, 1u32<<12, 1u32<<12, 1u32<<12]
                };
                adjacent_alive_count += adjacent_lower_indices
                    .iter()
                    .filter(|lower_bit_index| lower & **lower_bit_index == **lower_bit_index)
                    .count();

                if is_bug_alive && adjacent_alive_count == 1 || !is_bug_alive && (adjacent_alive_count == 1 ||adjacent_alive_count == 2) {
                    new += bit_index;
                }
            }
            new_layouts.push_back(new);
        }

        if new_layouts.front().map(|i| *i != 0).unwrap_or(false) { new_layouts.push_front(0) }
        if new_layouts.back().map(|i| *i != 0).unwrap_or(false) { new_layouts.push_back(0) }
        layouts = new_layouts;
    }
    
    let mut result = 0;
    for layout in layouts {
        for i in 0..25 {
            let bit_index = 1u32<<i;
            if layout & bit_index == bit_index {
                result += 1;
            }
        }
    }
    result
}

fn get_biodiversity_of_recurring_layout (initial_layout: u32) -> u32 {
    let mut current_layout = initial_layout;
    let mut previous_layouts: HashSet<u32> = HashSet::new();
    previous_layouts.insert(initial_layout);
    loop {
        let mut new_layout = 0u32;
        for i in 0..25 {
            let bit_index = 1u32<<i;
            let is_bug_alive = current_layout & bit_index == bit_index;
            let x = i % 5;
            let y = i / 5;
            let adjacent_alive_bug_count = [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)]
                .iter()
                .filter(|(x, y)| *x >= 0 && *x < 5 && *y >= 0 && *y < 5)
                .filter(|(x, y)| { let bit_index = 1u32<<(*x + *y * 5); (current_layout & bit_index) == bit_index })
                .count();
                
            if is_bug_alive && adjacent_alive_bug_count == 1 || !is_bug_alive && (adjacent_alive_bug_count == 1 ||adjacent_alive_bug_count == 2) {
                new_layout += bit_index;
            }
        }
        current_layout = new_layout;
        if previous_layouts.contains(&current_layout) {
            return current_layout;
        }
        else {
            previous_layouts.insert(current_layout);
        }
    }
}

fn parse_1 (input: &String) -> u32 {
    input
        .lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .map(|(i, c)| if i != 12 && c == '#' { 1u32<<i } else { 0u32 })
        .sum()
}

fn parse_0 (input: &String) -> u32 {
    input
        .lines()
        .flat_map(|line| line.chars())
        .enumerate()
        .map(|(i, c)| if c == '#' { 1u32<<i } else { 0u32 })
        .sum()
}
