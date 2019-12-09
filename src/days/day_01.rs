use crate::day_tasks;

pub struct Day01;

impl day_tasks::DayTasks for Day01 {
    fn day_number (self: &Self) -> String {
        "01".to_string()
    }
    fn task_0 (self: &Self, input: &String) -> String {
        let result: i32 = input
            .lines()
            .map(|line| fuel_calculation(line.parse::<i32>().unwrap()))
            .sum();
        result.to_string()
    }
    fn task_1 (self: &Self, input: &String) -> String {
        let result : i32 = 
        input
            .lines()
            .map(|line| {
                let module_mass_fuel = fuel_calculation(line.parse::<i32>().unwrap());
                let sum : i32 = std::iter::successors(Some(module_mass_fuel), |current_mass| {
                    let next_mass = fuel_calculation(*current_mass);
                    if next_mass > 0 {
                        Some(next_mass)
                    }
                    else {
                        None
                    }
                }).sum();
                sum
            })
            .sum();
        result.to_string()
    }
}

fn fuel_calculation(mass : i32) -> i32 {
    mass / 3 - 2
}