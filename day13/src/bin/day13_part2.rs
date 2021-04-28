use std::fmt::Write;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("day13/input")?;
    let mut debug = String::new();

    let mut lines = input.lines();
    lines.next().unwrap();
    let bus_list: Vec<&str> = lines.next().unwrap().split(",").collect();

    writeln!(debug, "bus list is {:?}", bus_list).unwrap();

    // Brute force solution
    // let mut timestamp: u64 = 100000000000182;

    // loop {
    //     let mut timestamp_local = timestamp - 23;
    //     let mut did_complete_cycle = true;

    //     for (i, bus) in bus_list.clone().iter().enumerate() {
    //         if *bus != "x" {
    //             let bus: u64 = bus.parse().unwrap();
    //             if timestamp_local % bus == 0 {
    //                 if i > 23 {
    //                     writeln!(debug, "bus {} leaves on {}", bus, timestamp_local).unwrap();
    //                     println!("going far! bus {} leaving at {}", bus, timestamp_local);
    //                 }
    //             } else {
    //                 did_complete_cycle = false;
    //                 break;
    //             }
    //         }
    //         timestamp_local += 1;
    //     }

    //     if did_complete_cycle {
    //         println!("Found earliest timestamp: {}", timestamp);
    //         break;
    //     }

    //     timestamp += 479;
    // }

    // Chinese Remainder Theorem
    let mut timestamp: u64 = bus_list[0].parse().unwrap();
    let mut least_common_multiple = 23;
    for i in 1..bus_list.len() {
        if bus_list[i] == "x" {
            continue;
        }
        let bus: u64 = bus_list[i].parse().unwrap();
        while (timestamp + i as u64) % bus != 0 {
            timestamp += least_common_multiple;
        }
        least_common_multiple *= bus; // All inputs are prime so the LCM is the multiplication of the two
    }

    println!("Earliest timestamp: {}", timestamp);

    fs::write("day13/debug", debug)?;

    Ok(())
}
