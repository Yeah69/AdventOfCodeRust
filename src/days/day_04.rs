use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use std::ops::RangeInclusive;

pub struct Day04;

impl day_tasks::DayTasks for Day04 {
    fn day_number (&self) -> String {
        "04".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        fn adjacency_criterion (s_0: char, s_1: char, s_2: char, s_3: char, s_4: char, s_5: char) -> bool {
            s_0 == s_1 || s_1 == s_2 || s_2 == s_3 || s_3 == s_4 || s_4 == s_5
        }
        task_impl(input, &adjacency_criterion)
    }
    fn task_1 (&self, input: &String) -> String {
        fn adjacency_criterion (s_0: char, s_1: char, s_2: char, s_3: char, s_4: char, s_5: char) -> bool {
            s_0 == s_1 && s_1 != s_2 
            || s_1 == s_2 && s_0 != s_1 && s_2 != s_3 
            || s_2 == s_3 && s_1 != s_2 && s_3 != s_4 
            || s_3 == s_4 && s_2 != s_3 && s_4 != s_5
            || s_4 == s_5 && s_3 != s_4
        }
        task_impl(input, &adjacency_criterion)
    }
}

fn task_impl (input: &String, adjacency_criterion: &dyn Fn(char, char, char, char, char, char) -> bool) -> String {
    let range_maybe = parse_to_range(input);
        range_maybe
            .map(|range| {
                count_of_possible_passwords(range, &adjacency_criterion)
            })
            .map(|i| i.to_string())
            .unwrap_or("- input couldn't be parsed -".to_string())
}

fn count_of_possible_passwords (range: RangeInclusive<i32>, adjacency_criterion: &dyn Fn(char, char, char, char, char, char) -> bool) -> usize {
    range
        .into_iter()
        .map(|i| i.to_string())
        .filter(|s| {
            let s_0 = s.chars().nth(0).unwrap();
            let s_1 = s.chars().nth(1).unwrap();
            let s_2 = s.chars().nth(2).unwrap();
            let s_3 = s.chars().nth(3).unwrap();
            let s_4 = s.chars().nth(4).unwrap();
            let s_5 = s.chars().nth(5).unwrap();
            s_0 <= s_1 && s_1 <= s_2 && s_2 <= s_3 && s_3 <= s_4 && s_4 <= s_5
            && adjacency_criterion(s_0, s_1, s_2, s_3, s_4, s_5)
        })
        .count()
}

fn parse_to_range(text: &str) -> Option<RangeInclusive<i32>> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(\\d*)-(\\d*)").unwrap();
    }
    RE
        .captures(text)
        .map(|captures| {
            let lower_bound_maybe = captures
                .get(1)
                .map(|text_lower_bound| text_lower_bound.as_str().parse::<i32>().unwrap());
            let higher_bound_maybe = captures
                .get(2)
                .map(|text_higher_bound| text_higher_bound.as_str().parse::<i32>().unwrap());
            if lower_bound_maybe.is_some() && higher_bound_maybe.is_some() { 
                Some(lower_bound_maybe.unwrap()..=higher_bound_maybe.unwrap()) }
            else { None }
        }).unwrap()
}
