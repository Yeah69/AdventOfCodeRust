use crate::day_tasks;
use super::int_code;
use std::collections::HashMap;

pub struct Day23;

impl day_tasks::DayTasks for Day23 {
    fn day_number (&self) -> String {
        "23".to_string()
    }
    fn task_0 (&self, input: &String) -> String {
        let mut network = create_network(input);
        network.do_task(true).to_string()
    }
    fn task_1 (&self, input: &String) -> String {
        let mut network = create_network(input);
        network.do_task(false).to_string()
    }
}

struct Network {
    programs: Vec<(i128,int_code::IntCodeProgram)>
}

impl Network {
    fn do_task (&mut self, is_task_0: bool) -> i128 {
        let mut packet_cache: HashMap<i128, (i128, Option<i128>, Option<i128>)> = HashMap::new();
        let mut idle_map: HashMap<i128, bool> = HashMap::new();
        let mut last_nat_packet_send_y = -1i128;
        let mut nat_packet = (-1i128, -1i128);
        loop {
            for (i, program) in &mut self.programs {
                let output_maybe = program.step();
                if program.get_status() == int_code::IntCodeProgramStatus::WaitingForInput {
                    idle_map.insert(*i, true);
                    program.push_input(-1);
                    program.step(); // output_maybe should be None because this step is an input instruction. Therefore no need to update.
                }
                if let Some(output) = output_maybe {
                    let mut found = false;
                    let mut item: Option<(i128, Option<i128>, Option<i128>)> = None;
                    for (o_i, (address, x_maybe, _)) in (&packet_cache).iter() {
                        if o_i == i {
                            if let Some(x) = x_maybe {
                                if *address == 255 && is_task_0 { return output; }
                                else if *address == 255 { nat_packet = (*x, output); }
                                else if *address >= 0 && *address < 50 {
                                    item = Some((*address, Some(*x), Some(output)));
                                }
                            }
                            else {
                                item = Some((*address, Some(output), None));
                            }
                            found = true;
                        }
                    } 
                    if !found { packet_cache.insert(*i, (output, None, None)); }
                    else if let Some(item) = item { packet_cache.insert(*i, item); }
                }
            }
            let mut addresses_to_remove: Vec<i128> = Vec::new();
            for (origin_address, (target_address, x_maybe, y_maybe)) in (&packet_cache).iter() {
                if let Some(x) = x_maybe {
                    if let Some(y) = y_maybe {
                        for (i, target_program) in &mut self.programs {
                            if i == target_address {
                                target_program.push_input(*x);
                                target_program.push_input(*y);
                                idle_map.insert(*i, false);
                            }
                        }
                        addresses_to_remove.push(*origin_address);
                    }
                }
            }
            for address in addresses_to_remove {
                packet_cache.remove(&address);
            }
            if idle_map.len() == 50 && idle_map.iter().all(|(_, b)| *b) {
                if nat_packet != (-1, -1) {
                    if last_nat_packet_send_y == nat_packet.1 { return last_nat_packet_send_y; }
                    let (_, first_program) = self.programs.get_mut(0).expect("First program not found");
                    first_program.push_input(nat_packet.0);
                    first_program.push_input(nat_packet.1);
                    idle_map.insert(0, false);
                    last_nat_packet_send_y = nat_packet.1;
                }
            }
        }
    }
}

fn create_network (text_code: &String) -> Network {
    let mut programs: Vec<(i128,int_code::IntCodeProgram)> = Vec::new();
    
    for i in 0..50 {
        let mut program = int_code::create_program(text_code);
        program.push_input(i as i128);
        programs.push((i as i128, program));
    }

    Network { programs: programs }
}
