use std::fmt::Write;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day13/input")?;
    let mut debug = String::new();

    // bus: (id, min waiting time)
    let mut buses = Vec::<(usize, usize)>::new();

    let mut lines = input.lines();
    let starting_timestamp: usize = lines.next().unwrap().parse().unwrap();
    let bus_list = lines.next().unwrap();

    for bus in bus_list.split(",") {
        if bus != "x" {
            let bus: usize = bus.parse().unwrap();
            let min_waiting_time = bus - (starting_timestamp % bus);
            buses.push((bus, min_waiting_time));
            writeln!(debug, "Will wait {} for bus id {}", min_waiting_time, bus).unwrap();
        }
    }

    let bus = buses.iter().min_by_key(|x| x.1).unwrap();

    println!(
        "Minimum wait time: {} for bus ID {}. Multiplied it's {}.",
        bus.1,
        bus.0,
        bus.0 * bus.1,
    );

    fs::write("day13/debug", debug)?;

    Ok(())
}
