use std::collections::HashMap;
use std::fs;
use std::io::Error;

// bagmap: <bag_name> -> (number, contained_bag_name)[] or none
type BagMap = HashMap<String, Option<Vec<(usize, String)>>>;

// Auxiliary function to find if a bag can hold a 'shiny gold' bag
fn can_hold_shiny(name: &str, bags_map: &BagMap) -> bool {
    let val = bags_map
        .get(name)
        .expect(&format!("Trying to get invalid bag {}", name));

    if let Some(contained_bags) = val {
        for bag in contained_bags {
            if &bag.1 == "shiny gold" {
                return true;
            } else if can_hold_shiny(&bag.1, bags_map) {
                return true;
            }
        }
    }
    false
}

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input")?;
    let mut debug = String::new();
    let mut count = 0;
    let mut bags_map = BagMap::new();

    // Populate the bags_map
    for line in input.lines() {
        let mut bag_name = String::new();
        let mut contained_bag_name = String::new();
        let mut contained_bag_number = 0;
        let mut reading_bag_name = true;
        let mut contained_bags = Vec::<(usize, String)>::new();
        let mut unit_bag = false; // If the bag can contain no other

        for word in line.split_ascii_whitespace() {
            if word == "bags" {
                continue;
            }
            if word == "contain" {
                reading_bag_name = false;
                continue;
            }
            if reading_bag_name {
                bag_name.push_str(&format!("{} ", word));
            } else {
                if word == "bags," || word == "bag," || word == "bags." || word == "bag." {
                    contained_bags.push((
                        contained_bag_number,
                        contained_bag_name.trim_end().to_owned(),
                    ));
                    contained_bag_name = String::new();
                    contained_bag_number = 0;
                    continue;
                }
                if word == "no" {
                    unit_bag = true;
                    break;
                }
                if let Some(val) = word.parse::<usize>().ok() {
                    contained_bag_number = val;
                } else {
                    contained_bag_name.push_str(&format!("{} ", word));
                }
            }
        }
        debug.push_str(&format!(
            "Inserting {}: {:?}\n",
            bag_name.trim_end(),
            contained_bags
        ));
        bags_map.insert(
            bag_name.trim_end().to_owned(),
            if unit_bag { None } else { Some(contained_bags) },
        );
    }

    // Find out how many bags can contain 'shiny gold'
    for (key, _) in bags_map.iter() {
        if can_hold_shiny(key, &bags_map) {
            debug.push_str(&format!("{} can hold shiny\n", key));
            count += 1;
        }
    }
    debug.push_str(&format!("Count: {}\n", count));

    println!("Total: {}", count);

    fs::write("debug", debug)?;

    Ok(())
}
