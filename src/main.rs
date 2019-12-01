use std::fs;

fn main() {
    day_1();
}

fn day_1() {
    let contents = fs::read_to_string("E:\\GitReps\\AdventOfCodeRust\\Input\\2019\\Day01.txt")
        .expect("Something went wrong reading the file");
    let result_1 : i32 = contents
        .lines()
        .map(|line| day_1_fuel_calculation(line.parse::<i32>().unwrap()))
        .sum();
    println!("First Result = {}", result_1);

    let result_2 : i32 = 
    contents
        .lines()
        .map(|line| {
            let module_mass_fuel = day_1_fuel_calculation(line.parse::<i32>().unwrap());
            let sum : i32 = std::iter::successors(Some(module_mass_fuel), |current_mass| {
                let next_mass = day_1_fuel_calculation(*current_mass);
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
    println!("Second Result = {}", result_2);
}

fn day_1_fuel_calculation(mass : i32) -> i32 {
    mass / 3 - 2
}
