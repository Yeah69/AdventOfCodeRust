use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashSet;

pub struct Day22;

#[derive(Clone, Copy, Debug)]
enum ShuffleInstruction {
    DealIntoNewStack,
    DealWithIncrement(i128),
    Cut(i128)
}

impl day_tasks::DayTasks for Day22 {
    fn day_number (&self) -> String {
        "22".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let instructions = parse(input);
        get_position(&instructions, 2019, 10_007, 1).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let instructions = parse(input);
        get_position(&instructions, 2020, 119_315_717_514_047, 101_741_582_076_661).to_string()
    }
}

fn get_position (instructions: &Vec<ShuffleInstruction>, seeked_position: i128, count_of_cards: i128, repetitions: i128) -> i128 {
    let mut i = 0;
    let mut position = seeked_position;
    let mut past_positions: HashSet<i128> = HashSet::new();
    past_positions.insert(position);
    println!("{}",position);
    while i < repetitions {
        position = do_instructions(instructions, seeked_position, count_of_cards);
        println!("{}",position);
        if past_positions.contains(&position) {
            //break;
        }
        else {
            past_positions.insert(position);
        }
        i = i + 1;
    };
    println!("{}",past_positions.len());
    position
}

fn do_instructions (instructions: &Vec<ShuffleInstruction>, seeked_position: i128, count_of_cards: i128) -> i128 {
    let mut position = seeked_position;
    for instruction in instructions {
        position = match instruction {
            ShuffleInstruction::DealIntoNewStack => count_of_cards - 1 - position,
            ShuffleInstruction::Cut(count) => {
                let count = *count;
                if count >= 0 { 
                    if count < position { position - count } 
                    else { count_of_cards + position - count } 
                }
                else { 
                    if position < count_of_cards + count - 1 { (position - count) % count_of_cards }
                    else { position - (count_of_cards + count) } 
                } 
            }
            ShuffleInstruction::DealWithIncrement(count) => (position * count) % count_of_cards 
        };
    }
    position
}

fn parse (input: &String) -> Vec<ShuffleInstruction> {
    lazy_static! {
        static ref REGEX_DEAL_INTO_NEW_STACK: Regex = Regex::new("deal into new stack").unwrap();
        static ref REGEX_DEAL_WITH_INCREMENT: Regex = Regex::new("deal with increment (\\d+)").unwrap();
        static ref REGEX_CUT: Regex = Regex::new("cut (.+)").unwrap();
    }
    input
        .lines()
        .map(|line| {
            let line = &line.to_string();
            let one = REGEX_DEAL_INTO_NEW_STACK.captures(line).map(|_| ShuffleInstruction::DealIntoNewStack);
            let two = REGEX_DEAL_WITH_INCREMENT.captures(line).map(|caps| ShuffleInstruction::DealWithIncrement(caps.get(1).map(|text_number| text_number.as_str().parse::<i128>().unwrap()).unwrap()));
            let three = REGEX_CUT.captures(line).map(|caps| ShuffleInstruction::Cut(caps.get(1).map(|text_number| text_number.as_str().parse::<i128>().unwrap()).unwrap()));
            [one, two, three].iter().filter_map(|opt| *opt).nth(0)
        })
        .filter_map(|opt| opt)
        .collect::<Vec<ShuffleInstruction>>()
}
