use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?;
    let mut debug = String::new();
    let mut highest_seat_id = 0;

    for line in input.lines() {
        let (mut start, mut end, mut mid) = (0, 127, 63);
        debug.push_str(line);
        let mut instructions = line.chars();

        for _ in 0..7 {
            let next_instruction = instructions.next().expect("Early row end");
            match next_instruction {
                'F' => {
                    end = mid;
                    mid = (start + end) / 2;
                }
                'B' => {
                    start = mid + 1;
                    mid = (start + end) / 2;
                }
                _ => panic!("Unexpected row value {}", next_instruction),
            }
        }

        if start != end {
            panic!("Unexpected row {}-{}", start, end);
        }

        let row = start;

        let (mut start, mut end, mut mid) = (0, 7, 3);
        for _ in 0..3 {
            let next_instruction = instructions.next().expect("Early col end");
            match next_instruction {
                'L' => {
                    end = mid;
                    mid = (start + end) / 2;
                }
                'R' => {
                    start = mid + 1;
                    mid = (start + end) / 2;
                }
                _ => panic!("Unexpected col value {}", next_instruction),
            }
        }

        if start != end {
            panic!("Unexpected col {}-{}", start, end);
        }
        let id = row * 8 + start;

        debug.push_str(&format!(" - row {} col {}\n", row, start));

        if id > highest_seat_id {
            highest_seat_id = id
        };
    }

    println!("Highest seat id: {}", highest_seat_id);

    fs::write("debug", debug)?;

    Ok(())
}
