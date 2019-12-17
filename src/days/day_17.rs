use crate::day_tasks;
use super::int_code;
use std::collections::HashSet;

pub struct Day17;

impl day_tasks::DayTasks for Day17 {
    fn day_number (&self) -> String {
        "17".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let (scaffolds, _, _) = get_map_and_robot_status(input);

        scaffolds
            .iter()
            .filter(|(x, y)| scaffolds.contains(&(*x + 1, *y))
                && scaffolds.contains(&(*x - 1, *y)) 
                && scaffolds.contains(&(*x, *y + 1)) 
                && scaffolds.contains(&(*x, *y - 1)))
            .map(|(x,y)| (*x)*(*y))
            .sum::<i128>().to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let (scaffolds, robot_position, robot_facing_direction) = get_map_and_robot_status(input);

        let input_sequence = get_input_sequence(robot_position, robot_facing_direction, &scaffolds);

        let machine_specific_sequence = prepare_input_sequence_for_specific_machine(&input_sequence);

        let mut program = int_code::create_program(input);
        program.day_17_part_two_initialize();

        for i in machine_specific_sequence {
            program.push_input(i as i128);
        }

        program.run_until_stopped();

        program.get_last_output().map(|i| i.to_string()).unwrap_or("- Something went wrong -".to_string())
    }
}

fn prepare_input_sequence_for_specific_machine (original_sequence: &Vec<i32>) -> Vec<i32> {
    
    fn get_main_a_b_and_c (a_count: i32, b_count: i32, c_count: i32, original_sequence: &Vec<i32>) ->  Option<(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>)>{
        fn compare (x: &Vec<i32>, y: &Vec<i32>) -> bool {
            if x.len() != y.len() { false }
            else { x.iter().zip(y.iter()).all(|(i_x, i_y)| *i_x == *i_y) }
        }

        fn compare_sequence_start_with_segment (sequence: &Vec<i32>, segment: &Option<Vec<i32>>, ascii: i32) -> Option<(i32, Vec<i32>)> {
            if let Some(segment) = segment {
                let sequence_start = sequence.iter().take(segment.len()).map(|i| *i).collect::<Vec<i32>>();
                if compare(&sequence_start, segment) { Some((ascii, sequence.iter().skip(segment.len() + 1).map(|i| *i).collect::<Vec<i32>>())) } else { None }
            }
            else { None }
        }

        fn get_fitting_segment (sequence: &Vec<i32>, a: &Option<Vec<i32>>, b: &Option<Vec<i32>>, c: &Option<Vec<i32>>) -> Option<(i32, Vec<i32>)> {
            compare_sequence_start_with_segment(sequence, a, 'A' as u8 as i32)
                .or_else(|| compare_sequence_start_with_segment(sequence, b, 'B' as u8 as i32))
                .or_else(|| compare_sequence_start_with_segment(sequence, c, 'C' as u8 as i32))
        }

        let mut main: Vec<i32> = Vec::new();
        let mut a: Option<Vec<i32>> = None;
        let mut b: Option<Vec<i32>> = None;
        let mut c: Option<Vec<i32>> = None;
        let mut first_main_insertion = true;

        let mut to_be_processed: Vec<i32> = original_sequence.iter().map(|i| *i).collect();

        while !to_be_processed.is_empty() {
            if let Some((ascii, new_sequence)) = get_fitting_segment(&to_be_processed, &a, &b, &c) {
                to_be_processed = new_sequence;
                if !first_main_insertion { main.push(44) }
                main.push(ascii);
                first_main_insertion = false;
            }
            else {
                if a.is_some() && b.is_some() && c.is_some() { return None }
                let count = if a.is_none() { a_count } else if b.is_none() { b_count } else { c_count };
                let segment = to_be_processed.iter().take(count as usize).map(|i| *i).collect::<Vec<i32>>();
                if a.is_none() { a = Some(segment) } else if b.is_none() { b = Some(segment) } else { c = Some(segment) };
            }
        }

        if a.is_some() && b.is_some() && c.is_some() {
            Some((main, a.unwrap(), b.unwrap(), c.unwrap()))
        }
        else { None }
    };
    
    for a_count in (1..=20).into_iter().rev() {
        for b_count in (1..=20).into_iter().rev() {
            for c_count in (1..=20).into_iter().rev() {
                if let Some((main, a, b, c)) = get_main_a_b_and_c(a_count, b_count, c_count, original_sequence) {
                    let mut output: Vec<i32> = Vec::new();
                    output.extend(main);
                    output.push(10);
                    output.extend(a);
                    output.push(10);
                    output.extend(b);
                    output.push(10);
                    output.extend(c);
                    output.push(10);
                    output.push('n' as u8 as i32);
                    output.push(10);
                    return output;
                }
            }
        }
    }

    Vec::new()
}

fn get_input_sequence (start_position: (i128, i128), start_direction: i32, map: &HashSet<(i128,i128)>) -> Vec<i32> {
    fn next_direction (position: (i128, i128), previous_direction: i32, map: &HashSet<(i128, i128)>) -> Option<(i32, i32)> /* ascii, direction */ {
        let (x, y) = position;
        let (left_position, right_position) = 
            match previous_direction {
                1 => (((x - 1, y), 3), ((x + 1, y), 4)),
                2 => (((x + 1, y), 4), ((x - 1, y), 3)),
                3 => (((x, y + 1), 2), ((x, y - 1), 1)),
                _ => (((x, y - 1), 1), ((x, y + 1), 2)),
            };
        if map.contains(&left_position.0) {
            Some(('L' as i32, left_position.1))
        }
        else if map.contains(&right_position.0) {
            Some(('R' as i32, right_position.1))
        }
        else { None }
    }
    
    let mut output: Vec<i32> = Vec::new();
    let mut origin = start_position;
    let mut origin_direction = start_direction;
    let mut first_iteration = true;

    while let Some((ascii, direction)) = next_direction(origin, origin_direction, &map) {
        if !first_iteration { output.push(44) };
        output.push(ascii);
        output.push(44);
        let step = match direction {
            1 => (0, -1),
            2 => (0, 1),
            3 => (-1, 0),
            _ => (1, 0)
        };
        let mut step_count = if first_iteration { -1 } else { 0 };
        origin_direction = direction;
        origin = (origin.0 + step.0, origin.1 + step.1);
        while map.contains(&origin) {
            step_count = step_count + 1;
            origin = (origin.0 + step.0, origin.1 + step.1);
        } 
        origin = (origin.0 - step.0, origin.1 - step.1);
        for c in step_count.to_string().chars() {
            output.push(c as i32);
        }
        first_iteration = false;
    }

    output
}

fn get_map_and_robot_status (text_code: &String, ) -> (HashSet<(i128,i128)>, (i128, i128), i32) {
    let mut program = int_code::create_program(text_code);
        let mut scaffolds: HashSet<(i128,i128)> = HashSet::new();

        let mut ascii_text: Vec<i128> = Vec::new();

        while let Some(next) = program.run_until_next_output() {
            ascii_text.push(next);
        }

        let mut x = 0;
        let mut y = 0;
        let mut robot_x = 0;
        let mut robot_y = 0;
        let mut robot_facing_direction = 1;

        for tile in ascii_text {
            if tile == 46 {
                x = x + 1;
            }
            else if tile == 10 {
                x = 0;
                y = y + 1;
            }
            else {
                scaffolds.insert((x, y));
                x = x + 1;
                if tile != 35 {
                    robot_x = x;
                    robot_y = y;
                    robot_facing_direction = match (tile as u8) as char {
                        '^' => 1,
                        'v' => 2,
                        '<' => 3,
                        '>' => 4,
                        _ => 1
                    }
                }
            }
        }

        (scaffolds, (robot_x, robot_y), robot_facing_direction)
}
