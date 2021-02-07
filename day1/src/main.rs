use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let contents = fs::read_to_string("input")?;

    let mut entries: Vec<u16> = Vec::new();
    for line in contents.lines() {
        let entry: u16 = line.parse().unwrap();
        for &ele in &entries {
            if ele + entry < 2020 {
                for &second_ele in &entries {
                    if ele + second_ele + entry == 2020 {
                        println!("Result! {} * {} * {}", ele, second_ele, entry);
                        return Ok(());
                    }
                }
            }
        }
        entries.push(entry);
    }
    Ok(())
}
