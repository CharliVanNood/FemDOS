use crate::{println, string::BigString, window, warnln};

pub fn exec(input: [u8; 256]) {
    let mut input_string = BigString::from_b256(input);
    for _ in 0..32 {
        input_string.replace("\n", " lnnew ");
        input_string.replace("|", " lnnew ");
        input_string.replace("]", " lnnew ");
    }
    let tokenized_code = tokenize(input_string);
    run_tokens(tokenized_code);
}

fn shift_list(list: [(u8, i32); 255], index: usize, length: usize) -> [(u8, i32); 255] {
    let mut return_list = [(0, 0); 255];

    for i in 0..255 - length {
        if i < index {
            return_list[i] = list[i];
        } else {
            return_list[i] = list[i + length];
        }
    }

    return_list
}
fn run_tokens(mut tokens: [[(u8, i32); 255]; 32]) {
    let mut variables: [u16; 256] = [0; 256];
    let mut indentation: [i8; 16] = [-1; 16];
    let mut running = true;
    //let mut original_tokens = tokens;

    let mut line_index = 0;
    while line_index < tokens.len() {
        let line: [(u8, i32); 255] = tokens[line_index];

        let mut indentation_depth: u8 = 0;
        for indentation_layer in indentation {
            if indentation_layer > -1 {
                indentation_depth += 1;
            }
        }

        let operation_result = run_line(line, &mut indentation, line_index, &mut variables, indentation_depth, running);

        line_index = operation_result.1;
        running = operation_result.3;
        if operation_result.2 {
            tokens[line_index] = operation_result.0;
            line_index = indentation[indentation_depth as usize] as usize;
        }

        line_index += 1;
    }
}

fn run_line(line: [(u8, i32); 255], mut indentation: &mut [i8; 16], line_index: usize, mut variables: &mut [u16; 256], indentation_depth: u8, running: bool) -> ([(u8, i32); 255], usize, bool, bool) {
    if running {
        let tokens_after_fact = run_tokens_fact(line, *variables, *indentation, indentation_depth);
        let tokens_after_math = run_tokens_math(tokens_after_fact, *variables, *indentation, indentation_depth);
        let tokens_after_first = run_tokens_first(tokens_after_math, *variables, *indentation, indentation_depth);
        let tokens_after_bool = run_tokens_boolean(tokens_after_first, *variables, *indentation, indentation_depth);
        let operation_result = run_tokens_last(tokens_after_bool, &mut variables, &mut indentation, indentation_depth, line_index, running);
        return operation_result;
    } else {
        let operation_result = run_tokens_last(line, variables, indentation, indentation_depth, line_index, running);
        return operation_result;
    }
}

fn run_tokens_fact(mut tokens: [(u8, i32); 255], _variables: [u16; 256], _indentation: [i8; 16], _indentation_depth: u8) -> [(u8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            13 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result: f32 = tokens[token_index - 1].1 as f32 / tokens[token_index + 1].1 as f32 * 100.0;
                    tokens[token_index - 1] = (2, operation_result as i32);
                    tokens = shift_list(tokens, token_index, 2);
                    token_index -= 1;
                }
            },
            14 => {
                if tokens[token_index - 1].0 == 1 && tokens[token_index + 1].0 == 1 {
                    let operation_result = (1, tokens[token_index - 1].1 * tokens[token_index + 1].1);
                    tokens[token_index - 1] = operation_result;
                    tokens = shift_list(tokens, token_index, 2);
                    token_index -= 1;
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_math(mut tokens: [(u8, i32); 255], _variables: [u16; 256], _indentation: [i8; 16], _indentation_depth: u8) -> [(u8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            11 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let operation_result = (1, tokens[token_index - 1].1 + tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let operation_result = (2, tokens[token_index - 1].1 + tokens[token_index + 1].1 * 100);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 * 100 + tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 + tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            12 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let operation_result = (1, tokens[token_index - 1].1 - tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let operation_result = (2, tokens[token_index - 1].1 - tokens[token_index + 1].1 * 100);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 * 100 - tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let operation_result = (2, tokens[token_index - 1].1 - tokens[token_index + 1].1);
                        tokens[token_index - 1] = operation_result;
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_boolean(mut tokens: [(u8, i32); 255], _variables: [u16; 256], _indentation: [i8; 16], _indentation_depth: u8) -> [(u8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            17 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (3, 3) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 == tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            18 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 > tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 > tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 > tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 > tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            19 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 < tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 < tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 < tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 < tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            20 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 >= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 >= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 >= tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 >= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            21 => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 <= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 * 100 <= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 <= tokens[token_index + 1].1 * 100 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens[token_index - 1].1 <= tokens[token_index + 1].1 {
                            operation_result = 1;
                        }
                        tokens[token_index - 1] = (3, operation_result);
                        tokens = shift_list(tokens, token_index, 2);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_first(mut tokens: [(u8, i32); 255], _variables: [u16; 256], _indentation: [i8; 16], _indentation_depth: u8) -> [(u8, i32); 255] {
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match token.0 {
            22 => {
                if tokens[token_index + 1].0 == 3 {
                    if tokens[token_index + 1].1 == 0 {
                        tokens[token_index] = (3, 1);
                    } else {
                        tokens[token_index] = (3, 0);
                    }
                    tokens = shift_list(tokens, token_index + 1, 1);
                } else {
                    warnln!("This is an unsupported type conversion");
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

fn run_tokens_last(
    mut tokens: [(u8, i32); 255], variables: &mut [u16; 256], indentation: &mut [i8; 16], 
    indentation_depth: u8, line_index: usize, mut running: bool) -> ([(u8, i32); 255], usize, bool, bool) {
    let mut return_to_last_indent = false;
    
    let mut token_index = 0;
    for _ in 0..255 {
        let token = tokens[token_index];

        match (token.0, running) {
            (10, true) => {
                match tokens[token_index + 1].0 {
                    1 => {
                        println!("{}", tokens[token_index + 1].1);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    2 => {
                        println!("{}", tokens[token_index + 1].1 as f32 / 100.0);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    3 => {
                        if tokens[token_index + 1].1 == 0 {
                            println!("false");
                        } else {
                            println!("true");
                        }
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    7 => {
                        let variable_read = variables[tokens[token_index + 1].1 as usize];
                        println!("{}", variable_read);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (23, true) => {
                match tokens[token_index + 1].0 {
                    1 => {
                        warnln!("{}", tokens[token_index + 1].1);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    2 => {
                        warnln!("{}", tokens[token_index + 1].1 as f32 / 100.0);
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    3 => {
                        if tokens[token_index + 1].1 == 0 {
                            warnln!("false");
                        } else {
                            warnln!("true");
                        }
                        tokens = shift_list(tokens, token_index, 2);
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (24, true) => {
                match (tokens[token_index - 1].0, tokens[token_index + 1].0) {
                    (7, 1) => {
                        variables[tokens[token_index - 1].1 as usize] = tokens[token_index + 1].1 as u16;
                        tokens = shift_list(tokens, token_index - 1, 3);
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (25, true) => {
                if token_index == 0 {
                    indentation[indentation_depth as usize + 1] = line_index as i8;
                    running = false;
                    tokens = shift_list(tokens, token_index, 1);
                }
            },
            (26, true) | (26, false) => {
                match tokens[token_index + 1].0 {
                    1 => {
                        if tokens[token_index + 1].1 > 0 {
                            tokens[token_index + 1] = (tokens[token_index + 1].0, tokens[token_index + 1].1 - 1);
                            return_to_last_indent = true;
                        } else {
                            indentation[indentation_depth as usize + 1] = -1;
                        }
                        running = true;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            }
            (27, true) | (27, false) => {
                indentation[indentation_depth as usize + 1] = -1;
                running = true;
            },
            (28, true) => {
                match tokens[token_index + 1].0 {
                    3 => {
                        if tokens[token_index + 1].1 == 1 {
                            indentation[indentation_depth as usize + 1] = line_index as i8;
                            running = true;
                        } else {
                            indentation[indentation_depth as usize + 1] = line_index as i8;
                            running = false;
                        }
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (29, true) =>  {
                match (tokens[token_index + 1].0, tokens[token_index + 2].0) {
                    (1, 1) => {
                        window::set_terminal_color(tokens[token_index + 1].1 as u8, tokens[token_index + 2].1 as u8);
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            }
            _ => {}
        }

        token_index += 1;
    }

    (tokens, line_index, return_to_last_indent, running)
}

fn match_token(token: [u8; 64], variables: [[u8; 64]; 64]) -> (u8, i32, [[u8; 64]; 64]) {
    let tokens_val = [
        "say", "print", "+", "-", "/", "*", "(", ")", "==", 
        ">=", "<=", ">", "<", "true", "false", "not", "yell", 
        "warn", "\n", "lnnew", "=", "do", "repeat", "end", "if", "color"];
    let tokens_keys  = [
         10,    10,      11,  12,  13,  14,  15,  16,  17,   
         20,   21,   18,  19,  3,      3,       22,    23,
         23,     8,    8,       24,  25,   26,       27,    28,   29];

    for command_index in 0..tokens_val.len() {
        let command = tokens_val[command_index];
        let command_bytes = command.bytes();
        let mut is_command = true;

        let mut i = 0;
        for byte in command_bytes {
            if byte != token[i] as u8 {
                is_command = false;
            }
            i += 1;
        }
        if !is_command { continue; }

        if tokens_val[command_index] == "true" { return (tokens_keys[command_index], 1, variables) }
        return (tokens_keys[command_index], 0, variables)
    }

    let mut is_int = true;
    let mut is_float = false;
    let mut int_len = 0;
    for byte in token {
        if byte == 0 { break; }
        if (byte < 48 || byte > 57) && byte != 46 {
            is_int = false;
            break;
        }
        if byte == 46 {
            is_int = false;
            is_float = true;
        }
        int_len += 1;
    }
    if is_int {
        let mut int_val = 0;

        for i in 0..int_len {
            let byte_number = token[i] as i32 - 48;
            int_val += byte_number * 10_i32.pow((int_len - i) as u32 - 1);
        }

        return (1, int_val, variables)
    } else if is_float {
        let mut int_val = 0;
        let mut dec_place = 0;

        for i in 0..int_len {
            if token[i] == 46 {
                dec_place = i;
                break;
            }
        }

        let decimals = (int_len - dec_place) as u32 - 1;

        for i in 0..int_len {
            if token[i] == 46 {
                continue;
            }
            if i > 2 && token[i - 3] == 46 { break; }
            let byte_number = token[i] as i32 - 48;
            if i < dec_place { int_val += byte_number * 10_i32.pow((int_len - i) as u32 - decimals); }
            else { int_val += byte_number * 10_i32.pow((int_len - i) as u32 - (decimals - 1)); }
        }

        return (2, int_val, variables)
    }
    
    let mut variables_new = variables;
    for variable in variables.iter().enumerate() {
        if variable.1 == &token {
            return (7, variable.0 as i32, variables);
        } else if variable.1 == &[0; 64] {
            variables_new[variable.0] = token;
            return (7, variable.0 as i32, variables_new);
        }
    }
    (7, 63, variables)
}

fn tokenize(input: BigString) -> [[(u8, i32); 255]; 32] {
    let mut lines: [[(u8, i32); 255]; 32] = [[(0, 0); 255]; 32];
    let mut tokens_index = 0;
    let mut line = 0;

    // this creates a max token length of 64
    let mut temp_token = [0; 64];
    let mut temp_token_index = 0;

    let mut is_comment = false;

    let mut variables = [[0; 64]; 64];

    for char_index in 0..input.len() {
        let char = input.get(char_index);
        if char == 0 { continue; }
        if char == b'#' as usize {
            is_comment = true;
            continue;
        }
        if char == 32 {
            let token = match_token(temp_token, variables);
            variables = token.2;
            if token.0 == 8 {
                is_comment = false;
                line += 1;
                tokens_index = 0;
                temp_token = [0; 64];
                temp_token_index = 0;
            } else {
                if !is_comment {
                    lines[line][tokens_index] = (token.0, token.1);
                }
                tokens_index += 1;
                temp_token = [0; 64];
                temp_token_index = 0;
            }
        } else {
            temp_token[temp_token_index] = char as u8;
            temp_token_index += 1;
        }
    }
    let token = match_token(temp_token, variables);
    if token.0 != 8 && !is_comment {
        lines[line][tokens_index] = (token.0, token.1);
    }

    lines
}