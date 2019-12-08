use std::fs;

fn main() {
    day_02();
    //_day_01();
}

fn day_02() {
    let contents = fs::read_to_string(".\\Input\\2019\\02.txt")
        .expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents.split(',').map(|text_number| text_number.parse::<i32>().unwrap()).collect();
    
    let result_0 = day_02_iteration(&numbers, 12, 2);
    println!("First Result = {}", result_0);

    let (noun, verb) = day_02_find_output(&numbers, 19690720);
    let result_1 = 100 * noun + verb;
    println!("Second Result = {}", result_1);

}

fn day_02_find_output (initial_state: &Vec<i32>, seeked_output: i32) -> (i32, i32) {
    for noun in 0..=99 {
        for verb in 0..=99 {
            let result = day_02_iteration(&(initial_state.to_vec()), noun, verb);
            if result == seeked_output {
                return (noun, verb);
            }
        }
    }
    (0, 0)
}

fn day_02_iteration(initial_state: &Vec<i32>, noun: i32, verb: i32) -> i32 {
    let mut numbers = initial_state.to_vec();
    numbers[1] = noun;
    numbers[2] = verb;
    let mut current_index = 0;
    while numbers[current_index] != 99 {
        let op_code = numbers[current_index];
        if op_code == 1 || op_code == 2 {
            let operator_0 = numbers[numbers[current_index + 1] as usize];
            let operator_1 = numbers[numbers[current_index + 2] as usize];
            let target_index = numbers[current_index + 3];
            let result = if op_code == 1 { operator_0 + operator_1 } else { operator_0 * operator_1};
            numbers[target_index as usize] = result;
        }
        current_index = (current_index + 4) % numbers.len();
    }

    numbers[0]
}

fn _day_01() {
    let contents = fs::read_to_string(".\\Input\\2019\\01.txt")
        .expect("Something went wrong reading the file");
    let result_0 : i32 = contents
        .lines()
        .map(|line| _day_1_fuel_calculation(line.parse::<i32>().unwrap()))
        .sum();
    println!("First Result = {}", result_0);

    let result_1 : i32 = 
    contents
        .lines()
        .map(|line| {
            let module_mass_fuel = _day_1_fuel_calculation(line.parse::<i32>().unwrap());
            let sum : i32 = std::iter::successors(Some(module_mass_fuel), |current_mass| {
                let next_mass = _day_1_fuel_calculation(*current_mass);
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
    println!("Second Result = {}", result_1);
}

fn _day_1_fuel_calculation(mass : i32) -> i32 {
    mass / 3 - 2
}
