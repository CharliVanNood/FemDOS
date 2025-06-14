use crate::{clock, input, print, println, string::BigString, vec::{TokenVec, Vec}, warnln, window};

pub fn exec(input: [u8; 512]) {
    let mut input_string = BigString::from_b512(input);
    for _ in 0..32 {
        input_string.replace("\n", " lnnew ");
        input_string.replace(";", " lnnew ");
        input_string.replace("\"", " lnlist ");
    }
    let (tokenized_code, lists) = tokenize(&input_string);
    input_string.remove();
    run_tokens(tokenized_code, lists);
}

fn match_token(token: [u8; 64], variables: [Vec; 64]) -> (usize, usize, [Vec; 64]) {
    let tokens_val = [
        "PRINT", "\n", "lnnew", "TRUE", "FALSE", "+", "-", "/", "*", "INPUT", "lnlist", "==", "NOT", "=",
        "DO", "LOOP", "SLEEP", "QUIT", "PIXEL", "RECT", "IF", "END"];
    let tokens_keys  = [
         10,      8,    8,       3,      3,       11,  12,  13,  14,  15,      16,       18,   19,    20,
         25,   26,     27,      28,     29,      30,     31,   32];

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

        if tokens_val[command_index] == "TRUE" { return (tokens_keys[command_index], 1, variables) }
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
    if is_int && int_len > 0 {
        let mut int_val = 0;

        for i in 0..int_len {
            let byte_number = token[i] as i32 - 48;
            int_val += byte_number * 10_i32.pow((int_len - i) as u32 - 1);
        }

        return (1, int_val as usize, variables)
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

        return (2, int_val as usize, variables)
    }

    let mut variables_new = variables;
    for variable in variables.iter().enumerate() {
        if variable.1.get_as_b64() == token {
            if token == [0; 64] {
                return (17, 0, variables);
            }
            return (7, variable.0, variables);
        } else if variable.1.get_as_b64() == [0; 64] {
            if token == [0; 64] {
                return (17, 0, variables);
            }
            variables_new[variable.0].set_as_b64(token);
            return (7, variable.0, variables_new);
        }
    }
    (7, 63, variables)
}

fn tokenize(input: &BigString) -> ([TokenVec; 128], [TokenVec; 64]) {
    let mut lines: [TokenVec; 128] = [TokenVec::new(); 128];
    for i in 1..128 {
        lines[i] = TokenVec::new();
    }

    let mut line = 0;

    let mut temp_token = [0; 64];
    let mut temp_token_index = 0;

    let mut is_string = false;
    let mut is_comment = false;

    let mut lists = [TokenVec::new(); 64];
    let mut lists_len = 0;
    for i in 1..64 {
        lists[i] = TokenVec::new();
    }

    let mut variables = [Vec::new(); 64];
    for i in 1..64 {
        variables[i] = Vec::new();
    }

    for char_index in 0..input.len() {
        let char = input.get(char_index);
        if char == '\'' as usize {
            is_comment = true;
            continue;
        }
        if char == 0 { continue; }
        if char == 32 {
            let token = match_token(temp_token, variables);
            variables = token.2;
            if token.0 == 17 { continue; }
            if token.0 == 16 {
                is_string = !is_string;
                if !is_string {
                    lines[line].add(5, lists_len);
                    lists_len += 1;
                }
                temp_token = [0; 64];
                temp_token_index = 0;
            } else if token.0 == 8 {
                is_comment = false;
                line += 1;
                temp_token = [0; 64];
                temp_token_index = 0;
            } else if !is_string {
                if !is_comment { lines[line].add(token.0, token.1) }
                temp_token = [0; 64];
                temp_token_index = 0;
            } else if is_string {
                if !is_comment {
                    for character in temp_token {
                        if character == 0 { continue; }
                        lists[lists_len].add(6, character as usize);
                    }
                    lists[lists_len].add(6, b' ' as usize);
                }
                temp_token = [0; 64];
                temp_token_index = 0;
            }
        } else {
            //println!("New token {} with {}", temp_token_index, char);
            temp_token[temp_token_index] = char as u8;
            temp_token_index += 1;
        }
    }
    if !is_comment {
        let token = match_token(temp_token, variables);
        if token.0 != 8 {
            lines[line].add(token.0, token.1);
        }
    }

    for variable in variables {
        variable.remove();
    }

    (lines, lists)
}

fn run_tokens(mut tokens: [TokenVec; 128], mut lists: [TokenVec; 64]) {
    let mut variables: Vec = Vec::new();
    let mut indentation: Vec = Vec::new();
    let mut running = true;
    //let mut original_tokens = tokens;

    let mut line_index = 0;
    while line_index < tokens.len() {
        let line = tokens[line_index];
        if line.len() == 0 {
            line_index += 1;
            continue;
        }

        let mut indentation_depth: u8 = 0;
        for indentation_layer in indentation.get_as_b64() {
            if indentation_layer > 0 {
                indentation_depth += 1;
            }
        }

        let line_running = line.copy();
        let operation_result = run_line(line_running, &mut indentation, line_index, &mut variables, &mut lists, indentation_depth, running);

        line_index = operation_result.1;
        running = operation_result.3;
        if operation_result.2 {
            tokens[line_index].remove();
            tokens[line_index] = operation_result.0.copy();
            line_index = indentation.get(indentation_depth as usize - 1) as usize;
        }

        if operation_result.4 {
            line_index = tokens.len();
        }

        line_running.remove();

        line_index += 1;
    }
}

fn run_line(line: TokenVec, mut indentation: &mut Vec, line_index: usize, mut variables: &mut Vec, mut lists: &mut [TokenVec; 64], indentation_depth: u8, running: bool) -> (TokenVec, usize, bool, bool, bool) {
    if running {
        let tokens_after_fact = run_tokens_fact(line, *variables, *lists, *indentation, indentation_depth);
        let tokens_after_math = run_tokens_math(tokens_after_fact, *variables, *lists, *indentation, indentation_depth);
        let tokens_after_first = run_tokens_first(tokens_after_math, *variables, *lists, *indentation, indentation_depth);
        let tokens_after_bool = run_tokens_boolean(tokens_after_first, *variables, *lists, *indentation, indentation_depth);
        let operation_result = run_tokens_last(tokens_after_bool, &mut variables, &mut lists, &mut indentation, indentation_depth, line_index, running);
        return operation_result;
    } else {
        let operation_result = run_tokens_last(line, variables, &mut lists, indentation, indentation_depth, line_index, running);
        return operation_result;
    }
}

fn run_tokens_fact(mut tokens: TokenVec, variables: Vec, _lists: [TokenVec; 64], _indentation: Vec, _indentation_depth: u8) -> TokenVec {
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match token.0 {
            13 => {
                match (tokens.get(token_index - 1).0, tokens.get(token_index + 1).0) {
                    (1, 1) => {
                        let operation_result: f32 = tokens.get(token_index - 1).1 as f32 / tokens.get(token_index + 1).1 as f32 * 100.0;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (7, 7) => {
                        let operation_result: f32 = variables.get(tokens.get(token_index - 1).1) as f32 / variables.get(tokens.get(token_index + 1).1) as f32 * 100.0;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            14 => {
                match (tokens.get(token_index - 1).0, tokens.get(token_index + 1).0) {
                    (1, 1) => {
                        let operation_result = tokens.get(token_index - 1).1 * tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (7, 7) => {
                        let operation_result = variables.get(tokens.get(token_index - 1).1) * variables.get(tokens.get(token_index + 1).1);
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
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

fn run_tokens_math(mut tokens: TokenVec, variables: Vec, _lists: [TokenVec; 64], _indentation: Vec, _indentation_depth: u8) -> TokenVec {
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match token.0 {
            11 => {
                match (tokens.get(token_index - 1).0, tokens.get(token_index + 1).0) {
                    (1, 1) => {
                        let operation_result = tokens.get(token_index - 1).1 + tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let operation_result = tokens.get(token_index - 1).1 + tokens.get(token_index + 1).1 * 100;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let operation_result = tokens.get(token_index - 1).1 * 100 + tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let operation_result = tokens.get(token_index - 1).1 + tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (7, 7) => {
                        let operation_result = variables.get(tokens.get(token_index - 1).1) + variables.get(tokens.get(token_index + 1).1);
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (7, 1) => {
                        let operation_result = variables.get(tokens.get(token_index - 1).1) + tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (1, 7) => {
                        let operation_result = tokens.get(token_index - 1).1 + variables.get(tokens.get(token_index + 1).1);
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            12 => {
                match (tokens.get(token_index - 1).0, tokens.get(token_index + 1).0) {
                    (1, 1) => {
                        let operation_result = tokens.get(token_index - 1).1 - tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    },
                    (2, 1) => {
                        let operation_result = tokens.get(token_index - 1).1 - tokens.get(token_index + 1).1 * 100;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let operation_result = tokens.get(token_index - 1).1 * 100 - tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let operation_result = tokens.get(token_index - 1).1 - tokens.get(token_index + 1).1;
                        tokens.set(token_index - 1, 2, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (7, 7) => {
                        let operation_result = variables.get(tokens.get(token_index - 1).1) - variables.get(tokens.get(token_index + 1).1);
                        tokens.set(token_index - 1, 1, operation_result as usize);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
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

fn run_tokens_boolean(mut tokens: TokenVec, variables: Vec, _lists: [TokenVec; 64], _indentation: Vec, _indentation_depth: u8) -> TokenVec {
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match token.0 {
            18 => {
                match (tokens.get(token_index - 1).0, tokens.get(token_index + 1).0) {
                    (1, 1) => {
                        let mut operation_result = 0;
                        if tokens.get(token_index - 1).1 == tokens.get(token_index + 1).1 {
                            operation_result = 1;
                        }
                        tokens.set(token_index - 1, 3, operation_result);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (1, 2) => {
                        let mut operation_result = 0;
                        if tokens.get(token_index - 1).1 * 100 == tokens.get(token_index + 1).1 {
                            operation_result = 1;
                        }
                        tokens.set(token_index - 1, 3, operation_result);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (2, 1) => {
                        let mut operation_result = 0;
                        if tokens.get(token_index - 1).1 == tokens.get(token_index + 1).1 * 100 {
                            operation_result = 1;
                        }
                        tokens.set(token_index - 1, 3, operation_result);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (2, 2) => {
                        let mut operation_result = 0;
                        if tokens.get(token_index - 1).1 == tokens.get(token_index + 1).1 {
                            operation_result = 1;
                        }
                        tokens.set(token_index - 1, 3, operation_result);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (3, 3) => {
                        let mut operation_result = 0;
                        if tokens.get(token_index - 1).1 == tokens.get(token_index + 1).1 {
                            operation_result = 1;
                        }
                        tokens.set(token_index - 1, 3, operation_result);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    (7, 7) => {
                        let mut operation_result = 0;
                        if variables.get(tokens.get(token_index - 1).1) == variables.get(tokens.get(token_index + 1).1) {
                            operation_result = 1;
                        }
                        tokens.set(token_index - 1, 3, operation_result);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
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

fn run_tokens_first(mut tokens: TokenVec, _variables: Vec, _lists: [TokenVec; 64], _indentation: Vec, _indentation_depth: u8) -> TokenVec {
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match token.0 {
            19 => {
                if tokens.get(token_index + 1).0 == 3 {
                    if tokens.get(token_index + 1).1 == 0 {
                        tokens.set(token_index, 3, 1);
                    } else {
                        tokens.set(token_index, 3, 0);
                    }
                    tokens.shift(token_index + 1, 1);
                    token_length = tokens.len();
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
    mut tokens: TokenVec, variables: &mut Vec, lists: &mut [TokenVec; 64], indentation: &mut Vec, 
    indentation_depth: u8, line_index: usize, mut running: bool) -> (TokenVec, usize, bool, bool, bool) {
    let mut return_to_last_indent = false;
    let mut stop_program = false;
    
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match (token.0, running) {
            (27, true) => {
                if token_index + 1 < token_length {
                    match tokens.get(token_index + 1).0 {
                        1 => {
                            let end_time = *clock::MILLISECONDS.lock() + tokens.get(token_index + 1).1;
                            while *clock::MILLISECONDS.lock() < end_time {
                                x86_64::instructions::hlt();
                            }

                            tokens.shift(token_index, 2);
                            token_length = tokens.len();
                        },
                        _ => warnln!("This is an unsupported type conversion")
                    }
                } else {
                    let mut ended = false;
                    
                    input::KEYPRESSES.lock().0 = [0; 8];
                    input::KEYPRESSES.lock().1 = 0;

                    while !ended {
                        let keypresses = {
                            let lock = input::KEYPRESSES.lock();
                            lock.clone()
                        };
                    
                        for keypress in keypresses.0 {
                            if keypress == 0 { break; }
                            ended = true;
                        }
                    
                        input::KEYPRESSES.lock().0 = [0; 8];
                        input::KEYPRESSES.lock().1 = 0;

                        x86_64::instructions::hlt();
                    }

                    tokens.shift(token_index, 1);
                    token_length = tokens.len();
                }
            },
            (28, true) => {
                stop_program = true;
            },
            (29, true) => {
                match (
                    tokens.get(token_index + 1).0, 
                    tokens.get(token_index + 2).0, 
                    tokens.get(token_index + 3).0) {
                    (1, 1, 1) => {
                        window::set_pixel(
                            tokens.get(token_index + 1).1, 
                            tokens.get(token_index + 2).1, 
                            tokens.get(token_index + 3).1 as u8
                        );
                    },
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (30, true) => {
                match (
                    tokens.get(token_index + 1).0, 
                    tokens.get(token_index + 2).0, 
                    tokens.get(token_index + 3).0,
                    tokens.get(token_index + 4).0,
                    tokens.get(token_index + 5).0) {
                    (1, 1, 1, 1, 1) => {
                        window::set_rect(
                            tokens.get(token_index + 1).1, 
                            tokens.get(token_index + 2).1, 
                            tokens.get(token_index + 3).1, 
                            tokens.get(token_index + 4).1, 
                            tokens.get(token_index + 5).1 as u8
                        );
                    },
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (31, true) => {
                match tokens.get(token_index + 1).0 {
                    3 => {
                        if tokens.get(token_index + 1).1 == 0 {
                            running = false;
                        }
                    },
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (32, false) => {
                running = true;
            },
            (26, true) | (26, false) => {
                if token_index + 1 < token_length {
                    match tokens.get(token_index + 1).0 {
                        1 => {
                            if tokens.get(token_index + 1).1 > 0 {
                                tokens.set(token_index + 1, tokens.get(token_index + 1).0, tokens.get(token_index + 1).1 - 1);
                                return_to_last_indent = true;
                            }
                        },
                        _ => warnln!("This is an unsupported type conversion")
                    }
                } else {
                    return_to_last_indent = true;
                }
                running = true;
            },
            (25, true) => {
                indentation.set_add(indentation_depth as usize + 1, line_index);
                running = true;
                tokens.shift(token_index, 1);
            },
            (20, true) => {
                match (tokens.get(token_index - 1).0, tokens.get(token_index + 1).0) {
                    (7, 1) => {
                        variables.set_add(
                            tokens.get(token_index - 1).1 as usize, 
                            tokens.get(token_index + 1).1
                        );
                        tokens.shift(token_index - 1, 3);
                        token_length = tokens.len();
                        token_index -= 1;
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (10, true) => {
                match tokens.get(token_index + 1).0 {
                    1 => {
                        println!("{}", tokens.get(token_index + 1).1);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    2 => {
                        println!("{}", tokens.get(token_index + 1).1 as f32 / 100.0);
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    3 => {
                        if tokens.get(token_index + 1).1 == 0 {
                            println!("FALSE");
                        } else {
                            println!("TRUE");
                        }
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    5 => {
                        let list = lists[tokens.get(token_index + 1).1];
                        for list_index in 0..list.len() {
                            print!("{}", list.get(list_index).1 as u8 as char);
                        }
                        println!("");
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    7 => {
                        println!("{}", variables.get(tokens.get(token_index + 1).1));
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (15, true) => {
                match tokens.get(token_index + 1).0 {
                    5 => {
                        lists[tokens.get(token_index + 1).1].print();
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    _ => print!("INPUT: ")
                }
                
                let mut ended = false;
                let mut text_input = [0; 64];
                let mut text_input_len = 0;

                while !ended {
                    let keypresses = {
                        let lock = input::KEYPRESSES.lock();
                        lock.clone()
                    };
                
                    for keypress in keypresses.0 {
                        if keypress == 0 { break; }
                        if keypress > 255 { continue; }
                        if keypress == 10 && text_input_len > 0 {
                            ended = true;
                            break;
                        } else if keypress == 10 {
                            break;
                        }
                        print!("{}", keypress as u8 as char);
                        text_input[text_input_len] = keypress;
                        text_input_len += 1;
                    }
                
                    input::KEYPRESSES.lock().0 = [0; 8];
                    input::KEYPRESSES.lock().1 = 0;

                    x86_64::instructions::hlt();
                }

                println!("\nreceived {} characters", text_input_len);
            },
            _ => {}
        }

        token_index += 1;
    }

    (tokens, line_index, return_to_last_indent, running, stop_program)
}