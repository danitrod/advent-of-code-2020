use std::fs;
use std::io::Error;

// Part 1

// fn main() -> Result<(), Error> {
//     let map = fs::read_to_string("input").unwrap();
//     let mut hit_count = 0;

//     let mut output = String::new();

//     let mut lines = map.lines();
//     output.push_str(lines.next().unwrap());
//     output.push('\n');

//     for (index, line) in lines.enumerate() {
//         let mut output_line = String::new();
//         for (pos, char) in line.char_indices() {
//             if pos == (index * 3 + 3) % 31 {
//                 if char == '#' {
//                     output_line.push('X');
//                     hit_count += 1;
//                 } else {
//                     output_line.push('O');
//                 }
//             } else {
//                 output_line.push(char);
//             }
//         }
//         output_line.push('\n');
//         output.push_str(&output_line);
//     }

//     fs::write("output", output).unwrap();

//     println!("Total hit count: {}", hit_count);

//     Ok(())
// }

// Part 2

fn count_hits(map: String, slope_r: usize, slope_d: usize, output_path: &str) -> usize {
    let mut hit_count = 0;
    let mut lines = map.lines();
    let mut output = String::new();

    for _ in 0..slope_d {
        output.push_str(lines.next().unwrap());
        output.push('\n');
    }

    let mut index: isize = 0;

    while let Some(line) = lines.next() {
        let mut log_line = String::new();
        for (pos, char) in line.char_indices() {
            if pos == (index as usize * slope_r + slope_r) % 31 {
                if char == '#' {
                    hit_count += 1;
                    log_line.push('X');
                } else {
                    log_line.push('O');
                }
            } else {
                log_line.push(char);
            }
        }
        log_line.push('\n');
        output.push_str(&log_line);
        index += 1;
        for _ in 1..slope_d {
            if let Some(l) = lines.next() {
                output.push_str(l);
                output.push('\n');
            };
        }
    }

    fs::write(output_path, output).unwrap();

    hit_count
}

fn main() -> Result<(), Error> {
    let map = fs::read_to_string("input").unwrap();

    // Right 1, down 1.
    // Right 3, down 1. (This is the slope you already checked.)
    // Right 5, down 1.
    // Right 7, down 1.
    // Right 1, down 2.

    let s1 = count_hits(map.clone(), 1, 1, "logs/output-1");
    let s2 = count_hits(map.clone(), 3, 1, "logs/output-2");
    let s3 = count_hits(map.clone(), 5, 1, "logs/output-3");
    let s4 = count_hits(map.clone(), 7, 1, "logs/output-4");
    let s5 = count_hits(map, 1, 2, "logs/output-5");

    println!("Total hit count: {}", s1 * s2 * s3 * s4 * s5);
    println!("Part 1: {}", s2);

    Ok(())
}
