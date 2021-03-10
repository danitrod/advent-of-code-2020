use std::fs;
use std::io::Error;

fn parse_signed_int(val: &str) -> isize {
    let mut chars = val.chars();
    let sign = chars.next().expect("value sign");
    let mut value = String::new();

    for ch in chars {
        value.push(ch);
    }

    let value = value.parse::<isize>().expect("invalid value");

    match sign {
        '+' => value,
        '-' => value * -1,
        _ => panic!("invalid sign"),
    }
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day8/input")?;
    let mut debug = String::new();
    let mut accumulator = 0;

    /* Instruction data structure: vec with one string for each line,
    and a bool for if the line has been read or not */
    let mut instructions: Vec<(&str, bool)> = Vec::new();
    for line in input.lines() {
        instructions.push((line, false));
    }

    let mut current_line: isize = 0;
    loop {
        // Validate line number
        if current_line < 0 {
            panic!("negative line read");
        }
        let line_index = current_line as usize;

        if instructions[line_index].1 {
            // Found loop
            debug.push_str(&format!("found loop at line {}\n", current_line));
            break;
        }
        instructions[line_index].1 = true;

        // Read and apply instruction
        let instruction: Vec<&str> = instructions[line_index].0.split(' ').collect();
        let (instruction, val) = (instruction[0], instruction[1]);
        match instruction {
            "acc" => {
                debug.push_str(&format!("accumulating {}\n", val));
                accumulator += parse_signed_int(val);
                current_line += 1;
            }
            "jmp" => {
                debug.push_str(&format!("jumping {}\n", val));
                current_line += parse_signed_int(val);
            }
            "nop" => {
                debug.push_str("noop\n");
                current_line += 1;
            }
            _ => panic!("Unexpected instruction received"),
        }
        debug.push_str(&format!("passing thru line {}\n", current_line));
    }

    println!("Accumulator value: {}", accumulator);

    fs::write("day8/debug", debug)?;

    Ok(())
}
