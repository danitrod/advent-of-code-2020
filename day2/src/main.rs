use std::fs;
use std::io::Error;

// Part one
// fn main() -> Result<(), Error> {
//     let contents = fs::read_to_string("input")?;

//     let mut valid_pws = 0;

//     for line in contents.lines() {
//         // Example line:
//         // 15-16 f: ffffffffffffffhf
//         let contents: Vec<&str> = line.split(" ").collect();
//         let constraints: Vec<&str> = contents[0].split("-").collect();
//         let letter: char = contents[1].chars().next().unwrap();
//         let (min, max): (usize, usize) = (
//             constraints[0].parse().unwrap(),
//             constraints[1].parse().unwrap(),
//         );
//         let pass = contents[2];

//         let mut count = 0;
//         for l in pass.chars() {
//             if l == letter {
//                 count += 1;
//             }
//         }
//         if count >= min && count <= max {
//             valid_pws += 1;
//         }
//     }
//     println!("Total valid pws: {}", valid_pws);
//     Ok(())
// }

// Part two
fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("input")?;

    let mut valid_pws = 0;

    for line in contents.lines() {
        // Example line:
        // 15-16 f: ffffffffffffffhf
        let contents: Vec<&str> = line.split(" ").collect();
        let constraints: Vec<&str> = contents[0].split("-").collect();
        let letter: char = contents[1].chars().next().unwrap();
        let (min, max): (usize, usize) = (
            constraints[0].parse().unwrap(),
            constraints[1].parse().unwrap(),
        );
        let pass = contents[2];

        let mut valid = false;
        for l in pass.chars().enumerate() {
            if l.0 == min - 1 {
                if l.1 == letter {
                    valid = true;
                }
            } else if l.0 == max - 1 {
                if l.1 == letter {
                    if valid {
                        valid = false;
                        break;
                    } else {
                        valid = true;
                        break;
                    }
                }
            }
        }
        if valid {
            valid_pws += 1;
        }
    }
    println!("Total valid pws: {}", valid_pws);
    Ok(())
}
