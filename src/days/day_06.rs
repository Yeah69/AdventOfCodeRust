use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use itertools::Itertools;

pub struct Day06;

impl day_tasks::DayTasks for Day06 {
    fn day_number (self: &Self) -> String {
        "06".to_string()
    }
    fn task_0 (self: &Self, input: &String) -> String {
        count_connections(&generate_adjacency_map(input), "COM".to_string()).1.to_string()
    }
    fn task_1 (self: &Self, input: &String) -> String {
        match determine_orbital_transfers_to_santa(&generate_adjacency_map(input), "COM".to_string()) {
            (Some(you), Some(santa)) => (you + santa).to_string(),
            _ => "- Something went wrong -".to_string()
        }
    }
}

fn determine_orbital_transfers_to_santa(adjacency_map: &HashMap<String, Vec<String>>, current_node: String) -> (Option<i32>, Option<i32>) {
    if current_node == "YOU" { (Some(0), None) }
    else if current_node == "SAN" { (None, Some(0)) }
    else if adjacency_map.contains_key(&current_node) {
        let vec = &adjacency_map[&current_node];
        let nodes = vec
            .into_iter()
            .map(|child| determine_orbital_transfers_to_santa(&adjacency_map, child.to_string()))
            .fold((None, None), |(current_node_you, current_node_santa), (child_node_you, child_node_santa)| 
                (current_node_you.or(child_node_you), current_node_santa.or(child_node_santa)));
        match nodes {
            (Some(_), Some(_)) => nodes,
            (Some(number), None) => (Some(number + 1), None),
            (None, Some(number)) => (None, Some(number + 1)),
            (None, None) => (None, None)
        }
    }
    else { (None, None)}
}

fn count_connections(adjacency_map: &HashMap<String, Vec<String>>, current_node: String) -> (i32, i32) {
    if adjacency_map.contains_key(&current_node) {
        let vec = &adjacency_map[&current_node];
        let (child_count, score) = vec
            .into_iter()
            .map(|child| count_connections(&adjacency_map, child.to_string()))
            .fold((0, 0), |(current_node_count, current_score), (child_node_count, child_score)| (current_node_count + child_node_count, current_score + child_score));
        (child_count + 1, child_count + score)
    }
    else { (1, 0)}
}

fn generate_adjacency_map (input: &String) -> HashMap<String, Vec<String>> {
    input
        .lines()
        .map(|line| parse_to_tuple(line))
        .filter_map(|option| option)
        .sorted()
        .group_by(|(key, _)| key.to_string())
        .into_iter()
        .map(|(key, values)| (key.to_string(), values.map(|(_, value)| value).collect::<Vec<String>>()))
        .collect()
}

fn parse_to_tuple(text: &str) -> Option<(String, String)> {
    lazy_static! {
        static ref RE: Regex = Regex::new("(.*)\\)(.*)").unwrap();
    }
    RE
        .captures(text)
        .map(|captures| {
            let left_maybe = captures
                .get(1)
                .map(|left|  left.as_str().to_string());
            let right_maybe = captures
                .get(2)
                .map(|right| right.as_str().to_string());
            if left_maybe.is_some() && right_maybe.is_some() { 
                Some((left_maybe.unwrap(), right_maybe.unwrap()))}
            else { None }
        }).unwrap()
}
