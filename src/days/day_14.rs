use crate::day_tasks;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

pub struct Day14;

impl day_tasks::DayTasks for Day14 {
    fn day_number (&self) -> String {
        "14".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let recipes = input
            .lines()
            .filter_map(parse_to_recipe)
            .collect::<HashMap<String, (Vec<(String, i64)>, i64)>>();

        let mut available_elements: HashMap<String, i64> = HashMap::new();

        let required_ore_for_one_fuel = get_required_ore("FUEL".to_string(), 1, 0, &mut available_elements, &recipes);

        required_ore_for_one_fuel.to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let recipes = input
            .lines()
            .filter_map(parse_to_recipe)
            .collect::<HashMap<String, (Vec<(String, i64)>, i64)>>();

        let mut available_elements: HashMap<String, i64> = HashMap::new();

        let mut ore_amount = 1_000_000_000_000i64;
        let mut i = 0;

        while ore_amount > 0 {
            let minus = get_required_ore("FUEL".to_string(), 1, 0, &mut available_elements, &recipes);
            ore_amount = ore_amount - minus;
            i = i + 1;
        }


        (i - 1).to_string()
    }
}

fn get_required_ore (
    element: String, 
    amount: i64, 
    current_ore_amount: i64,
    available_elements: &mut HashMap<String, i64>,
    recipes: &HashMap<String, (Vec<(String, i64)>, i64)>) -> i64 {
    if element == "ORE" { 
        current_ore_amount + amount }
    else {
        let mut required_amount = amount;
        if available_elements.contains_key(&element) {
            let available_amount = available_elements[&element];
            if available_amount >= required_amount {
                available_elements.remove(&element);
                if available_amount > required_amount {
                    available_elements.insert(element, available_amount - required_amount);
                }
                return current_ore_amount;
            }
            else {
                available_elements.remove(&element);
                required_amount = required_amount - available_amount;
            }
        }

        let (ingredients, recipe_amount) = &recipes[&element];
        let factor = if required_amount % recipe_amount == 0 { required_amount / recipe_amount } else { required_amount / recipe_amount + 1 };
        let remainding_amount = if required_amount % recipe_amount == 0 { 0 } else { recipe_amount - required_amount % recipe_amount };
        if remainding_amount > 0 {
            available_elements.insert(element, remainding_amount);
        }
        ingredients
            .iter()
            .fold(current_ore_amount, |current_ore, (next_element_name, next_element_amount)|
                get_required_ore(
                    next_element_name.to_string(), 
                    next_element_amount * factor, 
                    current_ore_amount, 
                    available_elements,
                    &recipes) + current_ore
            )
    }
}

fn parse_to_recipe(line: &str) -> Option<(String, (Vec<(String, i64)>, i64))> {
    lazy_static! {
        static ref RE_0: Regex = Regex::new("(.*) => (.*)").unwrap();
        static ref RE_1: Regex = Regex::new("(\\d+) ([A-Z]+)").unwrap();
    }
    RE_0
        .captures(line)
        .map(|captures| {
            let ingredients = captures
                .get(1)
                .map(|text_ingredients| { 
                    text_ingredients
                        .as_str()
                        .split(", ")
                        .map(|text_ingredient| RE_1
                            .captures(text_ingredient)
                            .map(|captures| {
                                let amount = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                                let name = captures.get(2).unwrap().as_str();
                                (name.to_string(), amount)
                            }))
                        .filter_map(|x| x)
                        .collect::<Vec<(String, i64)>>()
                })
                .unwrap();
            let (name, amount) = captures
                .get(2)
                .map(|text_result| RE_1
                    .captures(text_result.as_str())
                    .map(|captures| {
                        let amount = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
                        let name = captures.get(2).unwrap().as_str();
                        (name.to_string(), amount)
                    }))
                .unwrap()
                .unwrap();
            (name, (ingredients, amount))
        })
}
