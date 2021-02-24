use std::fs;
use std::io::prelude::*;

fn main() {
    let input = fs::read_to_string("input").unwrap();
    fs::write("logs", "").unwrap();
    let mut logs_file = fs::OpenOptions::new().append(true).open("logs").unwrap();

    let mut log_buffer = String::new();

    let mut current_passport_fields = 0; // Has to be at least 7 if missing cid
    let mut valid_passports = 0;
    let mut cat_buff = String::new();
    let mut is_reading_category = true;
    let mut has_cid = false;
    for line in input.lines() {
        log_buffer.push_str(line);
        for char in line.chars() {
            if is_reading_category {
                cat_buff.push(char);
                if cat_buff.len() == 3 {
                    current_passport_fields += 1;
                    if cat_buff == "cid" {
                        has_cid = true;
                    }
                    is_reading_category = false;
                    cat_buff = String::new();
                }
            }
            if char == ' ' {
                is_reading_category = true;
            }
        }
        if line == "" {
            if current_passport_fields > 7 {
                writeln!(logs_file, "Was valid: {}", log_buffer).unwrap();
                valid_passports += 1;
            } else if current_passport_fields == 7 {
                if !has_cid {
                    writeln!(logs_file, "Was valid: {}", log_buffer).unwrap();
                    valid_passports += 1;
                } else {
                    writeln!(logs_file, "Was invalid: {}", log_buffer).unwrap();
                }
            } else {
                writeln!(logs_file, "Was invalid: {}", log_buffer).unwrap();
            }
            current_passport_fields = 0;
            has_cid = false;
            log_buffer = String::new();
        }
        is_reading_category = true;
        log_buffer.push('\n');
    }

    if current_passport_fields > 7 {
        valid_passports += 1;
    } else if current_passport_fields == 7 {
        if !has_cid {
            valid_passports += 1;
        }
    }

    println!("{} valid passports", valid_passports);
}
