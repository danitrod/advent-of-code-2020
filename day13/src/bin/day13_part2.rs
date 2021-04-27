use std::fmt::Write;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day13/input")?;
    let mut debug = String::new();

    let mut lines = input.lines();
    lines.next().unwrap();
    let bus_list: Vec<&str> = lines.next().unwrap().split(",").collect();

    println!("Bus: {:?}", bus_list);

    fs::write("day13/debug", debug)?;

    Ok(())
}
