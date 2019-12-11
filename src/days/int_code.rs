pub fn parse_into_int_code (input: &String) -> Vec<i32>{
    input.split(',').map(|text_number| text_number.parse::<i32>().unwrap()).collect()
}

pub fn step (state: &mut Vec<i32>, program_counter: i32, input: i32) -> (i32, Option<i32>)  {
    let program_counter_usize = program_counter as usize;
    let full_op_code = state[program_counter_usize];
    let op_code = full_op_code % 100;

    match op_code {
        1 | 2 | 5 | 6 | 7 | 8 => {
            let position_mode_0 = (full_op_code /    100) % 10 == 0;
            let position_mode_1 = (full_op_code /  1_000) % 10 == 0;
    
            let operator_0 = state[program_counter_usize + 1];
            let operator_0 = if position_mode_0 { state[operator_0 as usize] } else { operator_0 };
            let operator_1 = state[program_counter_usize + 2];
            let operator_1 = if position_mode_1 { state[operator_1 as usize] } else { operator_1 };

            match op_code {
                5 | 6 => {
                    if op_code == 5 && operator_0 != 0 || op_code == 6 && operator_0 == 0 { (operator_1, None) } 
                    else { ((program_counter + 3) as i32, None)}
                }
                _ => {
                    let target_index = state[program_counter_usize + 3];
                    let result = match op_code {
                        1 => operator_0 + operator_1,
                        2 => operator_0 * operator_1,
                        7 => if operator_0 < operator_1 { 1 } else { 0 },
                        _ => if operator_0 == operator_1 { 1 } else { 0 } // Should be op code 8
                    };
                    state[target_index as usize] = result;
            
                    ((program_counter + 4) as i32, None)}
            }
        }
        3 => {
            let target_index = state[program_counter_usize + 1];
            state[target_index as usize] = input;

            ((program_counter + 2) as i32, None)
        }
        4 => {
            let position_mode_0 = (full_op_code /    100) % 10 == 0;
            
            let operator_0 = state[program_counter_usize + 1];
            let operator_0 = if position_mode_0 { state[operator_0 as usize] } else { operator_0 };
            
            ((program_counter + 2) as i32, Some(operator_0))
        }
        _ => (-1, None)
    }
}