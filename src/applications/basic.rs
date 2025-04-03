use crate::{input, print, println, string::BigString, vec::{TokenVec, Vec}, warnln};

pub fn exec(input: [u8; 512]) {
    let mut input_string = BigString::from_b512(input);
    for _ in 0..32 {
        input_string.replace("\n", " lnnew ");
        input_string.replace(";", " lnnew ");
        input_string.replace("\"", " lnlist ");
    }
    let (tokenized_code, lists) = tokenize(input_string);
    run_tokens(tokenized_code, lists);
}

fn match_token(token: [u8; 64], variables: [Vec; 64]) -> (usize, usize, [Vec; 64]) {
    let tokens_val = [
        "PRINT", "\n", "lnnew", "TRUE", "FALSE", "+", "-", "/", "*", "INPUT", "lnlist"];
    let tokens_keys  = [
         10,      8,    8,       3,      3,       11,  12,  13,  14,  15,      16];

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

fn tokenize(input: BigString) -> ([TokenVec; 128], [TokenVec; 64]) {
    let mut lines: [TokenVec; 128] = [TokenVec::new(); 128];
    for i in 1..128 {
        lines[i] = TokenVec::new();
    }

    let mut line = 0;

    let mut temp_token = [0; 64];
    let mut temp_token_index = 0;

    let mut is_string = false;
    let mut is_comment = false;

    let mut variables = [Vec::new(); 64];
    for i in 1..64 {
        variables[i] = Vec::new();
    }

    let mut lists = [TokenVec::new(); 64];
    let mut lists_len = 0;
    for i in 1..64 {
        lists[i] = TokenVec::new();
    }

    for char_index in 0..input.len() {
        let char = input.get(char_index);
        if char == '\'' as usize { is_comment = true; }
        if char == 0 { continue; }
        if char == 32 {
            let token = match_token(temp_token, variables);
            variables = token.2;
            if token.0 == 17 { continue; }
            if token.0 == 16 {
                is_string = !is_string;
                if !is_string {
                    lines[line].add(5, lists_len - 1);
                }
                temp_token = [0; 64];
                temp_token_index = 0;
            } else if token.0 == 8 {
                is_comment = false;
                line += 1;
                temp_token = [0; 64];
                temp_token_index = 0;
            } else if !is_comment && !is_string {
                lines[line].add(token.0, token.1);
                temp_token = [0; 64];
                temp_token_index = 0;
            } else if !is_comment && is_string {
                for character in temp_token {
                    if character == 0 { continue; }
                    lists[lists_len].add(6, character as usize);
                }
                lists_len += 1;
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

    (lines, lists)
}

fn run_tokens(mut tokens: [TokenVec; 128], mut lists: [TokenVec; 64]) {
    let mut variables: [u16; 256] = [0; 256];
    let mut indentation: [i8; 16] = [-1; 16];
    let mut running = true;
    //let mut original_tokens = tokens;

    let mut line_index = 0;
    while line_index < tokens.len() {
        let line = tokens[line_index];

        let mut indentation_depth: u8 = 0;
        for indentation_layer in indentation {
            if indentation_layer > -1 {
                indentation_depth += 1;
            }
        }

        let operation_result = run_line(line, &mut indentation, line_index, &mut variables, &mut lists, indentation_depth, running);

        line_index = operation_result.1;
        running = operation_result.3;
        if operation_result.2 {
            tokens[line_index] = operation_result.0;
            line_index = indentation[indentation_depth as usize] as usize;
        }

        line_index += 1;
    }
}

fn run_line(line: TokenVec, mut indentation: &mut [i8; 16], line_index: usize, mut variables: &mut [u16; 256], mut lists: &mut [TokenVec; 64], indentation_depth: u8, running: bool) -> (TokenVec, usize, bool, bool) {
    if running {
        let tokens_after_fact = run_tokens_fact(line, *variables, *lists, *indentation, indentation_depth);
        //let tokens_after_math = run_tokens_math(tokens_after_fact, *variables, *indentation, indentation_depth);
        //let tokens_after_first = run_tokens_first(tokens_after_math, *variables, *indentation, indentation_depth);
        //let tokens_after_bool = run_tokens_boolean(tokens_after_first, *variables, *indentation, indentation_depth);
        let operation_result = run_tokens_last(tokens_after_fact, &mut variables, &mut lists, &mut indentation, indentation_depth, line_index, running);
        return operation_result;
    } else {
        let operation_result = run_tokens_last(line, variables, &mut lists, indentation, indentation_depth, line_index, running);
        return operation_result;
    }
}

fn run_tokens_fact(mut tokens: TokenVec, _variables: [u16; 256], _lists: [TokenVec; 64], _indentation: [i8; 16], _indentation_depth: u8) -> TokenVec {
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match token.0 {
            13 => {
                if tokens.get(token_index - 1).0 == 1 && tokens.get(token_index + 1).0 == 1 {
                    let operation_result: f32 = tokens.get(token_index - 1).1 as f32 / tokens.get(token_index + 1).1 as f32 * 100.0;
                    tokens.set(token_index - 1, 2, operation_result as usize);
                    tokens.shift(token_index, 2);
                    token_length = tokens.len();
                    token_index -= 1;
                }
            },
            14 => {
                if tokens.get(token_index - 1).0 == 1 && tokens.get(token_index + 1).0 == 1 {
                    let operation_result = tokens.get(token_index - 1).1 * tokens.get(token_index + 1).1;
                    tokens.set(token_index - 1, 1, operation_result as usize);
                    tokens.shift(token_index, 2);
                    token_length = tokens.len();
                    token_index -= 1;
                }
            },
            _ => {}
        }

        token_index += 1;
    }

    tokens
}

/*fn run_tokens_math(mut tokens: [(u8, i32); 255], _variables: [u16; 256], _indentation: [i8; 16], _indentation_depth: u8) -> [(u8, i32); 255] {
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
}*/

fn run_tokens_last(
    mut tokens: TokenVec, _variables: &mut [u16; 256], lists: &mut [TokenVec; 64], _indentation: &mut [i8; 16], 
    _indentation_depth: u8, line_index: usize, running: bool) -> (TokenVec, usize, bool, bool) {
    let return_to_last_indent = false;
    
    let mut token_index = 0;
    let mut token_length = tokens.len();

    while token_index < token_length {
        let token = tokens.get(token_index);

        match (token.0, running) {
            (10, true) => {
                tokens.print();
                println!("PRINTING {}", tokens.get(token_index + 1).0);
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
                        println!("string");
                        tokens.shift(token_index, 2);
                        token_length = tokens.len();
                    }
                    _ => warnln!("This is an unsupported type conversion")
                }
            },
            (15, true) => {
                print!("INPUT: ");
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
                        if keypress == 10 && text_input_len > 0 {
                            ended = true;
                            break;
                        } else if keypress == 10 {
                            break;
                        }
                        print!("{}", keypress as char);
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

    (tokens, line_index, return_to_last_indent, running)
}