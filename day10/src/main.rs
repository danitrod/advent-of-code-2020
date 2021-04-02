use std::fmt::Write;
use std::fs;
use std::io::Error;

// Order an array of adapters and find the differences of 1 and differences of 3 in the sequence

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day10/input")?;
    let mut debug = String::new();

    let mut values = Vec::<usize>::new();
    for line in input.lines() {
        values.push(line.parse().expect("invalid line value"));
    }

    values.sort();
    writeln!(debug, "Sorted values: {:?}", values).unwrap();

    let mut one_diffs = 1; // Starts with a difference from 0-joltage to 1
    let mut three_diffs = 1; // Starts with a difference from highest adapter to 3 jolt higher device
    let mut values_iter = values.iter();
    let mut last = values_iter.next().unwrap();
    for value in values_iter {
        if *value == last + 1 {
            one_diffs += 1;
            writeln!(debug, "1-diff from {} to {}", last, value).unwrap();
        } else if *value == last + 3 {
            three_diffs += 1;
            writeln!(debug, "3-diff from {} to {}", last, value).unwrap();
        } else if *value > last + 3 {
            println!("Invalid sequence: {} to {}", last, value);
        }
        last = value;
    }

    println!(
        "{} 1-diffs and {} 3-diffs. Result: {}",
        one_diffs,
        three_diffs,
        one_diffs * three_diffs
    );

    fs::write("day10/debug", debug)?;

    Ok(())
}
