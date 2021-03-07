use std::collections::HashMap;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
  let input = fs::read_to_string("day6/input")?;
  let mut debug = String::new();
  let mut count = 0;
  let mut answerees_count = 0;
  let mut questions_answered = HashMap::<char, (bool, usize)>::new();

  for line in input.lines() {
    if line == "" {
      for answer_count in questions_answered.values() {
        if answer_count.1 == answerees_count {
          count += 1;
        }
      }
      debug.push_str(&format!(
        "Count: {}, {} answerees\n",
        count, answerees_count
      ));
      questions_answered.clear();
      answerees_count = 0;
    } else {
      answerees_count += 1;
      // Reset visited chars
      for val in questions_answered.values_mut() {
        val.0 = false;
      }

      // Count chars
      for chr in line.chars() {
        if let Some(val) = questions_answered.get(&chr) {
          if !val.0 {
            let new_val = val.1 + 1;
            questions_answered.insert(chr, (true, new_val));
          }
        } else {
          questions_answered.insert(chr, (true, 1));
        }
      }
    }
    debug.push_str(line);
    debug.push('\n');
  }

  for answer_count in questions_answered.values() {
    if answer_count.1 == answerees_count {
      count += 1;
    }
  }
  debug.push_str(&format!("Count: {}\n", count));

  println!("Total: {}", count);

  fs::write("debug", debug)?;

  Ok(())
}
