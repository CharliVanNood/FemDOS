use crate::string::BigString;
use crate::vec::{TokenVec, Vec};

pub fn exec(input: BigString) {
    let mut input_string = input;
    for _ in 0..32 {
        input_string.replace("\n", " lnnew ");
    }
    let (tokenized_code, _lists) = tokenize(input_string);
    for line in tokenized_code {
        line.print();
    }
    //run_tokens(tokenized_code, lists);
}

fn match_token(token: [u8; 256], variables: [Vec; 64]) -> (usize, usize, [Vec; 64]) {
    let tokens_val = [
        "\n", "lnnew", "+", "-", "/", "*",
        "section", ".data", ".text", "global", "_start:", "_start",
        "mov", "int", "xor", "db",
        "eax", "ebx", "ecx", "edx", "0x80"];
    let tokens_keys  = [
         8,    8,       9,   10,  11,  12,
         16,        17,      18,      19,       20,        21,
         32,    33,    34,    35,
         128,   129,   130,   131,   132];

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
        if variable.1.get_as_b256() == token {
            if token == [0; 256] {
                return (9, 0, variables);
            }
            return (7, variable.0, variables);
        } else if variable.1.get_as_b256() == [0; 256] {
            if token == [0; 256] {
                return (9, 0, variables);
            }
            variables_new[variable.0].set_as_b256(token);
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

    let mut temp_token = [0; 256];
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
        if char == ';' as usize {
            is_comment = true;
            continue;
        }

        if char == 0 { continue; }
        if char == 32 {
            let token = match_token(temp_token, variables);

            variables = token.2;
            if token.0 == 9 {
                temp_token = [0; 256];
                temp_token_index = 0;
                continue;
            }
            if token.0 == 16 {
                is_string = !is_string;
                if !is_string && !is_comment {
                    lines[line].add(5, lists_len - 1);
                }
                temp_token = [0; 256];
                temp_token_index = 0;
            } else if token.0 == 8 {
                is_comment = false;
                line += 1;
                temp_token = [0; 256];
                temp_token_index = 0;
            } else if !is_string {
                if !is_comment { lines[line].add(token.0, token.1) }
                temp_token = [0; 256];
                temp_token_index = 0;
            } else if is_string {
                if !is_comment { 
                    for character in temp_token {
                        if character == 0 { continue; }
                        lists[lists_len].add(6, character as usize);
                    }
                    lists_len += 1;
                }
                temp_token = [0; 256];
                temp_token_index = 0;
            }
        } else {
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