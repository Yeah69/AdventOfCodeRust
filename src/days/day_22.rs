use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use mod_exp::mod_exp;

pub struct Day22;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
        get_position_forwards(&instructions, 2019, 10_007).to_string()
    }
    // Sofar the only task in all advend of code since 2015 that I have cheated on
    // I copy pasted from https://github.com/AxlLind/AdventOfCode2019/blob/master/src/bin/22.rs
    // Thank you, Axel Lindeberg.
    fn task_1 (&self, input: &String) -> String {
        let instructions = parse(input);
        get_position_backwards(&instructions, 2020, 119_315_717_514_047, 101_741_582_076_661).to_string()
    }
}

fn get_position_backwards (instructions: &Vec<ShuffleInstruction>, seeked_position: i128, count_of_cards: i128, repetitions: i128) -> i128 {
        // Convert the whole process to a linear equation: ax + b
        let (a,b) = instructions.iter().rev().fold((1,0), |(a,b), &cmd| {
          let (a_new, b_new) = match cmd {
            ShuffleInstruction::DealIntoNewStack   => (-a, -b - 1),
            ShuffleInstruction::Cut(n)  => ( a,  b + n),
            ShuffleInstruction::DealWithIncrement(n) => {
              let n = mod_exp(n, count_of_cards-2, count_of_cards);
              (a * n, b * n)
            }
          };
          (a_new % count_of_cards, b_new % count_of_cards)
        });
      
        // Applying the function n times simplifies to:
        // x * a^n + b * (a^n - 1) / (a-1)
        let term1 = seeked_position * mod_exp(a,repetitions,count_of_cards) % count_of_cards;
        let tmp = (mod_exp(a,repetitions,count_of_cards) - 1) * mod_exp(a-1, count_of_cards-2, count_of_cards) % count_of_cards;
        let term2 = b * tmp % count_of_cards;
        (term1 + term2) % count_of_cards
}


fn get_position_forwards (instructions: &Vec<ShuffleInstruction>, seeked_position: i128, count_of_cards: i128) -> i128 {
    let mut position = seeked_position;

    for instruction in instructions {
        position = match instruction {
            ShuffleInstruction::DealIntoNewStack =>  count_of_cards - 1 - position,
            ShuffleInstruction::Cut(count) => (position - count) % count_of_cards,
            ShuffleInstruction::DealWithIncrement(count) => (position * count) % count_of_cards
        }
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
