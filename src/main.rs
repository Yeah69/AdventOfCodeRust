mod days;
mod day_tasks;
use day_tasks::DayTasks;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let finish = Instant::now();
    let diff = finish - start;
    println!("My machine does \"nothing\" as fast as {:?}!", diff);
    let start = Instant::now();
    //days::day_01::Day01.run();
    //days::day_02::Day02.run();
    //days::day_03::Day03.run();
    //days::day_04::Day04.run();
    //days::day_05::Day05.run();
    //days::day_06::Day06.run();
    //days::day_07::Day07.run();
    //days::day_08::Day08.run();
    //days::day_09::Day09.run();
    //days::day_10::Day10.run();
    //days::day_11::Day11.run();
    //days::day_12::Day12.run();
    //days::day_13::Day13.run();
    //days::day_14::Day14.run();
    //days::day_15::Day15.run();
    //days::day_16::Day16.run();
    //days::day_17::Day17.run();
    //days::day_18::Day18.run();
    //days::day_19::Day19.run();
    //days::day_20::Day20.run();
    //days::day_21::Day21.run();
    days::day_22::Day22.run();
    //days::day_23::Day23.run();
    //days::day_24::Day24.run();
    //days::day_25::Day25.run();
    let finish = Instant::now();
    let diff = finish - start;
    println!("The whole run took in total {:?}!", diff);
}
