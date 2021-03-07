use std::collections::HashMap;
use std::fs;
use std::io::Error;

// bagmap: <bag_name> -> (number, contained_bag_name)[] or none
type BagMap = HashMap<String, Option<Vec<(usize, String)>>>;

// Find out how many bags a bag holds
fn held_quantity(name: &str, map: &BagMap, debug: &mut String) -> usize {
  let mut count = 0;
  let val = map.get(name).expect(&format!("bag {} not found", name));
  if let Some(bags) = val {
    for bag in bags {
      count += bag.0;
      count += bag.0 * held_quantity(&bag.1, map, debug);
    }
  }
  debug.push_str(&format!("{} holds {} bags\n", name, count));
  count
}

fn main() -> Result<(), Error> {
  let input = fs::read_to_string("input")?;
  let mut debug = String::new();
  let count;
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

  // Find out how many bags 'shiny gold' holds
  count = held_quantity("shiny gold", &bags_map, &mut debug);

  debug.push_str(&format!("Count: {}\n", count));

  println!("Total: {}", count);

  fs::write("debug", debug)?;

  Ok(())
}
