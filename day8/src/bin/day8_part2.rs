use std::fs;
use std::io::Error;

/** Naive solution: keeps running the program over, swapping one instruction
and checking if it ends correctly. If not, try it again swapping jmp to nop or vice-versa
*/

const INSTRUCTION_SWAP_FROM: &str = "jmp";
const INSTRUCTION_SWAP_TO: &str = "nop";

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
  let mut debug = String::new();

  let mut tried_line_swaps = Vec::<usize>::new();
  let mut tentatives = 0;
  let mut accumulator = -1;
  let mut swapped = true;
  let mut did_finish = false;
  loop {
    if !swapped {
      println!("Exiting, all swap tries have failed. Try again swapping nop for jmp");
      break;
    }
    swapped = false;
    tentatives += 1;
    accumulator = 0;
    did_finish = false;
    let input = fs::read_to_string("day8/input")?;

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
      if current_line == instructions.len() as isize - 1 {
        // Reached bottom
        debug.push_str("reached bottom\n");
        did_finish = true;
        break;
      }
      let line_index = current_line as usize;

      // Handle found loop
      if instructions[line_index].1 {
        debug.push_str(&format!("found loop at line {}\n", current_line));
        break;
      }
      instructions[line_index].1 = true;

      // Read and apply instruction
      let instruction: Vec<&str> = instructions[line_index].0.split(' ').collect();
      let (mut instruction, val) = (instruction[0], instruction[1]);

      // Swap instruction if it hasn't been tried yet
      if !swapped && instruction == INSTRUCTION_SWAP_FROM && !tried_line_swaps.contains(&line_index)
      {
        tried_line_swaps.push(line_index);
        instruction = INSTRUCTION_SWAP_TO;
        swapped = true;
      }

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

    if did_finish {
      break;
    }

    debug.push_str(&format!("Tentative {} failed\n", tentatives));
  }

  if did_finish {
    println!("Success! Accumulated value: {}", accumulator);
  }

  fs::write("day8/debug", debug)?;

  Ok(())
}
