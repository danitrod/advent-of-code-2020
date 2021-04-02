use std::fmt::Write;
use std::fs;
use std::io::Error;

// Find the first sequence of numbers which add up to 400480901
const WANTED_NUMBER: usize = 400480901;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day9/input")?;
    let mut debug = String::new();

    // Naive n^2 method: try the sum starting from each possible value until it gets too big to fit

    // Store the lines in a vec
    let mut values = Vec::<usize>::new();
    for line in input.lines() {
        values.push(line.parse().expect("int value"));
    }

    // Do the mapping
    let mut sum;
    let mut found = false;
    for i in 0..values.len() {
        sum = values[i];
        for j in i + 1..values.len() {
            sum += values[j];
            if sum == WANTED_NUMBER {
                let sequence = values.get(i..=j).unwrap();
                let min = sequence.iter().min().unwrap();
                let max = sequence.iter().max().unwrap();
                println!(
                    "Found wanted sum! From {} to {}. Sum of smallest ({}) and largest ({}) num in sequence is {}.",
                    values[i],
                    values[j],
                    min,
                    max,
                    min + max
                );
                found = true;
            } else if sum > WANTED_NUMBER {
                writeln!(debug, "Exceeded sum from {} to {}", values[i], values[j]).unwrap();
                break;
            }
        }
        if found {
            break;
        }
    }

    fs::write("day9/debug", debug)?;

    Ok(())
}
