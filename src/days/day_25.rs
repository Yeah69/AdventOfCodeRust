use crate::day_tasks;
use super::ascii_code;
use std::collections::HashSet;
use std::io;

pub struct Day25;

impl day_tasks::DayTasks for Day25 {
    fn day_number (&self) -> String {
        "25".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let mut program = ascii_code::create_program(input);

        program.push_script_as_input(&"north\ntake candy cane\nsouth\nsouth\ntake fuel cell\nsouth\ntake manifold\nnorth\nnorth\n".to_string());
        program.push_script_as_input(&"west\ntake mutex\nsouth\nsouth\ntake coin\nwest\ntake dehydrated water\nsouth\ntake prime number\nnorth\neast\nnorth\n".to_string());
        program.push_script_as_input(&"east\ntake cake\nnorth\nwest\nsouth\nwest\n".to_string());
        brute_force_weight_check(&mut program, &mut ["coin", "cake", "prime number", "mutex", "dehydrated water", "manifold", "fuel cell", "candy cane"].iter().map(|s| s.to_string()).collect());
        program.run_and_prompt_user_input(&(|input| {
            match input.chars().nth(0) {
                Some('n') => return "north\n".to_string(),
                Some('s') => return "south\n".to_string(),
                Some('w') => return "west\n".to_string(),
                Some('e') => return "east\n".to_string(),
                Some('i') => return "inv\n".to_string(),
                Some('t') => return format!("take {}\n", &input[2..(input.len() - 2)]),
                Some('d') => return format!("drop {}\n", &input[2..(input.len() - 2)]),
                _ => return "\n".to_string()
            }
        }));
        "".to_string()
    }
    fn task_1 (&self, _: &String) -> String {
        "".to_string()
    }
}

fn brute_force_weight_check (program: &mut ascii_code::AsciiCodeProgram, items: &mut HashSet<String>) {
    let items_vec: Vec<String> = items.iter().map(|s| s.to_string()).collect();
    for item in items_vec {
        program.push_script_as_input(&format!("drop {}\nwest\n", item));
        //let mut input = String::new();
        //io::stdin().read_line(&mut input);
        items.remove(&item);
        brute_force_weight_check(program, items);
        program.push_script_as_input(&format!("take {}\n", item));
        items.insert(item);
    }
}
