use std::collections::VecDeque;
use std::fmt::Write;
use std::fs;
use std::io::Error;

// Preamble of 25 numbers
// Each next number is a sum of two of the immediate 25 previous numbers
// Find the first number which is not a sum of two of the immediate 25 previous numbers

const PREAMBLE_SIZE: usize = 25;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day9/input")?;
    let mut debug = String::new();

    // Use a queue to store the most recent PREAMBLE_SIZE numbers
    let mut prev_values = VecDeque::<usize>::with_capacity(PREAMBLE_SIZE);

    let mut line_index = 0;
    for line in input.lines() {
        let number: usize = line.parse().expect("line int value");
        if !(line_index < PREAMBLE_SIZE) {
            // Check if current number is a sum of any two of prev_values
            let mut found_valid_sum = false;
            for i in 0..PREAMBLE_SIZE {
                let ith = prev_values
                    .get(i)
                    .expect("queue shorter than preamble size");
                for j in i + 1..PREAMBLE_SIZE {
                    let jth = prev_values
                        .get(j)
                        .expect("queue shorter than preamble size");
                    if jth + ith == number {
                        writeln!(debug, "{}: sum of {} and {}", number, ith, jth).unwrap();
                        found_valid_sum = true;
                        break;
                    }
                    if found_valid_sum {
                        break;
                    }
                }
            }
            if !found_valid_sum {
                println!("Found first invalid number: {}", number);
                break;
            }

            prev_values.pop_front();
        }
        prev_values.push_back(number);
        line_index += 1;
    }

    fs::write("day9/debug", debug)?;

    Ok(())
}
