mod days;
mod day_tasks;
use day_tasks::DayTasks;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let finish = Instant::now();
    let diff = finish - start;
    println!("My machine does \"nothing\" as fast as {:?}!", diff);
    //days::day_01::Day01.run();
    days::day_02::Day02.run();
    //days::day_03::Day03.run();
    //days::day_04::Day04.run();
    days::day_05::Day05.run();
}
