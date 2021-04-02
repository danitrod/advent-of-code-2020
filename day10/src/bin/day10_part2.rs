use std::fmt::Write;
use std::fs;
use std::io::Error;

// Find out how many different ways the adapters can be arranged, respecting the rules

// Hypothesis: for a sequence of n 1-difference adapters, there are (n * (n - 1) / 2) + 1 possible arrangements.

// 3 - 4
// [1, 4, 5, 6, 7, 10, 11, 14]
// [1, 4, 5, 7, 10, 11, 14]
// [1, 4, 7, 10, 11, 14]
// [1, 4, 6, 7, 10, 11, 14]

// 4 - 7
// [1, 4, 5, 6, 7, 8, 11, 12, 15]
// [1, 4, 6, 7, 8, 11, 12, 15]
// [1, 4, 7, 8, 11, 12, 15]
// [1, 4, 6, 8, 11, 12, 15]
// [1, 4, 5, 7, 8, 11, 12, 15]
// [1, 4, 5, 8, 11, 12, 15]
// [1, 4, 5, 6, 8, 11, 12, 15]

// 5 - 11
// [1, 4, 5, 6, 7, 8, 9, 12, 13, 16]
// [1, 4, 6, 7, 8, 9, 12, 13, 16]
// [1, 4, 7, 8, 9, 12, 13, 16]
// [1, 4, 7, 9, 12, 13, 16]
// [1, 4, 6, 7, 9, 12, 13, 16]
// [1, 4, 6, 9, 12, 13, 16]
// [1, 4, 6, 8, 9, 12, 13, 16]
// [1, 4, 5, 7, 8, 9, 12, 13, 16]
// [1, 4, 5, 8, 9, 12, 13, 16]
// [1, 4, 5, 7, 9, 12, 13, 16]
// [1, 4, 5, 6, 7, 9, 12, 13, 16]

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day10/input")?;
    let mut debug = String::new();

    let mut values = Vec::<usize>::new();
    for line in input.lines() {
        values.push(line.parse().expect("invalid line value"));
    }

    values.sort_unstable();
    writeln!(debug, "Sorted values: {:?}", values).unwrap();

    // Add last joltage
    values.push(values[values.len() - 1] + 3);

    let mut sequence_arrangements = Vec::new();
    let mut last_value = 0;
    let mut sequence_length = 0;
    for value in values.iter() {
        if value - last_value == 1 {
            sequence_length += 1;
        } else {
            writeln!(
                debug,
                "Sequence of {} ending on {}: {} possible arrangements",
                sequence_length,
                value,
                (sequence_length * (sequence_length - 1) / 2) + 1
            )
            .unwrap();
            if sequence_length > 1 {
                sequence_arrangements.push((sequence_length * (sequence_length - 1) / 2) + 1);
            }
            sequence_length = 0;
        }
        last_value = *value;
    }

    println!(
        "Sequence possible arrangements: {:?}",
        sequence_arrangements
    );
    println!(
        "Total possible arrangements: {}",
        sequence_arrangements.iter().product::<i64>()
    );

    fs::write("day10/debug", debug)?;

    Ok(())
}
