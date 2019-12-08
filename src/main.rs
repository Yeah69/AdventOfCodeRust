mod days;

fn main() {
    fn run (runnable: impl days::day_tasks::DayTasks){
        runnable.run();
    }
    run(days::day_01::Day01);
    run(days::day_02::Day02);
}